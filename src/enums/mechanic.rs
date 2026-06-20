use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mechanic {
    None,
    Magicka,
    Stamina,
    EqualMagickaAndStamina,
    Ultimate,
    Health,
    EqualHealthAndMagicka,
    EqualHealthAndStamina,
    WerewolfFury,
}

impl Mechanic {
    pub fn from_id(id: &u8) -> Option<Self> {
        match id {
            0  => Some(Self::None),
            1  => Some(Self::Magicka),
            2  => Some(Self::WerewolfFury),
            4  => Some(Self::Stamina),
            5  => Some(Self::EqualMagickaAndStamina),
            8  => Some(Self::Ultimate),
            32 => Some(Self::Health),
            33 => Some(Self::EqualHealthAndMagicka),
            36 => Some(Self::EqualHealthAndStamina),
            _  => return None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None                  => "None",
            Self::Magicka               => "Magicka",
            Self::Stamina               => "Stamina",
            Self::EqualMagickaAndStamina => "Equal Magicka and Stamina",
            Self::Ultimate              => "Ultimate",
            Self::Health                => "Health",
            Self::EqualHealthAndMagicka => "Equal Health and Magicka",
            Self::EqualHealthAndStamina => "Equal Health and Stamina",
            Self::WerewolfFury          => "Werewolf Fury",
        }
    }
}

impl fmt::Display for Mechanic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}