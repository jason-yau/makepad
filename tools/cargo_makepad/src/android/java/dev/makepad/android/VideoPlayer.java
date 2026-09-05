package dev.makepad.android;

import android.util.Log;

import android.app.Activity;
import java.lang.ref.WeakReference;

import android.graphics.SurfaceTexture;
import android.view.Surface;
import android.os.Handler;
import android.os.HandlerThread;

import java.util.concurrent.atomic.AtomicInteger;
import android.media.MediaPlayer;

import java.io.FileInputStream;
import java.io.File;

import android.net.Uri;
import android.media.PlaybackParams;

import dev.makepad.android.MakepadNative;

public class VideoPlayer {
    public VideoPlayer(Activity activity, long videoId) {
        mActivityReference = new WeakReference<>(activity);
        mVideoId = videoId;
    }

    public void beginPlayback() {
        if (mMediaPlayer != null && !mMediaPlayer.isPlaying()) {
            mMediaPlayer.start();
            // Some devices only accept PlaybackParams after start().
            if (Math.abs(mPlaybackRate - 1.0f) > 0.02f) {
                applyPlaybackRate(mPlaybackRate);
            }
        }
    }

    public void prepareVideoPlayback() {
        try {
            mSurfaceTexture = new SurfaceTexture(mExternalTextureHandle);

            mHandlerThread = new HandlerThread("GLHandlerThread");
            mHandlerThread.start();
            mGlHandler = new Handler(mHandlerThread.getLooper());
            mSurfaceTexture.setOnFrameAvailableListener(new SurfaceTexture.OnFrameAvailableListener() {
                @Override 
                public void onFrameAvailable(SurfaceTexture surfaceTexture) {
                    mAvailableFrames.incrementAndGet();
                }
            }, mGlHandler);

            Surface surface = new Surface(mSurfaceTexture);

            mMediaPlayer = new MediaPlayer();
            mMediaPlayer.setSurface(surface);

            if (mSource instanceof byte[]) {
                byte[] bytes = (byte[]) mSource;
                Log.i("MakepadVideoPlayer", "videoId=" + mVideoId + " source=in-memory bytes=" + bytes.length);
                ByteArrayMediaDataSource dataSource = new ByteArrayMediaDataSource(bytes);
                mMediaPlayer.setDataSource(dataSource);
            } else if (mSource instanceof String) {
                String dataString = (String) mSource;
                Activity activity = mActivityReference.get();
                if (dataString.startsWith("http://") || dataString.startsWith("https://")) {
                    // Source is a network URL
                    Log.i("MakepadVideoPlayer", "videoId=" + mVideoId + " source=network");
                    mMediaPlayer.setDataSource(dataString);
                } else if (dataString.startsWith("content://") || dataString.startsWith("file://")) {
                    if (activity == null) {
                        throw new IllegalStateException("Activity unavailable for URI playback");
                    }
                    Log.i("MakepadVideoPlayer", "videoId=" + mVideoId + " source=uri " + dataString);
                    mMediaPlayer.setDataSource(activity, Uri.parse(dataString));
                } else {
                    // Source is a path on the local filesystem
                    Log.i("MakepadVideoPlayer", "videoId=" + mVideoId + " source=file path=" + dataString);
                    mMediaPlayer.setDataSource(new FileInputStream(new File(dataString)).getFD());
                }
            }
            
            mMediaPlayer.setLooping(mShouldLoop);

            mMediaPlayer.setOnErrorListener(new MediaPlayer.OnErrorListener() {
                @Override
                public boolean onError(MediaPlayer mp, int what, int extra) {
                    String message = "MediaPlayer error what=" + what + " extra=" + extra;
                    Log.e("MakepadVideoPlayer", "videoId=" + mVideoId + " " + message);
                    MakepadNative.onVideoDecodingError(mVideoId, message);
                    return true;
                }
            });

            mMediaPlayer.prepareAsync();
            mMediaPlayer.setOnPreparedListener(new MediaPlayer.OnPreparedListener() {
                @Override
                public void onPrepared(MediaPlayer mp) {
                    Activity activity = mActivityReference.get();
                    if (activity != null) {
                        activity.runOnUiThread(() -> {
                            MakepadNative.onVideoPlaybackPrepared( 
                                mVideoId,
                                mMediaPlayer.getVideoWidth(),
                                mMediaPlayer.getVideoHeight(),
                                mMediaPlayer.getDuration(),
                                VideoPlayer.this);
                        });
                    }
                    
                    mIsPrepared = true;
                    if (mAutoplay) {
                        mp.start();
                    }
                    if (Math.abs(mPlaybackRate - 1.0f) > 0.02f) {
                        applyPlaybackRate(mPlaybackRate);
                    }
                }
            });

            mMediaPlayer.setOnCompletionListener(new MediaPlayer.OnCompletionListener() {
                @Override
                public void onCompletion(MediaPlayer mp) {
                    // Video playback has finished
                    Activity activity = mActivityReference.get();
                    if (activity != null) {
                        activity.runOnUiThread(() -> {
                            MakepadNative.onVideoPlaybackCompleted(mVideoId);
                        });
                    }
                }
            });
        } catch (Exception e) {
            String message = e.getMessage() != null? e.getMessage() : ("Error decoding video: " + e.toString());
            MakepadNative.onVideoDecodingError(mVideoId, message);
        }
    }


    public boolean maybeUpdateTexImage() {
        boolean updated = false;
        if (!mIsDecoding) {
            mIsDecoding = true;
            return false;
        }

        if (mAvailableFrames.get() > 0) {
            mSurfaceTexture.updateTexImage();

            mAvailableFrames.decrementAndGet();
            updated = true;
        }
        return updated;
    }

    public void pausePlayback() {
        if (mMediaPlayer != null && mMediaPlayer.isPlaying()) {
            mMediaPlayer.pause();
        }
    }

    public void resumePlayback() {
        if (mMediaPlayer != null && !mMediaPlayer.isPlaying()) {
            mMediaPlayer.start();
        }
    }

    public void mute() {
        if (mMediaPlayer != null) {
            mMediaPlayer.setVolume(0, 0);
        }
    }

    public void unmute() {
        if (mMediaPlayer != null) {
            mMediaPlayer.setVolume(1, 1);
        }
    }

    /** Pitch-preserving tempo via {@link PlaybackParams} (API 23+). */
    public void setPlaybackRate(double rate) {
        float clamped = (float) Math.max(0.05, rate);
        mPlaybackRate = clamped;
        if (mMediaPlayer == null || !mIsPrepared) {
            return;
        }
        applyPlaybackRate(clamped);
    }

    private void applyPlaybackRate(float rate) {
        if (mMediaPlayer == null) {
            return;
        }
        if (android.os.Build.VERSION.SDK_INT < android.os.Build.VERSION_CODES.M) {
            Log.w("MakepadVideoPlayer", "setPlaybackRate requires API 23+; ignored rate=" + rate);
            return;
        }
        boolean wasPlaying = false;
        try {
            wasPlaying = mMediaPlayer.isPlaying();
        } catch (IllegalStateException ignored) {
            return;
        }
        if (trySetPlaybackParams(rate, false)) {
            resumeIfNeeded(wasPlaying);
            return;
        }
        // HW audio often cannot time-stretch; FAIL is the OEM default and
        // aborts the whole params change so video stays at 1×.
        if (trySetPlaybackParams(rate, true)) {
            resumeIfNeeded(wasPlaying);
            return;
        }
        Log.e("MakepadVideoPlayer", "setPlaybackRate failed rate=" + rate);
    }

    private boolean trySetPlaybackParams(float rate, boolean muteAudioFallback) {
        try {
            PlaybackParams params;
            try {
                params = mMediaPlayer.getPlaybackParams();
            } catch (Exception e) {
                params = new PlaybackParams();
            }
            params.setSpeed(rate);
            params.setPitch(1.0f);
            params.setAudioFallbackMode(
                muteAudioFallback
                    ? PlaybackParams.AUDIO_FALLBACK_MODE_MUTE
                    : PlaybackParams.AUDIO_FALLBACK_MODE_DEFAULT
            );
            mMediaPlayer.setPlaybackParams(params);
            Log.i(
                "MakepadVideoPlayer",
                "setPlaybackRate ok rate=" + rate + " muteFallback=" + muteAudioFallback
            );
            return true;
        } catch (Exception e) {
            Log.w(
                "MakepadVideoPlayer",
                "setPlaybackRate try failed rate=" + rate
                    + " muteFallback=" + muteAudioFallback + ": " + e
            );
            return false;
        }
    }

    private void resumeIfNeeded(boolean wasPlaying) {
        try {
            if (wasPlaying && mMediaPlayer != null && !mMediaPlayer.isPlaying()) {
                mMediaPlayer.start();
            }
        } catch (IllegalStateException ignored) {
        }
    }

    public void seekToPosition(long positionMs) {
        if (mMediaPlayer != null) {
            mMediaPlayer.seekTo((int) positionMs);
        }
    }

    public long getCurrentPositionMs() {
        if (mMediaPlayer != null && mIsPrepared) {
            return (long) mMediaPlayer.getCurrentPosition();
        }
        return 0;
    }

    public void stopAndCleanup() {
        // stop and release MediaPlayer
        if (mMediaPlayer != null) {
            mMediaPlayer.stop();
            mMediaPlayer.release();
            mMediaPlayer = null;
        }

        mSource = null;

        // release the SurfaceTexture and Surface
        if (mSurfaceTexture != null) {
            mSurfaceTexture.release();
            mSurfaceTexture = null;
        }

        // stop the HandlerThread
        if (mHandlerThread != null) {
            mHandlerThread.quitSafely();
            try {
                mHandlerThread.join();
                mHandlerThread = null;
                mGlHandler = null;
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        Activity activity = mActivityReference.get();
        if (activity != null) {
            activity.runOnUiThread(() -> {
                MakepadNative.onVideoPlayerReleased(mVideoId);
            });
        }
    }

    public void setExternalTextureHandle(int textureHandle) {
        mExternalTextureHandle = textureHandle;
    }

    public void setAutoplay(boolean autoplay) {
        mAutoplay = autoplay;
    }

    public void setShouldLoop(boolean shouldLoop) {
        mShouldLoop = shouldLoop;
    }

    public void setSource(Object source) {
        mSource = source;
    }

    private long mVideoId;

    // player
    private MediaPlayer mMediaPlayer;
    private boolean mIsPrepared = false; 
    private boolean mIsDecoding = false;
    private Object mSource;
    private int mExternalTextureHandle;
    private SurfaceTexture mSurfaceTexture;
    private AtomicInteger mAvailableFrames = new AtomicInteger(0);
    private Handler mGlHandler;
    private HandlerThread mHandlerThread;

    // playback
    private boolean mAutoplay = false;
    private boolean mShouldLoop = false;
    private float mPlaybackRate = 1.0f;
    
    // context
    private WeakReference<Activity> mActivityReference;
}
