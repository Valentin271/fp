use std::str::FromStr;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeVariant {
    #[default]
    Dark,
    Light,
}

impl FromStr for ThemeVariant {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "light" => Ok(ThemeVariant::Light),
            _ => Ok(ThemeVariant::Dark),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light() {
        assert_eq!(
            ThemeVariant::from_str("light").unwrap(),
            ThemeVariant::Light
        );
        assert_eq!(
            ThemeVariant::from_str("  light").unwrap(),
            ThemeVariant::Light
        );
        assert_eq!(
            ThemeVariant::from_str("light  ").unwrap(),
            ThemeVariant::Light
        );
        assert_eq!(
            ThemeVariant::from_str("LIGHT").unwrap(),
            ThemeVariant::Light
        );
        assert_eq!(
            ThemeVariant::from_str(" LiGhT  ").unwrap(),
            ThemeVariant::Light
        );
    }

    #[test]
    fn dark() {
        assert_eq!(ThemeVariant::from_str("dark").unwrap(), ThemeVariant::Dark);
        assert_eq!(
            ThemeVariant::from_str("  dark  ").unwrap(),
            ThemeVariant::Dark
        );
        assert_eq!(ThemeVariant::from_str("DARK").unwrap(), ThemeVariant::Dark);
        assert_eq!(ThemeVariant::from_str("").unwrap(), ThemeVariant::Dark);
        assert_eq!(
            ThemeVariant::from_str("whatever").unwrap(),
            ThemeVariant::Dark
        );
    }
}
