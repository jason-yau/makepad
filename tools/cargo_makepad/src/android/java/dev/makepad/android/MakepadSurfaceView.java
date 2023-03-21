package dev.makepad.android;

import android.Manifest;
import android.content.Context;
import android.graphics.Canvas;
import android.opengl.GLES20;
import android.view.MotionEvent;
import android.view.View;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.util.Log;

import javax.microedition.khronos.egl.EGL10;
import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.egl.EGLContext;
import javax.microedition.khronos.egl.EGLDisplay;
import javax.microedition.khronos.egl.EGLSurface;
import javax.microedition.khronos.opengles.GL10;

import android.view.KeyEvent;

public class MakepadSurfaceView extends SurfaceView implements 
SurfaceHolder.Callback, 
View.OnTouchListener,
KeyEvent.Callback
{
    public MakepadSurfaceView(Context context, long cx) {
        super(context);

        setWillNotDraw(false);
        getHolder().addCallback(this);
        setOnTouchListener(this);

        mCx = cx;

        mEgl = (EGL10) EGLContext.getEGL();

        mEglDisplay = mEgl.eglGetDisplay(EGL10.EGL_DEFAULT_DISPLAY);
        if (mEglDisplay == EGL10.EGL_NO_DISPLAY) {
            throw new RuntimeException("eglGetDisplay failed");
        }

        int[] version = new int[2];
        if (!mEgl.eglInitialize(mEglDisplay, version)) {
            throw new RuntimeException("eglInitialize failed");
        }
        int[] attrib_list = new int[]{
                EGL10.EGL_RED_SIZE, 8,
                EGL10.EGL_GREEN_SIZE, 8,
                EGL10.EGL_BLUE_SIZE, 8,
                EGL10.EGL_ALPHA_SIZE, 8,
                EGL10.EGL_DEPTH_SIZE, 24,
                EGL10.EGL_STENCIL_SIZE, 8,
                EGL10.EGL_NONE,
        };
        EGLConfig[] configs = new EGLConfig[1];
        int[] num_config = new int[1];
        if (!mEgl.eglChooseConfig(mEglDisplay, new int[]{
                EGL10.EGL_RED_SIZE, 8,
                EGL10.EGL_GREEN_SIZE, 8,
                EGL10.EGL_BLUE_SIZE, 8,
                EGL10.EGL_ALPHA_SIZE, 8,
                EGL10.EGL_DEPTH_SIZE, 24,
                EGL10.EGL_STENCIL_SIZE, 8,
                EGL10.EGL_NONE,
        }, configs, 1, num_config)) {
            throw new RuntimeException("eglChooseConfig failed");
        }
        if (num_config[0] == 0) {
            throw new RuntimeException("No suitable EGL context found");
        }
        mEglConfig = configs[0];
    }

    @Override
    public void onDraw(Canvas canvas) {      
        if (!mEgl.eglMakeCurrent(mEglDisplay, mEglSurface, mEglSurface, mEglContext)) {
            throw new RuntimeException("eglMakeCurrent failed");
        }
        Makepad.onDraw(mCx, (Makepad.Callback)this.getContext());
    }

    public void surfaceCreated(SurfaceHolder holder) {
        int[] attrib_list = new int[]{
                EGL_CONTEXT_CLIENT_VERSION, 2,
                EGL10.EGL_NONE
        };
        mEglContext = mEgl.eglCreateContext(mEglDisplay, mEglConfig, EGL10.EGL_NO_CONTEXT, attrib_list);
        if (mEglContext == EGL10.EGL_NO_CONTEXT) {
            throw new RuntimeException("eglCreateContext failed");
        }

        mEglSurface = mEgl.eglCreateWindowSurface(mEglDisplay, mEglConfig, getHolder(), null);
        if (mEglSurface == EGL10.EGL_NO_SURFACE) {
            throw new RuntimeException("eglCreateWindowSurface failed");
        }

        if (!mEgl.eglMakeCurrent(mEglDisplay, mEglSurface, mEglSurface, mEglContext)) {
            throw new RuntimeException("eglMakeCurrent failed");
        }
    }

    public void surfaceDestroyed(SurfaceHolder holder) {
        if (!mEgl.eglDestroySurface(mEglDisplay, mEglSurface)) {
            throw new RuntimeException("eglDestroySurface failed");
        }
    }

    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        Makepad.onResize(mCx, width, height, (Makepad.Callback)this.getContext());
    }

    public boolean onTouch(View view, MotionEvent event) {
        Makepad.onTouch(mCx, event, (Makepad.Callback)this.getContext());
        return true;
    }

    public void swapBuffers() {
        if (!mEgl.eglSwapBuffers(mEglDisplay, mEglSurface)) {
            throw new RuntimeException("eglSwapBuffers failed");
        }
    }

    @Override
    public boolean onKeyDown(int keyCode, KeyEvent event) {
        Makepad.onKeyDown(mCx, keyCode, (Makepad.Callback)this.getContext());
        return true;
    }

    private static final int EGL_CONTEXT_CLIENT_VERSION = 0x3098;
    private long mCx;
    private EGL10 mEgl;
    private EGLDisplay mEglDisplay;
    private EGLConfig mEglConfig;
    private EGLContext mEglContext;
    private EGLSurface mEglSurface;
}
