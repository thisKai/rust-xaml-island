use {
    windows::{
        core::{Interface, Result},
        Win32::Foundation::HWND,
        UI::Xaml::{
            Controls::{Button, ContentControl, Grid, Panel, Primitives::ButtonBase, TextBlock},
            FrameworkElement, HorizontalAlignment, RoutedEventHandler,
        },
    },
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::windows::WindowExtWindows,
        window::WindowBuilder,
    },
    xaml_island::XamlIsland,
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let island = XamlIsland::attached(HWND(window.hwnd() as _))?;

    let grid = Grid::new()?;
    let button = Button::new()?;
    let text = TextBlock::new()?;
    text.SetText("button")?;
    button.cast::<ContentControl>()?.SetContent(&text)?;
    button
        .cast::<FrameworkElement>()?
        .SetHorizontalAlignment(HorizontalAlignment::Center)?;
    button
        .cast::<ButtonBase>()?
        .Click(RoutedEventHandler::new(|sender, event| {
            println!("Click");
            Ok(())
        }))?;
    grid.cast::<Panel>()?.Children()?.Append(&button)?;

    island.set_content(&grid)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    let _ = island.resize(new_size.width as _, new_size.height as _);
                }
                _ => (),
            },
            _ => (),
        }
    });
}
