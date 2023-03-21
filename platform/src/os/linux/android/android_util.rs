use {
  crate::event::KeyCode
};


pub fn to_key_code(key_code: i32) -> KeyCode {
  match key_code {      
      29 => KeyCode::KeyA,
      30 => KeyCode::KeyB,
      31 => KeyCode::KeyC,
      32 => KeyCode::KeyD,
      33 => KeyCode::KeyE,
      34 => KeyCode::KeyF,
      35 => KeyCode::KeyG,
      36 => KeyCode::KeyH,
      37 => KeyCode::KeyI,
      38 => KeyCode::KeyJ,
      39 => KeyCode::KeyK,
      40 => KeyCode::KeyL,
      41 => KeyCode::KeyM,
      42 => KeyCode::KeyN,
      43 => KeyCode::KeyO,
      44 => KeyCode::KeyP,
      45 => KeyCode::KeyQ,
      46 => KeyCode::KeyR,
      47 => KeyCode::KeyS,
      48 => KeyCode::KeyT,
      49 => KeyCode::KeyU,
      50 => KeyCode::KeyV,
      51 => KeyCode::KeyW,
      52 => KeyCode::KeyX,
      53 => KeyCode::KeyY,
      54 => KeyCode::KeyZ,
      _ => KeyCode::Unknown
  }
}

pub fn keycode_to_string(keycode: KeyCode, shift: bool) -> &'static str {
  if !shift {
      match keycode {
          KeyCode::Backtick => "`",
          KeyCode::Key0 => "0",
          KeyCode::Key1 => "1",
          KeyCode::Key2 => "2",
          KeyCode::Key3 => "3",
          KeyCode::Key4 => "4",
          KeyCode::Key5 => "5",
          KeyCode::Key6 => "6",
          KeyCode::Key7 => "7",
          KeyCode::Key8 => "8",
          KeyCode::Key9 => "9",
          KeyCode::Minus => "-",
          KeyCode::Equals => "=",
          
          KeyCode::KeyQ => "q",
          KeyCode::KeyW => "w",
          KeyCode::KeyE => "e",
          KeyCode::KeyR => "r",
          KeyCode::KeyT => "t",
          KeyCode::KeyY => "y",
          KeyCode::KeyU => "u",
          KeyCode::KeyI => "i",
          KeyCode::KeyO => "o",
          KeyCode::KeyP => "p",
          KeyCode::LBracket => "[",
          KeyCode::RBracket => "]",
          
          KeyCode::KeyA => "a",
          KeyCode::KeyS => "s",
          KeyCode::KeyD => "d",
          KeyCode::KeyF => "f",
          KeyCode::KeyG => "g",
          KeyCode::KeyH => "h",
          KeyCode::KeyJ => "j",
          KeyCode::KeyK => "l",
          KeyCode::KeyL => "l",
          KeyCode::Semicolon => ";",
          KeyCode::Quote => "'",
          KeyCode::Backslash => "\\",
          
          KeyCode::KeyZ => "z",
          KeyCode::KeyX => "x",
          KeyCode::KeyC => "c",
          KeyCode::KeyV => "v",
          KeyCode::KeyB => "b",
          KeyCode::KeyN => "n",
          KeyCode::KeyM => "m",
          KeyCode::Comma => ",",
          KeyCode::Period => ".",
          KeyCode::Slash => "/",
          _ => ""
      }
  }
  else {
      match keycode {
          KeyCode::Backtick => "~",
          KeyCode::Key0 => "!",
          KeyCode::Key1 => "@",
          KeyCode::Key2 => "#",
          KeyCode::Key3 => "$",
          KeyCode::Key4 => "%",
          KeyCode::Key5 => "^",
          KeyCode::Key6 => "&",
          KeyCode::Key7 => "*",
          KeyCode::Key8 => "(",
          KeyCode::Key9 => ")",
          KeyCode::Minus => "_",
          KeyCode::Equals => "+",
          
          KeyCode::KeyQ => "Q",
          KeyCode::KeyW => "W",
          KeyCode::KeyE => "E",
          KeyCode::KeyR => "R",
          KeyCode::KeyT => "T",
          KeyCode::KeyY => "Y",
          KeyCode::KeyU => "U",
          KeyCode::KeyI => "I",
          KeyCode::KeyO => "O",
          KeyCode::KeyP => "P",
          KeyCode::LBracket => "{",
          KeyCode::RBracket => "}",
          
          KeyCode::KeyA => "A",
          KeyCode::KeyS => "S",
          KeyCode::KeyD => "D",
          KeyCode::KeyF => "F",
          KeyCode::KeyG => "G",
          KeyCode::KeyH => "H",
          KeyCode::KeyJ => "J",
          KeyCode::KeyK => "K",
          KeyCode::KeyL => "L",
          KeyCode::Semicolon => ":",
          KeyCode::Quote => "\"",
          KeyCode::Backslash => "|",
          
          KeyCode::KeyZ => "Z",
          KeyCode::KeyX => "X",
          KeyCode::KeyC => "C",
          KeyCode::KeyV => "V",
          KeyCode::KeyB => "B",
          KeyCode::KeyN => "N",
          KeyCode::KeyM => "M",
          KeyCode::Comma => "<",
          KeyCode::Period => ">",
          KeyCode::Slash => "?",
          _ => ""
      }
  }
}