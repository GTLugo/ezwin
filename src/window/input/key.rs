use strum::EnumIter;
use windows::Win32::UI::Input::{KeyboardAndMouse::VIRTUAL_KEY, *};

#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Key {
  Unknown = 0,
  // ASCII
  Tab = 9,
  Enter = 10,
  Space = 32,
  Apostrophe = 39,
  Comma = 44,
  Minus = 45,
  Period = 46,
  ForwardSlash = 47,
  _0 = 48,
  _1 = 49,
  _2 = 50,
  _3 = 51,
  _4 = 52,
  _5 = 53,
  _6 = 54,
  _7 = 55,
  _8 = 56,
  _9 = 57,
  Semicolon = 59,
  Equals = 61,
  A = 65,
  B = 66,
  C = 67,
  D = 68,
  E = 69, // ;)
  F = 70,
  G = 71,
  H = 72,
  I = 73,
  J = 74,
  K = 75,
  L = 76,
  M = 77,
  N = 78,
  O = 79,
  P = 80,
  Q = 81,
  R = 82,
  S = 83,
  T = 84,
  U = 85,
  V = 86,
  W = 87,
  X = 88,
  Y = 89,
  Z = 90,
  LeftBracket = 91,
  BackSlash = 92,
  RightBracket = 93,
  Accent = 96,
  // Non-ASCII
  Escape = 256,
  NumEnter,
  Backspace,
  Insert,
  Delete,
  Up,
  Down,
  Left,
  Right,
  PageUp,
  PageDown,
  Home,
  End,
  CapsLock,
  ScrollLock,
  NumLock,
  PrintScreen,
  Pause,
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  F25,
  NumPeriod,
  NumComma,
  NumPlus,
  NumMinus,
  NumDivide,
  NumMultiply,
  NumEquals,
  LeftShift,
  LeftControl,
  LeftAlt,
  LeftSuper,
  RightShift,
  RightControl,
  RightAlt,
  RightSuper,
  Menu,
  Caret,
  AbntC1,
  AbntC2,
  Asterisk,
  At,
  Ax,
  Calculator,
  Colon,
  Convert,
  Kana,
  Kanji,
  Mail,
  MyComputer,
  NavigateForward,
  NavigateBackward,
  MediaPlayPause,
  MediaStop,
  MediaSelect,
  MediaNextTrack,
  MediaPrevTrack,
  VolumeDown,
  VolumeUp,
  VolumeMute,
  NoConvert,
  OEM102,
  Plus,
  Power,
  Sleep,
  Stop,
  SysRq,
  Underline,
  NoName,
  Wake,
  WebBack,
  WebFavorites,
  WebForward,
  WebHome,
  WebRefresh,
  WebSearch,
  WebStop,
  Yen,
  Copy,
  Paste,
  Cut,
}

impl From<VIRTUAL_KEY> for Key {
  fn from(value: VIRTUAL_KEY) -> Self {
    // #[allow(unreachable_patterns)]
    match value {
      KeyboardAndMouse::VK_1 => Key::_1,
      KeyboardAndMouse::VK_2 => Key::_2,
      KeyboardAndMouse::VK_3 => Key::_3,
      KeyboardAndMouse::VK_4 => Key::_4,
      KeyboardAndMouse::VK_5 => Key::_5,
      KeyboardAndMouse::VK_6 => Key::_6,
      KeyboardAndMouse::VK_7 => Key::_7,
      KeyboardAndMouse::VK_8 => Key::_8,
      KeyboardAndMouse::VK_9 => Key::_9,
      KeyboardAndMouse::VK_0 => Key::_0,
      KeyboardAndMouse::VK_A => Key::A,
      KeyboardAndMouse::VK_B => Key::B,
      KeyboardAndMouse::VK_C => Key::C,
      KeyboardAndMouse::VK_D => Key::D,
      KeyboardAndMouse::VK_E => Key::E,
      KeyboardAndMouse::VK_F => Key::F,
      KeyboardAndMouse::VK_G => Key::G,
      KeyboardAndMouse::VK_H => Key::H,
      KeyboardAndMouse::VK_I => Key::I,
      KeyboardAndMouse::VK_J => Key::J,
      KeyboardAndMouse::VK_K => Key::K,
      KeyboardAndMouse::VK_L => Key::L,
      KeyboardAndMouse::VK_M => Key::M,
      KeyboardAndMouse::VK_N => Key::N,
      KeyboardAndMouse::VK_O => Key::O,
      KeyboardAndMouse::VK_P => Key::P,
      KeyboardAndMouse::VK_Q => Key::Q,
      KeyboardAndMouse::VK_R => Key::R,
      KeyboardAndMouse::VK_S => Key::S,
      KeyboardAndMouse::VK_T => Key::T,
      KeyboardAndMouse::VK_U => Key::U,
      KeyboardAndMouse::VK_V => Key::V,
      KeyboardAndMouse::VK_W => Key::W,
      KeyboardAndMouse::VK_X => Key::X,
      KeyboardAndMouse::VK_Y => Key::Y,
      KeyboardAndMouse::VK_Z => Key::Z,
      KeyboardAndMouse::VK_ESCAPE => Key::Escape,
      KeyboardAndMouse::VK_F1 => Key::F1,
      KeyboardAndMouse::VK_F2 => Key::F2,
      KeyboardAndMouse::VK_F3 => Key::F3,
      KeyboardAndMouse::VK_F4 => Key::F4,
      KeyboardAndMouse::VK_F5 => Key::F5,
      KeyboardAndMouse::VK_F6 => Key::F6,
      KeyboardAndMouse::VK_F7 => Key::F7,
      KeyboardAndMouse::VK_F8 => Key::F8,
      KeyboardAndMouse::VK_F9 => Key::F9,
      KeyboardAndMouse::VK_F10 => Key::F10,
      KeyboardAndMouse::VK_F11 => Key::F11,
      KeyboardAndMouse::VK_F12 => Key::F12,
      KeyboardAndMouse::VK_F13 => Key::F13,
      KeyboardAndMouse::VK_F14 => Key::F14,
      KeyboardAndMouse::VK_F15 => Key::F15,
      KeyboardAndMouse::VK_F16 => Key::F16,
      KeyboardAndMouse::VK_F17 => Key::F17,
      KeyboardAndMouse::VK_F18 => Key::F18,
      KeyboardAndMouse::VK_F19 => Key::F19,
      KeyboardAndMouse::VK_F20 => Key::F20,
      KeyboardAndMouse::VK_F21 => Key::F21,
      KeyboardAndMouse::VK_F22 => Key::F22,
      KeyboardAndMouse::VK_F23 => Key::F23,
      KeyboardAndMouse::VK_F24 => Key::F24,
      KeyboardAndMouse::VK_SNAPSHOT => Key::PrintScreen,
      KeyboardAndMouse::VK_SCROLL => Key::ScrollLock,
      KeyboardAndMouse::VK_PAUSE => Key::Pause,
      KeyboardAndMouse::VK_INSERT => Key::Insert,
      KeyboardAndMouse::VK_HOME => Key::Home,
      KeyboardAndMouse::VK_DELETE => Key::Delete,
      KeyboardAndMouse::VK_END => Key::End,
      KeyboardAndMouse::VK_NEXT => Key::PageDown,
      KeyboardAndMouse::VK_PRIOR => Key::PageUp,
      KeyboardAndMouse::VK_LEFT => Key::Left,
      KeyboardAndMouse::VK_UP => Key::Up,
      KeyboardAndMouse::VK_RIGHT => Key::Right,
      KeyboardAndMouse::VK_DOWN => Key::Down,
      KeyboardAndMouse::VK_BACK => Key::Backspace,
      KeyboardAndMouse::VK_RETURN => Key::Enter,
      KeyboardAndMouse::VK_SPACE => Key::Space,
      //KeyboardAndMouse::VK_                    => KeyCode::Compose,
      //KeyboardAndMouse::VK_                    => KeyCode::Caret,
      KeyboardAndMouse::VK_NUMLOCK => Key::NumLock,
      KeyboardAndMouse::VK_NUMPAD0 => Key::Num0,
      KeyboardAndMouse::VK_NUMPAD1 => Key::Num1,
      KeyboardAndMouse::VK_NUMPAD2 => Key::Num2,
      KeyboardAndMouse::VK_NUMPAD3 => Key::Num3,
      KeyboardAndMouse::VK_NUMPAD4 => Key::Num4,
      KeyboardAndMouse::VK_NUMPAD5 => Key::Num5,
      KeyboardAndMouse::VK_NUMPAD6 => Key::Num6,
      KeyboardAndMouse::VK_NUMPAD7 => Key::Num7,
      KeyboardAndMouse::VK_NUMPAD8 => Key::Num8,
      KeyboardAndMouse::VK_NUMPAD9 => Key::Num9,
      KeyboardAndMouse::VK_ADD => Key::NumPlus,
      KeyboardAndMouse::VK_SUBTRACT => Key::NumMinus,
      KeyboardAndMouse::VK_MULTIPLY => Key::NumMultiply,
      KeyboardAndMouse::VK_DIVIDE => Key::NumDivide,
      KeyboardAndMouse::VK_DECIMAL => Key::NumPeriod,
      //KeyboardAndMouse::VK_COMMA               => KeyCode::NumComma,
      //KeyboardAndMouse::VK_ENTER               => KeyCode::NumEnter,
      //KeyboardAndMouse::VK_EQUALS              => KeyCode::NumEquals,
      KeyboardAndMouse::VK_ABNT_C1 => Key::AbntC1,
      KeyboardAndMouse::VK_ABNT_C2 => Key::AbntC2,
      KeyboardAndMouse::VK_OEM_7 => Key::Apostrophe,
      KeyboardAndMouse::VK_APPS => Key::Menu,
      //KeyboardAndMouse::VK_ASTERISK            => KeyCode::Asterisk,
      //KeyboardAndMouse::VK_AT                  => KeyCode::At,
      KeyboardAndMouse::VK_OEM_AX => Key::Ax,
      KeyboardAndMouse::VK_OEM_5 => Key::BackSlash,
      //KeyboardAndMouse::VK_LAUNCH_CALCULATOR   => KeyCode::Calculator,
      KeyboardAndMouse::VK_CAPITAL => Key::CapsLock,
      //KeyboardAndMouse::VK_COLON               => KeyCode::Colon,
      KeyboardAndMouse::VK_OEM_COMMA => Key::Comma,
      KeyboardAndMouse::VK_CONVERT => Key::Convert,
      KeyboardAndMouse::VK_OEM_PLUS => Key::Equals,
      KeyboardAndMouse::VK_OEM_3 => Key::Accent,
      KeyboardAndMouse::VK_KANA => Key::Kana,
      KeyboardAndMouse::VK_KANJI => Key::Kanji,
      KeyboardAndMouse::VK_LMENU => Key::LeftAlt,
      KeyboardAndMouse::VK_OEM_4 => Key::LeftBracket,
      KeyboardAndMouse::VK_LCONTROL => Key::LeftControl,
      KeyboardAndMouse::VK_LSHIFT => Key::LeftShift,
      KeyboardAndMouse::VK_LWIN => Key::LeftSuper,
      KeyboardAndMouse::VK_LAUNCH_MAIL => Key::Mail,
      KeyboardAndMouse::VK_LAUNCH_MEDIA_SELECT => Key::MediaSelect,
      KeyboardAndMouse::VK_MEDIA_STOP => Key::MediaStop,
      KeyboardAndMouse::VK_OEM_MINUS => Key::Minus,
      KeyboardAndMouse::VK_VOLUME_MUTE => Key::VolumeMute,
      //KeyboardAndMouse::VK_MYCOMPUTER          => KeyCode::MyComputer,
      //KeyboardAndMouse::VK_BROWSER_FORWARD     => KeyCode::NavigateForward,
      //KeyboardAndMouse::VK_BROWSER_BACK        => KeyCode::NavigateBackward,
      KeyboardAndMouse::VK_MEDIA_NEXT_TRACK => Key::MediaNextTrack,
      KeyboardAndMouse::VK_NONCONVERT => Key::NoConvert,
      KeyboardAndMouse::VK_OEM_102 => Key::OEM102,
      KeyboardAndMouse::VK_OEM_PERIOD => Key::Period,
      KeyboardAndMouse::VK_MEDIA_PLAY_PAUSE => Key::MediaPlayPause,
      //KeyboardAndMouse::VK_OEM_PLUS            => KeyCode::Plus,
      //KeyboardAndMouse::VK_POWER               => KeyCode::Power,
      KeyboardAndMouse::VK_MEDIA_PREV_TRACK => Key::MediaPrevTrack,
      KeyboardAndMouse::VK_RMENU => Key::RightAlt,
      KeyboardAndMouse::VK_OEM_6 => Key::RightBracket,
      KeyboardAndMouse::VK_RCONTROL => Key::RightControl,
      KeyboardAndMouse::VK_RSHIFT => Key::RightShift,
      KeyboardAndMouse::VK_RWIN => Key::RightSuper,
      KeyboardAndMouse::VK_OEM_1 => Key::Semicolon,
      KeyboardAndMouse::VK_OEM_2 => Key::ForwardSlash,
      KeyboardAndMouse::VK_SLEEP => Key::Sleep,
      //KeyboardAndMouse::VK_STOP                => KeyCode::Stop,
      //KeyboardAndMouse::VK_SYSRQ               => KeyCode::SysRq,
      KeyboardAndMouse::VK_TAB => Key::Tab,
      //KeyboardAndMouse::VK_UNDERLINE           => KeyCode::Underline,
      KeyboardAndMouse::VK_NONAME => Key::NoName,
      KeyboardAndMouse::VK_VOLUME_DOWN => Key::VolumeDown,
      KeyboardAndMouse::VK_VOLUME_UP => Key::VolumeUp,
      //KeyboardAndMouse::VK_WAKE                => KeyCode::Wake,
      KeyboardAndMouse::VK_BROWSER_BACK => Key::WebBack,
      KeyboardAndMouse::VK_BROWSER_FAVORITES => Key::WebFavorites,
      KeyboardAndMouse::VK_BROWSER_FORWARD => Key::WebForward,
      KeyboardAndMouse::VK_BROWSER_HOME => Key::WebHome,
      KeyboardAndMouse::VK_BROWSER_REFRESH => Key::WebRefresh,
      KeyboardAndMouse::VK_BROWSER_SEARCH => Key::WebSearch,
      KeyboardAndMouse::VK_BROWSER_STOP => Key::WebStop,
      //KeyboardAndMouse::VK_YEN                 => KeyCode::Yen,
      KeyboardAndMouse::VK_OEM_COPY => Key::Copy,
      //KeyboardAndMouse::VK_OEM_PASTE           => KeyCode::Paste,
      //KeyboardAndMouse::VK_OEM_CUT             => KeyCode::Cut,
      _ => Key::Unknown,
    }
  }
}

// pub struct KeyboardMessage {

// }
