use gpui::{px, size, AppContext, Bounds, WindowBounds, WindowOptions};
use crate::gui::file_chooser::FileChooser;
use crate::interfaces::file_chooser::FileChooserOptions;

#[derive(Debug, Clone)]
pub enum Message {
    FileChooserRequested(FileChooserOptions),
}

pub fn handle_message(msg: Message) {
    match msg {
        Message::FileChooserRequested(opts) => {
            let application = gpui::Application::new();
            application.run(|cx| {
                let window_bounds = WindowBounds::Windowed(
                    Bounds::centered(None, size(px(540.0), px(620.0)), cx)
                );

                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(window_bounds),
                        ..Default::default()
                    },
                    |_window, cx| cx.new(|_cx| FileChooser::new())
                ).unwrap();
                cx.activate(true);
            });       
        }
    }
}