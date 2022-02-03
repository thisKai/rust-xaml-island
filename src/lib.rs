use windows::{
    core::{Interface, IntoParam, Result},
    Win32::{
        Foundation::{HWND, RECT},
        System::WinRT::Xaml::IDesktopWindowXamlSourceNative,
        UI::WindowsAndMessaging::{GetClientRect, SetWindowPos, SWP_SHOWWINDOW},
    },
    UI::Xaml::{Hosting::DesktopWindowXamlSource, UIElement},
};

pub struct XamlIsland {
    source: DesktopWindowXamlSource,
}
impl XamlIsland {
    pub fn new() -> Result<Self> {
        let source = DesktopWindowXamlSource::new()?;

        Ok(XamlIsland { source })
    }
    pub fn attach(&self, hwnd: HWND) -> Result<()> {
        let interop: IDesktopWindowXamlSourceNative = self.source.cast()?;
        unsafe { interop.AttachToWindow(hwnd) }?;

        self.fill_window(hwnd)?;

        Ok(())
    }
    pub fn attached(hwnd: HWND) -> Result<Self> {
        let island = Self::new()?;
        island.attach(hwnd)?;

        Ok(island)
    }
    pub fn resize(&self, width: i32, height: i32) -> Result<()> {
        let source: IDesktopWindowXamlSourceNative = self.source.cast()?;
        let hwnd = unsafe { source.WindowHandle() }?;

        unsafe { SetWindowPos(hwnd, HWND::default(), 0, 0, width, height, SWP_SHOWWINDOW).ok() }
    }
    pub fn fill_window(&self, hwnd: HWND) -> Result<()> {
        let (width, height) = inner_size(hwnd)?;
        self.resize(width, height)?;

        Ok(())
    }
    pub fn set_content<'a>(&self, value: impl IntoParam<'a, UIElement>) -> Result<()> {
        self.source.SetContent(value)
    }
}

fn inner_size(hwnd: HWND) -> Result<(i32, i32)> {
    let mut rect = RECT::default();
    unsafe {
        GetClientRect(hwnd, &mut rect).ok()?;
    }
    Ok((rect.right, rect.bottom))
}
