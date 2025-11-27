//! Materia theme definitions for MateriaTrack
//!
//! Six elemental themes inspired by Final Fantasy's Materia system.

use super::{ColorPalette, IconSet, Theme};

pub static THEMES: &[MateriaTheme] = &[
    MateriaTheme::Fire,
    MateriaTheme::Ice,
    MateriaTheme::Lightning,
    MateriaTheme::Earth,
    MateriaTheme::Wind,
    MateriaTheme::Bahamut,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MateriaTheme {
    Fire,
    Ice,
    Lightning,
    Earth,
    Wind,
    Bahamut,
}

impl MateriaTheme {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "fire" | "red" | "ifrit" => Self::Fire,
            "ice" | "blue" | "shiva" => Self::Ice,
            "lightning" | "yellow" | "ramuh" | "thunder" => Self::Lightning,
            "earth" | "green" | "titan" => Self::Earth,
            "wind" | "white" | "garuda" => Self::Wind,
            "bahamut" | "dark" | "purple" | "dragon" => Self::Bahamut,
            _ => Self::Fire,
        }
    }

    const fn fire_palette() -> ColorPalette {
        ColorPalette {
            primary: (255, 69, 0),     // #FF4500 OrangeRed
            secondary: (255, 99, 71),  // #FF6347 Tomato
            accent: (255, 165, 0),     // #FFA500 Orange
            text: (255, 255, 255),
            muted: (180, 100, 80),
            success: (100, 255, 100),
            warning: (255, 200, 50),
            error: (255, 50, 50),
        }
    }

    const fn ice_palette() -> ColorPalette {
        ColorPalette {
            primary: (0, 206, 209),    // #00CED1 DarkTurquoise
            secondary: (70, 130, 180), // #4682B4 SteelBlue
            accent: (135, 206, 235),   // #87CEEB SkyBlue
            text: (255, 255, 255),
            muted: (100, 150, 180),
            success: (100, 255, 200),
            warning: (200, 200, 100),
            error: (255, 100, 150),
        }
    }

    const fn lightning_palette() -> ColorPalette {
        ColorPalette {
            primary: (255, 215, 0),    // #FFD700 Gold
            secondary: (147, 112, 219), // #9370DB MediumPurple
            accent: (186, 85, 211),    // #BA55D3 MediumOrchid
            text: (255, 255, 255),
            muted: (180, 160, 100),
            success: (200, 255, 100),
            warning: (255, 230, 100),
            error: (255, 100, 100),
        }
    }

    const fn earth_palette() -> ColorPalette {
        ColorPalette {
            primary: (34, 139, 34),    // #228B22 ForestGreen
            secondary: (139, 69, 19),  // #8B4513 SaddleBrown
            accent: (107, 142, 35),    // #6B8E23 OliveDrab
            text: (255, 255, 255),
            muted: (100, 120, 80),
            success: (150, 255, 100),
            warning: (200, 180, 80),
            error: (200, 80, 80),
        }
    }

    const fn wind_palette() -> ColorPalette {
        ColorPalette {
            primary: (240, 248, 255),  // #F0F8FF AliceBlue
            secondary: (211, 211, 211), // #D3D3D3 LightGray
            accent: (112, 128, 144),   // #708090 SlateGray
            text: (50, 50, 50),
            muted: (150, 150, 150),
            success: (100, 200, 100),
            warning: (200, 180, 100),
            error: (200, 100, 100),
        }
    }

    const fn bahamut_palette() -> ColorPalette {
        ColorPalette {
            primary: (46, 8, 84),      // #2E0854 Dark Purple
            secondary: (255, 215, 0),  // #FFD700 Gold
            accent: (75, 0, 130),      // #4B0082 Indigo
            text: (255, 255, 255),
            muted: (100, 80, 120),
            success: (150, 200, 255),
            warning: (255, 215, 100),
            error: (255, 80, 120),
        }
    }

    fn fire_icons() -> IconSet {
        IconSet {
            materia: "🔴",
            fire: "🔥",
            star: "⭐",
            ..IconSet::default()
        }
    }

    fn ice_icons() -> IconSet {
        IconSet {
            materia: "🔵",
            fire: "❄️",
            star: "💠",
            ..IconSet::default()
        }
    }

    fn lightning_icons() -> IconSet {
        IconSet {
            materia: "🟡",
            fire: "⚡",
            star: "✨",
            ..IconSet::default()
        }
    }

    fn earth_icons() -> IconSet {
        IconSet {
            materia: "🟢",
            fire: "🌿",
            star: "🌱",
            ..IconSet::default()
        }
    }

    fn wind_icons() -> IconSet {
        IconSet {
            materia: "⚪",
            fire: "🌬️",
            star: "☁️",
            ..IconSet::default()
        }
    }

    fn bahamut_icons() -> IconSet {
        IconSet {
            materia: "🟣",
            fire: "🐉",
            star: "👑",
            trophy: "🏆",
            ..IconSet::default()
        }
    }
}

impl Theme for MateriaTheme {
    fn name(&self) -> &'static str {
        match self {
            Self::Fire => "Fire",
            Self::Ice => "Ice",
            Self::Lightning => "Lightning",
            Self::Earth => "Earth",
            Self::Wind => "Wind",
            Self::Bahamut => "Bahamut",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Fire => "Burning passion and fierce determination",
            Self::Ice => "Cool precision and crystalline clarity",
            Self::Lightning => "Electric energy and swift action",
            Self::Earth => "Grounded strength and natural growth",
            Self::Wind => "Ethereal freedom and endless possibility",
            Self::Bahamut => "Ultimate power and legendary mastery",
        }
    }

    fn palette(&self) -> &ColorPalette {
        match self {
            Self::Fire => &FIRE_PALETTE,
            Self::Ice => &ICE_PALETTE,
            Self::Lightning => &LIGHTNING_PALETTE,
            Self::Earth => &EARTH_PALETTE,
            Self::Wind => &WIND_PALETTE,
            Self::Bahamut => &BAHAMUT_PALETTE,
        }
    }

    fn icons(&self) -> &IconSet {
        match self {
            Self::Fire => &FIRE_ICONS,
            Self::Ice => &ICE_ICONS,
            Self::Lightning => &LIGHTNING_ICONS,
            Self::Earth => &EARTH_ICONS,
            Self::Wind => &WIND_ICONS,
            Self::Bahamut => &BAHAMUT_ICONS,
        }
    }

    fn element_icon(&self) -> &'static str {
        match self {
            Self::Fire => "🔥",
            Self::Ice => "❄️",
            Self::Lightning => "⚡",
            Self::Earth => "🌍",
            Self::Wind => "🌬️",
            Self::Bahamut => "🐉",
        }
    }
}

static FIRE_PALETTE: ColorPalette = MateriaTheme::fire_palette();
static ICE_PALETTE: ColorPalette = MateriaTheme::ice_palette();
static LIGHTNING_PALETTE: ColorPalette = MateriaTheme::lightning_palette();
static EARTH_PALETTE: ColorPalette = MateriaTheme::earth_palette();
static WIND_PALETTE: ColorPalette = MateriaTheme::wind_palette();
static BAHAMUT_PALETTE: ColorPalette = MateriaTheme::bahamut_palette();

lazy_static::lazy_static! {
    static ref FIRE_ICONS: IconSet = MateriaTheme::fire_icons();
    static ref ICE_ICONS: IconSet = MateriaTheme::ice_icons();
    static ref LIGHTNING_ICONS: IconSet = MateriaTheme::lightning_icons();
    static ref EARTH_ICONS: IconSet = MateriaTheme::earth_icons();
    static ref WIND_ICONS: IconSet = MateriaTheme::wind_icons();
    static ref BAHAMUT_ICONS: IconSet = MateriaTheme::bahamut_icons();
}

impl std::str::FromStr for MateriaTheme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_name(s))
    }
}

impl std::fmt::Display for MateriaTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_from_name() {
        assert_eq!(MateriaTheme::from_name("fire"), MateriaTheme::Fire);
        assert_eq!(MateriaTheme::from_name("ICE"), MateriaTheme::Ice);
        assert_eq!(MateriaTheme::from_name("shiva"), MateriaTheme::Ice);
        assert_eq!(MateriaTheme::from_name("bahamut"), MateriaTheme::Bahamut);
        assert_eq!(MateriaTheme::from_name("unknown"), MateriaTheme::Fire);
    }

    #[test]
    fn test_theme_names() {
        assert_eq!(MateriaTheme::Fire.name(), "Fire");
        assert_eq!(MateriaTheme::Bahamut.name(), "Bahamut");
    }

    #[test]
    fn test_theme_palette() {
        let fire = MateriaTheme::Fire;
        let palette = fire.palette();
        assert_eq!(palette.primary, (255, 69, 0));
    }

    #[test]
    fn test_theme_icons() {
        let ice = MateriaTheme::Ice;
        let icons = ice.icons();
        assert_eq!(icons.fire, "❄️");
    }

    #[test]
    fn test_all_themes_exist() {
        assert_eq!(THEMES.len(), 6);
    }

    #[test]
    fn test_theme_display() {
        assert_eq!(format!("{}", MateriaTheme::Lightning), "Lightning");
    }
}
