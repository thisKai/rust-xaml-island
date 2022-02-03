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
    island_hwnd: HWND,
    source: DesktopWindowXamlSource,
}
impl XamlIsland {
    pub fn attach(hwnd: HWND) -> Result<Self> {
        let (width, height) = inner_size(hwnd)?;

        let source = DesktopWindowXamlSource::new()?;
        let interop: IDesktopWindowXamlSourceNative = source.cast()?;
        unsafe {
            interop.AttachToWindow(hwnd)?;
        }
        let island_hwnd = unsafe { interop.WindowHandle() }?;

        let island = XamlIsland { island_hwnd, source };
        island.resize(width, height)?;

        Ok(island)
    }
    pub fn resize(&self, width: i32, height: i32) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.island_hwnd,
                HWND::default(),
                0,
                0,
                width,
                height,
                SWP_SHOWWINDOW,
            )
            .ok()
        }
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
