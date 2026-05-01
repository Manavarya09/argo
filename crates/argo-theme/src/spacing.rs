#[derive(Debug, Clone, Copy)]
pub struct Spacing;

impl Spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const TITLEBAR_HEIGHT: f32 = 22.0;
    pub const DIVIDER_WIDTH: f32 = 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_scale_is_consistent() {
        assert_eq!(Spacing::XS, 4.0);
        assert_eq!(Spacing::SM, 8.0);
        assert_eq!(Spacing::MD, 12.0);
        assert_eq!(Spacing::LG, 16.0);
        assert_eq!(Spacing::XL, 24.0);
        assert_eq!(Spacing::TITLEBAR_HEIGHT, 22.0);
        assert_eq!(Spacing::DIVIDER_WIDTH, 1.0);
    }
}
