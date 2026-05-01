#[derive(Debug, Clone)]
pub struct Typography {
    pub mono_family: &'static str,
    pub mono_fallbacks: &'static [&'static str],
    pub size_sm: f32,
    pub size_md: f32,
    pub size_lg: f32,
    pub line_height: f32,
}

impl Typography {
    pub fn default_mono() -> Self {
        Self {
            mono_family: "JetBrains Mono",
            mono_fallbacks: &["Berkeley Mono", "SF Mono", "Menlo", "monospace"],
            size_sm: 12.0,
            size_md: 14.0,
            size_lg: 16.0,
            line_height: 1.4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mono_typography_uses_jetbrains_with_fallbacks() {
        let t = Typography::default_mono();
        assert_eq!(t.mono_family, "JetBrains Mono");
        assert!(t.mono_fallbacks.contains(&"Berkeley Mono"));
        assert!(t.mono_fallbacks.contains(&"monospace"));
        assert_eq!(t.size_md, 14.0);
        assert_eq!(t.line_height, 1.4);
    }
}
