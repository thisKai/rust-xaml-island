use windows::{
    core::{Interface, IntoParam, Result},
    Win32::{
        Foundation::{BOOL, HWND, RECT},
        System::WinRT::Xaml::IDesktopWindowXamlSourceNative2,
        UI::WindowsAndMessaging::{
            GetClientRect, SetWindowPos, MSG, SET_WINDOW_POS_FLAGS, SWP_SHOWWINDOW,
        },
    },
    UI::Xaml::{Hosting::DesktopWindowXamlSource, UIElement},
};

#[repr(transparent)]
pub struct XamlIsland {
    source: DesktopWindowXamlSource,
}
impl XamlIsland {
    pub fn new() -> Result<Self> {
        let source = DesktopWindowXamlSource::new()?;

        Ok(XamlIsland { source })
    }
    pub fn attach(&self, hwnd: HWND) -> Result<()> {
        let interop: IDesktopWindowXamlSourceNative2 = self.source.cast()?;
        unsafe { interop.AttachToWindow(hwnd) }?;

        self.fill_window(hwnd)?;

        Ok(())
    }
    pub fn attached(hwnd: HWND) -> Result<Self> {
        let island = Self::new()?;
        island.attach(hwnd)?;

        Ok(island)
    }
    pub fn set_position(&self, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        self.set_position_with_flags(x, y, width, height, SWP_SHOWWINDOW)
    }
    pub fn set_position_with_flags(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flags: SET_WINDOW_POS_FLAGS,
    ) -> Result<()> {
        let source: IDesktopWindowXamlSourceNative2 = self.source.cast()?;
        unsafe {
            let hwnd = source.WindowHandle()?;
            SetWindowPos(hwnd, HWND::default(), x, y, width, height, flags).ok()
        }
    }
    pub fn resize(&self, width: i32, height: i32) -> Result<()> {
        self.set_position(0, 0, width, height)
    }
    pub fn fill_window(&self, hwnd: HWND) -> Result<()> {
        let (width, height) = inner_size(hwnd)?;
        self.resize(width, height)?;

        Ok(())
    }
    pub fn set_content<'a>(&self, value: impl IntoParam<'a, UIElement>) -> Result<()> {
        self.source.SetContent(value)
    }
    pub fn pre_translate_message(&self, message: *const MSG, result: *mut BOOL) -> Result<()> {
        let source: IDesktopWindowXamlSourceNative2 = self.source.cast()?;
        unsafe { source.PreTranslateMessage(message, result) }
    }
}

fn inner_size(hwnd: HWND) -> Result<(i32, i32)> {
    let mut rect = RECT::default();
    unsafe {
        GetClientRect(hwnd, &mut rect).ok()?;
    }
    Ok((rect.right, rect.bottom))
}
