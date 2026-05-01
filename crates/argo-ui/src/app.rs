use argo_theme::ColorPalette;
use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, point, prelude::*,
    px, rgb, size,
};

pub struct ArgoApp {
    palette: ColorPalette,
}

impl ArgoApp {
    pub fn new() -> Self {
        Self {
            palette: ColorPalette::dark(),
        }
    }

    pub fn launch() {
        Application::new().run(|cx: &mut App| {
            let bounds = Bounds::new(point(px(200.), px(100.)), size(px(1280.), px(800.)));
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ArgoApp::new()),
            )
            .unwrap();
            cx.activate(true);
        });
    }

    fn bg_rgb(&self) -> u32 {
        let c = self.palette.bg;
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32)
    }
}

impl Default for ArgoApp {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ArgoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w_full().h_full().bg(rgb(self.bg_rgb()))
    }
}
