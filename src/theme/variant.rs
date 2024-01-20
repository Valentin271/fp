#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeVariant {
    #[default]
    Dark,
    Light,
}

impl From<&str> for ThemeVariant {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "light" => ThemeVariant::Light,
            _ => ThemeVariant::Dark,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light() {
        assert_eq!(ThemeVariant::from("light"), ThemeVariant::Light);
        assert_eq!(ThemeVariant::from("  light"), ThemeVariant::Light);
        assert_eq!(ThemeVariant::from("light  "), ThemeVariant::Light);
        assert_eq!(ThemeVariant::from("LIGHT"), ThemeVariant::Light);
        assert_eq!(ThemeVariant::from(" LiGhT  "), ThemeVariant::Light);
    }

    #[test]
    fn dark() {
        assert_eq!(ThemeVariant::from("dark"), ThemeVariant::Dark);
        assert_eq!(ThemeVariant::from("  dark  "), ThemeVariant::Dark);
        assert_eq!(ThemeVariant::from("DARK"), ThemeVariant::Dark);
        assert_eq!(ThemeVariant::from(""), ThemeVariant::Dark);
        assert_eq!(ThemeVariant::from("whatever"), ThemeVariant::Dark);
    }
}
