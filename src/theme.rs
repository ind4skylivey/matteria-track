//! Theme and icons for MatteriaTrack

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MateriaTheme {
    Fire,
    Ice,
    Lightning,
    Earth,
    Wind,
}

impl std::str::FromStr for MateriaTheme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fire" => Ok(Self::Fire),
            "ice" => Ok(Self::Ice),
            "lightning" => Ok(Self::Lightning),
            "earth" => Ok(Self::Earth),
            "wind" => Ok(Self::Wind),
            _ => Err(()),
        }
    }
}

impl MateriaTheme {
    pub fn primary_color(&self) -> (u8, u8, u8) {
        match self {
            Self::Fire => (255, 100, 50),
            Self::Ice => (100, 200, 255),
            Self::Lightning => (255, 255, 100),
            Self::Earth => (139, 90, 43),
            Self::Wind => (150, 255, 150),
        }
    }

    pub fn secondary_color(&self) -> (u8, u8, u8) {
        match self {
            Self::Fire => (255, 50, 0),
            Self::Ice => (50, 150, 255),
            Self::Lightning => (255, 200, 50),
            Self::Earth => (100, 70, 30),
            Self::Wind => (100, 200, 100),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Fire => "🔥",
            Self::Ice => "❄️",
            Self::Lightning => "⚡",
            Self::Earth => "🌍",
            Self::Wind => "🌬️",
        }
    }

    pub fn materia_icon(&self) -> &'static str {
        "💎"
    }
}

pub mod icons {
    pub const MATERIA: &str = "💎";
    pub const SWORD: &str = "⚔️";
    pub const SPARKLE: &str = "✨";
    pub const TROPHY: &str = "🏆";
    pub const STAR: &str = "⭐";
    pub const CLOCK: &str = "⏰";
    pub const CHECK: &str = "✓";
    pub const CROSS: &str = "✗";
    pub const ARROW_RIGHT: &str = "→";
    pub const PROJECT: &str = "";
    pub const TASK: &str = "";
    pub const TIME: &str = "";
    pub const CALENDAR: &str = "";
    pub const GIT: &str = "";
}
