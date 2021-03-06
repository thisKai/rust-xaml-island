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
    /// Create an empty xaml island.
    pub fn new() -> Result<Self> {
        let source = DesktopWindowXamlSource::new()?;

        Ok(XamlIsland { source })
    }
    /// Create a xaml island, attach it to a window and resize it to fill the window.
    pub unsafe fn attached(hwnd: HWND) -> Result<Self> {
        let island = Self::new()?;
        island.attach(hwnd)?;
        island.fill_window(hwnd)?;

        Ok(island)
    }
    /// Attach this xaml island to a window.
    pub unsafe fn attach(&self, hwnd: HWND) -> Result<()> {
        let interop: IDesktopWindowXamlSourceNative2 = self.source.cast()?;
        interop.AttachToWindow(hwnd)
    }
    /// Get the underlying DesktopWindowXamlSource object.
    pub fn source(&self) -> &DesktopWindowXamlSource {
        &self.source
    }
    /// Get the window handle of this xaml island.
    pub unsafe fn hwnd(&self) -> Result<HWND> {
        let source: IDesktopWindowXamlSourceNative2 = self.source.cast()?;
        source.WindowHandle()
    }
    /// Set the position of this xaml island in its parent window.
    pub unsafe fn set_position(&self, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        self.set_position_with_flags(x, y, width, height, SWP_SHOWWINDOW)
    }
    /// Set the position of this xaml island in its parent window (with flags).
    pub unsafe fn set_position_with_flags(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flags: SET_WINDOW_POS_FLAGS,
    ) -> Result<()> {
        let source: IDesktopWindowXamlSourceNative2 = self.source.cast()?;

        let hwnd = source.WindowHandle()?;
        SetWindowPos(hwnd, HWND::default(), x, y, width, height, flags).ok()
    }
    pub unsafe fn resize(&self, width: i32, height: i32) -> Result<()> {
        self.set_position(0, 0, width, height)
    }
    /// Resize this xaml island to to fill the window.
    pub unsafe fn fill_window(&self, hwnd: HWND) -> Result<()> {
        let (width, height) = inner_size(hwnd)?;
        self.resize(width, height)
    }
    /// Set xaml content for this xaml island
    pub fn set_content<'a>(&self, value: impl IntoParam<'a, UIElement>) -> Result<()> {
        self.source.SetContent(value)
    }
    /// Enables the WinRT XAML framework to process a Windows message for a DesktopWindowXamlSource object that hosts a WinRT XAML control.
    pub unsafe fn pre_translate_message(
        &self,
        message: *const MSG,
        result: *mut BOOL,
    ) -> Result<()> {
        let source: IDesktopWindowXamlSourceNative2 = self.source.cast()?;
        source.PreTranslateMessage(message, result)
    }
}

unsafe fn inner_size(hwnd: HWND) -> Result<(i32, i32)> {
    let mut rect = RECT::default();

    GetClientRect(hwnd, &mut rect).ok()?;

    Ok((rect.right, rect.bottom))
}
