use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageType {
    Generic,
    Physical,
    Flame,
    Shock,
    Oblivion,
    Frost,
    Earth,
    Magic,
    Drown,
    Disease,
    Poison,
    Bleed,
}

impl DamageType {
    pub fn from_id(id: &u16) -> Option<Self> {
        match id {
            1  => Some(Self::Generic),
            2  => Some(Self::Physical),
            3  => Some(Self::Flame),
            4  => Some(Self::Shock),
            5  => Some(Self::Oblivion),
            6  => Some(Self::Frost),
            7  => Some(Self::Earth),
            8  => Some(Self::Magic),
            9  => Some(Self::Drown),
            10 => Some(Self::Disease),
            11 => Some(Self::Poison),
            12 => Some(Self::Bleed),
            _  => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Generic  => "Generic",
            Self::Physical => "Physical",
            Self::Flame    => "Flame",
            Self::Shock    => "Shock",
            Self::Oblivion => "Oblivion",
            Self::Frost    => "Frost",
            Self::Earth    => "Earth",
            Self::Magic    => "Magic",
            Self::Drown    => "Drown",
            Self::Disease  => "Disease",
            Self::Poison   => "Poison",
            Self::Bleed    => "Bleed",
        }
    }
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}