#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        let hex = hex.strip_prefix('#').ok_or(ColorError::MissingHash)?;
        if hex.len() != 6 {
            return Err(ColorError::InvalidLength);
        }
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidDigit)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidDigit)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidDigit)?;
        Ok(Self { r, g, b })
    }

    pub fn as_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("hex string must start with #")]
    MissingHash,
    #[error("hex string must be 6 hex digits")]
    InvalidLength,
    #[error("hex string contains invalid digit")]
    InvalidDigit,
}

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub divider: Color,
    pub status_ok: Color,
    pub status_warn: Color,
    pub status_err: Color,
}

impl ColorPalette {
    pub fn dark() -> Self {
        Self {
            bg: Color::new(0x0a, 0x0a, 0x0c),
            fg: Color::new(0xe8, 0xe6, 0xe3),
            accent: Color::new(0xf6, 0xf5, 0xf3),
            divider: Color::new(0x1a, 0x1a, 0x1d),
            status_ok: Color::new(0x7f, 0xb0, 0x69),
            status_warn: Color::new(0xd4, 0xa9, 0x6a),
            status_err: Color::new(0xd9, 0x77, 0x66),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_palette_uses_locked_aesthetic() {
        let p = ColorPalette::dark();
        assert_eq!(p.bg.as_hex(), "#0a0a0c");
        assert_eq!(p.fg.as_hex(), "#e8e6e3");
        assert_eq!(p.accent.as_hex(), "#f6f5f3");
        assert_eq!(p.divider.as_hex(), "#1a1a1d");
        assert_eq!(p.status_ok.as_hex(), "#7fb069");
        assert_eq!(p.status_warn.as_hex(), "#d4a96a");
        assert_eq!(p.status_err.as_hex(), "#d97766");
    }

    #[test]
    fn color_from_hex_roundtrips() {
        let c = Color::from_hex("#0a0a0c").unwrap();
        assert_eq!(c.r, 0x0a);
        assert_eq!(c.g, 0x0a);
        assert_eq!(c.b, 0x0c);
        assert_eq!(c.as_hex(), "#0a0a0c");
    }

    #[test]
    fn color_from_hex_rejects_invalid() {
        assert!(Color::from_hex("not-a-color").is_err());
        assert!(Color::from_hex("#xyz").is_err());
        assert!(Color::from_hex("#abc").is_err());
    }
}
