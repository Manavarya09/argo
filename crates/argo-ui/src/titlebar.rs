use argo_theme::{ColorPalette, Spacing, Typography};
use gpui::prelude::*;
use gpui::{IntoElement, RenderOnce, div, px, rgb};

#[derive(Clone, IntoElement)]
pub struct PaneTitleBar {
    pub provider: String,
    pub model: String,
    pub cwd: String,
    pub status: PaneStatus,
}

#[derive(Clone, Copy)]
pub enum PaneStatus {
    Ready,
    Running,
    RateLimited,
    Errored,
}

impl PaneStatus {
    fn dot_color(&self, p: &ColorPalette) -> u32 {
        let c = match self {
            Self::Ready => p.status_ok,
            Self::Running => p.accent,
            Self::RateLimited => p.status_warn,
            Self::Errored => p.status_err,
        };
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32)
    }
}

impl RenderOnce for PaneTitleBar {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        let palette = ColorPalette::dark();
        let typography = Typography::default_mono();
        let bg = ((palette.bg.r as u32) << 16) | ((palette.bg.g as u32) << 8) | palette.bg.b as u32;
        let fg = ((palette.fg.r as u32) << 16) | ((palette.fg.g as u32) << 8) | palette.fg.b as u32;
        let divider = ((palette.divider.r as u32) << 16)
            | ((palette.divider.g as u32) << 8)
            | palette.divider.b as u32;
        let status = self.status.dot_color(&palette);

        let label = format!("{} · {} · {}", self.provider, self.model, self.cwd);

        div()
            .w_full()
            .h(px(Spacing::TITLEBAR_HEIGHT))
            .bg(rgb(bg))
            .border_b_1()
            .border_color(rgb(divider))
            .px(px(Spacing::SM))
            .flex()
            .items_center()
            .gap(px(Spacing::SM))
            .child(
                div()
                    .w(px(6.))
                    .h(px(6.))
                    .rounded_full()
                    .bg(rgb(status)),
            )
            .child(
                div()
                    .text_color(rgb(fg))
                    .font_family(typography.mono_family)
                    .text_size(px(typography.size_sm))
                    .child(label),
            )
    }
}
