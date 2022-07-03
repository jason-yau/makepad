use crate::{
    makepad_live_id::*,
    makepad_wasm_msg::*,
    makepad_math::{Vec2, Vec3, Quat},
    cx::{PlatformType},
    event::{
        KeyCode,
        FingerDownEvent,
        KeyModifiers,
        FingerInputType,
        FingerUpEvent,
        FingerMoveEvent,
        FingerHoverEvent,
        FingerScrollEvent,
        KeyEvent,
        TextInputEvent,
    },
    window::WindowGeom
};

#[derive(ToWasm)]
pub struct ToWasmGpuInfo {
    pub min_uniform_vectors: u32,
    pub vendor: String,
    pub renderer: String
}

#[derive(ToWasm)]
pub struct ToWasmBrowserInfo {
    pub port: u32,
    pub protocol: String,
    pub hostname: String,
    pub pathname: String,
    pub search: String,
    pub hash: String,
}

impl Into<PlatformType> for ToWasmBrowserInfo {
    fn into(self) -> PlatformType {
        PlatformType::WebBrowser {
            port: self.port as u16,
            protocol: self.protocol,
            hostname: self.hostname,
            pathname: self.pathname,
            search: self.search,
            hash: self.hash,
        }
    }
}

#[derive(ToWasm)]
pub struct ToWasmConstructAndGetDeps {
    pub gpu_info: ToWasmGpuInfo,
    pub browser_info: ToWasmBrowserInfo,
}

#[derive(ToWasm)]
pub struct ToWasmDepLoaded {
    pub path: String,
    pub data: ToWasmDataU8
}

#[derive(ToWasm)]
pub struct ToWasmDepsLoaded {
    pub gpu_info: ToWasmGpuInfo,
    pub browser_info: ToWasmBrowserInfo,
    pub deps: Vec<ToWasmDepLoaded>
}

#[derive(ToWasm)]
pub struct ToWasmWindowInfo {
    pub is_fullscreen: bool,
    pub can_fullscreen: bool,
    pub xr_is_presenting: bool,
    pub xr_can_present: bool,
    pub dpi_factor: f32,
    pub inner_width: f32,
    pub inner_height: f32
}

impl Into<WindowGeom> for ToWasmWindowInfo {
    fn into(self) -> WindowGeom {
        WindowGeom {
            is_fullscreen: self.is_fullscreen,
            is_topmost: false,
            inner_size: Vec2 {x: self.inner_width, y: self.inner_height},
            dpi_factor: self.dpi_factor,
            outer_size: Vec2 {x: 0., y: 0.},
            position: Vec2 {x: 0., y: 0.},
            xr_is_presenting: self.xr_is_presenting,
            xr_can_present: self.xr_can_present,
            can_fullscreen: self.can_fullscreen
        }
    }
}

#[derive(ToWasm)]
pub struct ToWasmInit {
    pub window_info: ToWasmWindowInfo
}

#[derive(ToWasm)]
pub struct ToWasmResizeWindow {
    pub window_info: ToWasmWindowInfo
}

#[derive(ToWasm)]
pub struct ToWasmAnimationFrame {
    pub time: f64
}


#[derive(ToWasm)]
pub struct ToWasmVec2 {
    pub x: f32,
    pub y: f32,
}

impl Into<Vec2> for ToWasmVec2 {
    fn into(self) -> Vec2 {
        Vec2{x:self.x, y: self.y}
    }
}

#[derive(ToWasm)]
pub struct ToWasmVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Into<Vec3> for ToWasmVec3 {
    fn into(self) -> Vec3 {
        Vec3{x:self.x, y: self.y, z:self.z}
    }
}

#[derive(ToWasm)]
pub struct ToWasmQuat {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Into<Quat> for ToWasmQuat {
    fn into(self) -> Quat {
        Quat{a:self.a, b: self.b, c:self.c, d:self.d}
    }
}

#[derive(ToWasm)]
pub struct ToWasmFinger {
    pub abs: ToWasmVec2,
    pub digit: usize,
    pub is_touch: bool,
    pub modifiers: u32,
    pub time: f64,
}

#[derive(ToWasm)]
pub struct ToWasmFingerDown {
    pub finger: ToWasmFinger,
}

fn unpack_key_modifier(modifiers: u32) -> KeyModifiers {
    KeyModifiers {
        shift: (modifiers & 1) != 0,
        control: (modifiers & 2) != 0,
        alt: (modifiers & 4) != 0,
        logo: (modifiers & 8) != 0
    }
}

impl ToWasmFingerDown {
    pub fn into_finger_down_event(self, tap_count: u32) -> FingerDownEvent {
        FingerDownEvent {
            window_id: 0,
            abs: self.finger.abs.into(),
            handled: false,
            digit: self.finger.digit,
            input_type: if self.finger.is_touch {FingerInputType::Touch} else {FingerInputType::Mouse},
            modifiers: unpack_key_modifier(self.finger.modifiers),
            time: self.finger.time,
            tap_count: tap_count
        }
    }
}

#[derive(ToWasm)]
pub struct ToWasmFingerUp {
    pub finger: ToWasmFinger,
}

impl Into<FingerUpEvent> for ToWasmFingerUp {
    fn into(self) -> FingerUpEvent {
        FingerUpEvent {
            window_id: 0,
            abs: self.finger.abs.into(),
            digit: self.finger.digit,
            input_type: if self.finger.is_touch {FingerInputType::Touch} else {FingerInputType::Mouse},
            modifiers: unpack_key_modifier(self.finger.modifiers),
            time: self.finger.time,
        }
    }
}

#[derive(ToWasm)]
pub struct ToWasmFingerMove {
    pub finger: ToWasmFinger,
}

impl Into<FingerMoveEvent> for ToWasmFingerMove {
    fn into(self) -> FingerMoveEvent {
        FingerMoveEvent {
            window_id: 0,
            abs: self.finger.abs.into(),
            digit: self.finger.digit,
            input_type: if self.finger.is_touch {FingerInputType::Touch} else {FingerInputType::Mouse},
            modifiers: unpack_key_modifier(self.finger.modifiers),
            time: self.finger.time,
        }
    }
}

#[derive(ToWasm)]
pub struct ToWasmFingerHover {
    pub finger: ToWasmFinger,
}

impl Into<FingerHoverEvent> for ToWasmFingerHover {
    fn into(self) -> FingerHoverEvent {
        FingerHoverEvent {
            window_id: 0,
            abs: self.finger.abs.into(),
            digit: self.finger.digit,
            handled: false,
            modifiers: unpack_key_modifier(self.finger.modifiers),
            time: self.finger.time,
        }
    }
}


#[derive(ToWasm)]
pub struct ToWasmFingerScroll {
    pub finger: ToWasmFinger,
    pub scroll: ToWasmVec2,
}

impl Into<FingerScrollEvent> for ToWasmFingerScroll {
    fn into(self) -> FingerScrollEvent {
        FingerScrollEvent {
            window_id: 0,
            digit: self.finger.digit,
            abs: self.finger.abs.into(),
            scroll: self.scroll.into(),
            input_type: if self.finger.is_touch {FingerInputType::Touch} else {FingerInputType::Mouse},
            handled_x: false,
            handled_y: false,
            modifiers: unpack_key_modifier(self.finger.modifiers),
            time: self.finger.time,
        }
    }
}

fn web_to_key_code(key_code: u32) -> KeyCode {
    match key_code {
        27 => KeyCode::Escape,
        192 => KeyCode::Backtick,
        48 => KeyCode::Key0,
        49 => KeyCode::Key1,
        50 => KeyCode::Key2,
        51 => KeyCode::Key3,
        52 => KeyCode::Key4,
        53 => KeyCode::Key5,
        54 => KeyCode::Key6,
        55 => KeyCode::Key7,
        56 => KeyCode::Key8,
        57 => KeyCode::Key9,
        173 => KeyCode::Minus,
        189 => KeyCode::Minus,
        61 => KeyCode::Equals,
        187 => KeyCode::Equals,
        
        8 => KeyCode::Backspace,
        9 => KeyCode::Tab,
        
        81 => KeyCode::KeyQ,
        87 => KeyCode::KeyW,
        69 => KeyCode::KeyE,
        82 => KeyCode::KeyR,
        84 => KeyCode::KeyT,
        89 => KeyCode::KeyY,
        85 => KeyCode::KeyU,
        73 => KeyCode::KeyI,
        79 => KeyCode::KeyO,
        80 => KeyCode::KeyP,
        219 => KeyCode::LBracket,
        221 => KeyCode::RBracket,
        13 => KeyCode::Return,
        
        65 => KeyCode::KeyA,
        83 => KeyCode::KeyS,
        68 => KeyCode::KeyD,
        70 => KeyCode::KeyF,
        71 => KeyCode::KeyG,
        72 => KeyCode::KeyH,
        74 => KeyCode::KeyJ,
        75 => KeyCode::KeyK,
        76 => KeyCode::KeyL,
        
        59 => KeyCode::Semicolon,
        186 => KeyCode::Semicolon,
        222 => KeyCode::Quote,
        220 => KeyCode::Backslash,
        
        90 => KeyCode::KeyZ,
        88 => KeyCode::KeyX,
        67 => KeyCode::KeyC,
        86 => KeyCode::KeyV,
        66 => KeyCode::KeyB,
        78 => KeyCode::KeyN,
        77 => KeyCode::KeyM,
        188 => KeyCode::Comma,
        190 => KeyCode::Period,
        191 => KeyCode::Slash,
        
        17 => KeyCode::Control,
        18 => KeyCode::Alt,
        16 => KeyCode::Shift,
        224 => KeyCode::Logo,
        91 => KeyCode::Logo,
        
        //RightControl,
        //RightShift,
        //RightAlt,
        93 => KeyCode::Logo,
        
        32 => KeyCode::Space,
        20 => KeyCode::Capslock,
        112 => KeyCode::F1,
        113 => KeyCode::F2,
        114 => KeyCode::F3,
        115 => KeyCode::F4,
        116 => KeyCode::F5,
        117 => KeyCode::F6,
        118 => KeyCode::F7,
        119 => KeyCode::F8,
        120 => KeyCode::F9,
        121 => KeyCode::F10,
        122 => KeyCode::F11,
        123 => KeyCode::F12,
        
        44 => KeyCode::PrintScreen,
        124 => KeyCode::PrintScreen,
        //Scrolllock,
        //Pause,
        
        45 => KeyCode::Insert,
        46 => KeyCode::Delete,
        36 => KeyCode::Home,
        35 => KeyCode::End,
        33 => KeyCode::PageUp,
        34 => KeyCode::PageDown,
        
        96 => KeyCode::Numpad0,
        97 => KeyCode::Numpad1,
        98 => KeyCode::Numpad2,
        99 => KeyCode::Numpad3,
        100 => KeyCode::Numpad4,
        101 => KeyCode::Numpad5,
        102 => KeyCode::Numpad6,
        103 => KeyCode::Numpad7,
        104 => KeyCode::Numpad8,
        105 => KeyCode::Numpad9,
        
        //NumpadEquals,
        109 => KeyCode::NumpadSubtract,
        107 => KeyCode::NumpadAdd,
        110 => KeyCode::NumpadDecimal,
        106 => KeyCode::NumpadMultiply,
        111 => KeyCode::NumpadDivide,
        12 => KeyCode::Numlock,
        //NumpadEnter,
        
        38 => KeyCode::ArrowUp,
        40 => KeyCode::ArrowDown,
        37 => KeyCode::ArrowLeft,
        39 => KeyCode::ArrowRight,
        _ => KeyCode::Unknown
    }
}

#[derive(ToWasm)]
pub struct ToWasmKey {
    pub key_code: u32,
    pub modifiers: u32,
    pub time: f64,
    pub is_repeat: bool
}

impl Into<KeyEvent> for ToWasmKey {
    fn into(self) -> KeyEvent {
        KeyEvent {
            key_code: web_to_key_code(self.key_code),
            is_repeat: self.is_repeat,
            modifiers: unpack_key_modifier(self.modifiers),
            time: self.time,
        }
    }
}


#[derive(ToWasm)]
pub struct ToWasmTextInput {
    pub was_paste: bool,
    pub replace_last: bool,
    pub input: String,
}

impl Into<TextInputEvent> for ToWasmTextInput {
    fn into(self) -> TextInputEvent {
        TextInputEvent {
            was_paste: self.was_paste,
            replace_last: self.replace_last,
            input: self.input
        }
    }
}

#[derive(ToWasm)]
pub struct ToWasmTextCopy {
}

#[derive(ToWasm)]
pub struct ToWasmTimerFired {
    timer_id: usize
}

#[derive(ToWasm)]
pub struct ToWasmPaintDirty {
}

#[derive(ToWasm)]
pub struct ToWasmWindowFocusChange {
    has_focus: bool
}

#[derive(ToWasm)]
pub struct ToWasmXRButton {
    pressed: bool,
    value: f32
}

#[derive(ToWasm)]
pub struct ToWasmXRTransform {
    orientation: ToWasmQuat,
    position: ToWasmVec3,
}

#[derive(ToWasm)]
pub struct ToWasmXRInput {
    active: bool,
    hand: u32,
    grip: ToWasmXRTransform,
    ray: ToWasmXRTransform,
    buttons: Vec<ToWasmXRButton>,
    axes: Vec<f32>
}

#[derive(ToWasm)]
pub struct ToWasmXRUpdate {
    inputs: Vec<ToWasmXRInput>
}
