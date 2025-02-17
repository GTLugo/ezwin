use std::{
  ops::BitAnd,
  sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock,
  },
};

use cursor_icon::CursorIcon;
use windows::{
  core::{PCSTR, PCWSTR},
  Win32::{
    Devices::HumanInterfaceDevice,
    Foundation::{HWND, NTSTATUS, RECT},
    Graphics::Gdi::{GetDC, GetMonitorInfoW, HMONITOR, MONITORINFO, MONITORINFOEXW},
    System::{
      LibraryLoader::{GetProcAddress, LoadLibraryA},
      SystemInformation::OSVERSIONINFOW,
    },
    UI::{
      HiDpi::{self, GetDpiForMonitor, GetDpiForWindow},
      Input::{
        self,
        GetRawInputData,
        RegisterRawInputDevices,
        HRAWINPUT,
        RAWINPUT,
        RAWINPUTDEVICE,
        RAWINPUTHEADER,
      },
      WindowsAndMessaging::{
        self,
        ClipCursor,
        GetClipCursor,
        GetSystemMetrics,
        ShowCursor,
        WINDOW_EX_STYLE,
        WINDOW_STYLE,
      },
    },
  },
  UI::ViewManagement::{UIColorType, UISettings},
};

use crate::{
  error::WindowError,
  prelude::{PhysicalPosition, PhysicalSize},
  window::{
    data::{Fullscreen, Visibility},
    frame::Style,
  },
};

pub fn signed_lo_word(dword: i32) -> i16 {
  dword as i16
}

pub fn lo_word(dword: u32) -> u16 {
  dword as u16
}

pub fn signed_hi_word(dword: i32) -> i16 {
  (dword >> 16) as i16
}

pub fn hi_word(dword: u32) -> u16 {
  (dword >> 16) as u16
}

pub fn signed_lo_byte(word: i16) -> i8 {
  word as i8
}

pub fn lo_byte(word: u16) -> u8 {
  word as u8
}

pub fn signed_hi_byte(word: i16) -> i8 {
  (word >> 8) as i8
}

pub fn hi_byte(word: u16) -> u8 {
  (word >> 8) as u8
}

/*
  Some of the following code was taken directly from `winit` and is currently under the Apache-2.0 copyright.
  > https://github.com/rust-windowing/winit/blob/master/src/platform_impl/windows/
  Some functions are simplified to be more specific to the goals of ezwin or reduce dependencies.
  Changes were also made to adapt from the crate `windows-sys` to `windows`

  The dark mode algorithm was NOT taken from `winit`, but instead from here:
  > https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/apply-windows-themes
*/

pub(crate) fn get_function_impl(
  library: &str,
  function: &str,
) -> Option<*const std::ffi::c_void> {
  assert_eq!(library.chars().last(), Some('\0'));
  assert_eq!(function.chars().last(), Some('\0'));

  // Library names we will use are ASCII so we can use the A version to avoid
  // string conversion.
  let module = match unsafe { LoadLibraryA(PCSTR::from_raw(library.as_ptr())) } {
    Ok(module) => module,
    Err(_) => return None,
  };

  unsafe { GetProcAddress(module, PCSTR::from_raw(function.as_ptr())) }
    .map(|function_ptr| function_ptr as _)
}

macro_rules! get_function {
  ($lib:expr, $func:ident) => {
    crate::utilities::get_function_impl(
      concat!($lib, '\0'),
      concat!(stringify!($func), '\0'),
    )
    .map(|f| unsafe { std::mem::transmute::<*const _, $func>(f) })
  };
}

pub fn windows_10_build_version() -> Option<u32> {
  static WIN10_BUILD_VERSION: OnceLock<Option<u32>> = OnceLock::new();
  *WIN10_BUILD_VERSION.get_or_init(|| {
    type RtlGetVersion = unsafe extern "system" fn(*mut OSVERSIONINFOW) -> NTSTATUS;
    let handle = get_function!("ntdll.dll", RtlGetVersion);

    if let Some(rtl_get_version) = handle {
      unsafe {
        let mut vi = OSVERSIONINFOW {
          dwOSVersionInfoSize: 0,
          dwMajorVersion: 0,
          dwMinorVersion: 0,
          dwBuildNumber: 0,
          dwPlatformId: 0,
          szCSDVersion: [0; 128],
        };

        let status = (rtl_get_version)(&mut vi);

        if status.0 >= 0 && vi.dwMajorVersion == 10 && vi.dwMinorVersion == 0 {
          Some(vi.dwBuildNumber)
        } else {
          None
        }
      }
    } else {
      None
    }
  })
}

pub fn is_dark_mode_supported() -> bool {
  static DARK_MODE_SUPPORTED: OnceLock<bool> = OnceLock::new();
  *DARK_MODE_SUPPORTED.get_or_init(|| {
    // We won't try to do anything for windows versions < 17763
    // (Windows 10 October 2018 update)
    match windows_10_build_version() {
      Some(v) => v >= 17763,
      None => false,
    }
  })
}

pub fn is_system_dark_mode_enabled() -> bool {
  static IS_SYSTEM_DARK_MODE: OnceLock<bool> = OnceLock::new();
  *IS_SYSTEM_DARK_MODE.get_or_init(|| {
    let settings = UISettings::new().unwrap();
    let foreground = settings
      .GetColorValue(UIColorType::Foreground)
      .unwrap_or_default();
    is_color_light(&foreground)
  })
}

#[inline]
fn is_color_light(clr: &windows::UI::Color) -> bool {
  ((5 * clr.G as u32) + (2 * clr.R as u32) + clr.B as u32) > (8 * 128)
}

pub(crate) fn get_window_style(info: &Style) -> WINDOW_STYLE {
  let mut style = WindowsAndMessaging::WS_CAPTION
    | WindowsAndMessaging::WS_BORDER
    | WindowsAndMessaging::WS_CLIPSIBLINGS
    | WindowsAndMessaging::WS_SYSMENU;

  if info.resizeable {
    style |= WindowsAndMessaging::WS_SIZEBOX;
    style |= WindowsAndMessaging::WS_MAXIMIZEBOX;
    style |= WindowsAndMessaging::WS_MINIMIZEBOX;
  }

  if let Visibility::Shown = info.visibility {
    style |= WindowsAndMessaging::WS_VISIBLE;
  }

  if let Some(Fullscreen::Borderless) = info.fullscreen {
    style &= !WindowsAndMessaging::WS_OVERLAPPEDWINDOW;
    style |= WindowsAndMessaging::WS_POPUP;
  }

  if let Visibility::Hidden = info.decorations {
    style &= !(WindowsAndMessaging::WS_CAPTION | WindowsAndMessaging::WS_BORDER);
  }

  style
}

pub(crate) fn get_window_ex_style(info: &Style) -> WINDOW_EX_STYLE {
  let mut style =
    WindowsAndMessaging::WS_EX_WINDOWEDGE | WindowsAndMessaging::WS_EX_APPWINDOW;

  if let Some(Fullscreen::Borderless) = info.fullscreen {
    style &= !WindowsAndMessaging::WS_EX_OVERLAPPEDWINDOW;
  }

  if let Visibility::Hidden = info.decorations {
    style &= !WindowsAndMessaging::WS_EX_WINDOWEDGE;
  }

  style
}

pub(crate) fn set_cursor_clip(rect: Option<&RECT>) {
  if let Err(_e) = unsafe { ClipCursor(rect.map(|r| r as _)) } {
    tracing::error!("{_e}");
  }
}

pub fn get_cursor_clip() -> Result<RECT, WindowError> {
  unsafe {
    let mut rect = RECT::default();
    Ok(GetClipCursor(&mut rect).map(|_| rect)?)
  }
}

pub fn get_desktop_rect() -> RECT {
  unsafe {
    let left = GetSystemMetrics(WindowsAndMessaging::SM_XVIRTUALSCREEN);
    let top = GetSystemMetrics(WindowsAndMessaging::SM_YVIRTUALSCREEN);
    RECT {
      left,
      top,
      right: left + GetSystemMetrics(WindowsAndMessaging::SM_CXVIRTUALSCREEN),
      bottom: top + GetSystemMetrics(WindowsAndMessaging::SM_CYVIRTUALSCREEN),
    }
  }
}

pub(crate) fn set_cursor_visibility(visible: Visibility) {
  let hidden = visible == Visibility::Hidden;
  static HIDDEN: AtomicBool = AtomicBool::new(false);
  let changed = HIDDEN.swap(hidden, Ordering::SeqCst) ^ hidden;
  if changed {
    unsafe { ShowCursor(!hidden) };
  }
}

pub const BASE_DPI: u32 = 96;

pub fn dpi_to_scale_factor(dpi: u32) -> f64 {
  dpi as f64 / BASE_DPI as f64
}

pub fn hwnd_dpi(hwnd: HWND) -> u32 {
  let hdc = unsafe { GetDC(hwnd) };
  if hdc.is_invalid() {
    panic!("device context was invalid");
  }

  match unsafe { GetDpiForWindow(hwnd) } {
    0 => BASE_DPI, // 0 is returned if hwnd is invalid
    dpi => dpi,
  }
}

pub fn register_all_mice_and_keyboards_for_raw_input(hwnd: HWND) -> bool {
  // RIDEV_DEVNOTIFY: receive hotplug events
  // RIDEV_INPUTSINK: receive events even if we're not in the foreground
  // RIDEV_REMOVE: don't receive device events (requires NULL hwndTarget)
  let flags = Input::RIDEV_DEVNOTIFY;

  let devices: [RAWINPUTDEVICE; 2] = [
    RAWINPUTDEVICE {
      usUsagePage: HumanInterfaceDevice::HID_USAGE_PAGE_GENERIC,
      usUsage: HumanInterfaceDevice::HID_USAGE_GENERIC_MOUSE,
      dwFlags: flags,
      hwndTarget: hwnd,
    },
    RAWINPUTDEVICE {
      usUsagePage: HumanInterfaceDevice::HID_USAGE_PAGE_GENERIC,
      usUsage: HumanInterfaceDevice::HID_USAGE_GENERIC_KEYBOARD,
      dwFlags: flags,
      hwndTarget: hwnd,
    },
  ];

  register_raw_input_devices(&devices)
}

pub fn register_raw_input_devices(devices: &[RAWINPUTDEVICE]) -> bool {
  let device_size = std::mem::size_of::<RAWINPUTDEVICE>() as u32;

  unsafe { RegisterRawInputDevices(devices, device_size) }.is_err()
}

pub fn read_raw_input(handle: HRAWINPUT) -> Option<RAWINPUT> {
  let mut data = RAWINPUT::default();
  let mut data_size = std::mem::size_of::<RAWINPUT>() as u32;
  let header_size = std::mem::size_of::<RAWINPUTHEADER>() as u32;

  let status = unsafe {
    GetRawInputData(
      handle,
      Input::RID_INPUT,
      Some(&mut data as *mut _ as _),
      &mut data_size,
      header_size,
    )
  };

  if status == u32::MAX || status == 0 {
    return None;
  }

  Some(data)
}

pub fn is_flag_set<T: Copy + BitAnd<T, Output = T> + PartialEq<T>>(
  var: T,
  flag: T,
) -> bool {
  (var & flag) == flag
}

pub struct Monitor {
  hmonitor: HMONITOR,
}

impl Monitor {
  pub fn new(hmonitor: HMONITOR) -> Self {
    Self { hmonitor }
  }

  fn monitor_info(&self) -> Option<MONITORINFOEXW> {
    let mut monitor_info: MONITORINFOEXW = unsafe { std::mem::zeroed() };
    monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    let status = unsafe {
      GetMonitorInfoW(
        self.hmonitor,
        &mut monitor_info as *mut MONITORINFOEXW as *mut MONITORINFO,
      )
    };

    if status.as_bool() {
      Some(monitor_info)
    } else {
      None
    }
  }

  pub fn position(&self) -> PhysicalPosition {
    let info = self.monitor_info();
    info
      .map(|info| {
        let rect = info.monitorInfo.rcMonitor;
        PhysicalPosition {
          x: rect.left,
          y: rect.top,
        }
      })
      .unwrap_or_default()
  }

  pub fn size(&self) -> PhysicalSize {
    let info = self.monitor_info();
    info
      .map(|info| {
        let rect = info.monitorInfo.rcMonitor;
        PhysicalSize {
          width: (rect.right - rect.left) as u32,
          height: (rect.bottom - rect.top) as u32,
        }
      })
      .unwrap_or_default()
  }

  pub fn scale_factor(&self) -> f64 {
    let mut dpi_x = 0;
    let mut _dpi_y = 0;
    unsafe {
      GetDpiForMonitor(self.hmonitor, HiDpi::MDT_EFFECTIVE_DPI, &mut dpi_x, &mut _dpi_y)
    }
    .unwrap();

    dpi_to_scale_factor(dpi_x)
  }
}

pub(crate) fn to_windows_cursor(cursor: CursorIcon) -> PCWSTR {
  match cursor {
    CursorIcon::Default => WindowsAndMessaging::IDC_ARROW,
    CursorIcon::Pointer => WindowsAndMessaging::IDC_HAND,
    CursorIcon::Crosshair => WindowsAndMessaging::IDC_CROSS,
    CursorIcon::Text | CursorIcon::VerticalText => WindowsAndMessaging::IDC_IBEAM,
    CursorIcon::NotAllowed | CursorIcon::NoDrop => WindowsAndMessaging::IDC_NO,
    CursorIcon::Grab
    | CursorIcon::Grabbing
    | CursorIcon::Move
    | CursorIcon::AllScroll => WindowsAndMessaging::IDC_SIZEALL,
    CursorIcon::EResize
    | CursorIcon::WResize
    | CursorIcon::EwResize
    | CursorIcon::ColResize => WindowsAndMessaging::IDC_SIZEWE,
    CursorIcon::NResize
    | CursorIcon::SResize
    | CursorIcon::NsResize
    | CursorIcon::RowResize => WindowsAndMessaging::IDC_SIZENS,
    CursorIcon::NeResize | CursorIcon::SwResize | CursorIcon::NeswResize => {
      WindowsAndMessaging::IDC_SIZENESW
    }
    CursorIcon::NwResize | CursorIcon::SeResize | CursorIcon::NwseResize => {
      WindowsAndMessaging::IDC_SIZENWSE
    }
    CursorIcon::Wait => WindowsAndMessaging::IDC_WAIT,
    CursorIcon::Progress => WindowsAndMessaging::IDC_APPSTARTING,
    CursorIcon::Help => WindowsAndMessaging::IDC_HELP,
    _ => WindowsAndMessaging::IDC_ARROW, // use arrow for the missing cases.
  }
}
