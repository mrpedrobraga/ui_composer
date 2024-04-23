use std::{error::Error, str::Chars};

use crate::renderer::modules::ui::PrimitiveRenderModule;

use super::UIApp;

pub struct UIAppBuilder<TState> {
    initial_state: TState,
    window_title: String,
    window_size: (i32, i32),
}

impl<TState> UIAppBuilder<TState> {
    pub fn new(initial_state: TState) -> Self {
        Self {
            initial_state,
            window_title: "Untitled App".to_owned(),
            window_size: (640, 360),
        }
    }

    /// Changes the window title of the main app window. You can change this later when the app is already running.
    pub fn with_window_title<'a, I: AsRef<str>>(mut self, title: I) -> Self {
        self.window_title.clear();
        self.window_title.push_str(title.as_ref());
        self
    }

    /// Changes the window size of the main app window. You can change this later when the app is already running.
    /// This will have no effect if the window is fullscreen (for example, running in mobile).
    pub fn with_window_size(mut self, size: (i32, i32)) -> Self {
        self.window_size = size;
        self
    }

    /// Builds the UI App.
    pub async fn build(self) -> Result<UIApp<TState>, Box<dyn Error>> {
        let app = UIApp::new(
            self.initial_state,
            super::UIAppCreateDescriptor {
                initial_window_title: self.window_title,
                initial_window_size: self.window_size,
            },
        ).await?;

        Ok(app)
    }
}
