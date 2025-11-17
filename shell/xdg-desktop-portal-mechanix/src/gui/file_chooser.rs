use gpui::*;

pub struct FileChooser {}

impl FileChooser {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(&mut self) {
    }
}

impl Render for FileChooser {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::black())
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .text_color(gpui::white())
            .child("RunningApps")
    }
}
