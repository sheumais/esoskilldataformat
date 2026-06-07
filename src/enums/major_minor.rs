use std::fmt;

macro_rules! major_minor_buffs {
    (
        $(
            $variant:ident = $id:expr => $name:expr => $tooltip:expr
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MajorMinorBuff {
            $(
                $variant,
            )*
        }

        impl MajorMinorBuff {
            pub fn from_id(id: &u32) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        $name => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn to_id(&self) -> u32 {
                match self {
                    $(
                        Self::$variant => $id,
                    )*
                }
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }

            pub fn tooltip_value(&self) -> u32 {
                match self {
                    $(
                        Self::$variant => $tooltip,
                    )*
                }
            }
        }

        impl fmt::Display for MajorMinorBuff {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

major_minor_buffs! {
    MinorBrutality      = 1  => "Minor Brutality"       => 10,
    MajorBrutality      = 2  => "Major Brutality"       => 20,
    MinorSavagery       = 3  => "Minor Savagery"        => 1314,
    MajorSavagery       = 4  => "Major Savagery"        => 2629,
    MinorSorcery        = 5  => "Minor Sorcery"         => 10,
    MajorSorcery        = 6  => "Major Sorcery"         => 20,
    MinorProphecy       = 7  => "Minor Prophecy"        => 1314,
    MajorProphecy       = 8  => "Major Prophecy"        => 2629,
    MinorResolve        = 9  => "Minor Resolve"         => 2974,
    MajorResolve        = 10 => "Major Resolve"         => 5948,
    MinorBrittle        = 11 => "Minor Brittle"         => 10,
    MajorBrittle        = 12 => "Major Brittle"         => 20,
    MinorFortitude      = 13 => "Minor Fortitude"       => 15,
    MajorEndurance      = 14 => "Major Endurance"       => 30,
    MinorEndurance      = 15 => "Minor Endurance"       => 15,
    MajorFortitude      = 16 => "Major Fortitude"       => 30,
    MinorHeroism        = 19 => "Minor Heroism"         => 1,
    MajorHeroism        = 20 => "Major Heroism"         => 3,
    MinorMending        = 21 => "Minor Mending"         => 8,
    MajorMending        = 22 => "Major Mending"         => 16,
    MinorVitality       = 23 => "Minor Vitality"        => 6,
    MajorVitality       = 24 => "Major Vitality"        => 12,
    MinorEvasion        = 25 => "Minor Evasion"         => 10,
    MajorEvasion        = 26 => "Major Evasion"         => 20,
    MinorProtection     = 27 => "Minor Protection"      => 5,
    MajorProtection     = 28 => "Major Protection"      => 10,
    MinorMaim           = 29 => "Minor Maim"            => 5,
    MajorMaim           = 30 => "Major Maim"            => 10,
    MinorDefile         = 31 => "Minor Defile"          => 6,
    MajorDefile         = 32 => "Major Defile"          => 12,
    MinorExpedition     = 35 => "Minor Expedition"      => 15,
    MajorExpedition     = 36 => "Major Expedition"      => 30,
    Empower             = 37 => "Empower"               => 70,
    MinorCowardice      = 38 => "Minor Cowardice"       => 215,
    MajorCowardice      = 39 => "Major Cowardice"       => 430,
    MinorBreach         = 40 => "Minor Breach"          => 2974,
    MajorBreach         = 41 => "Major Breach"          => 5948,
    MinorBerserk        = 42 => "Minor Berserk"         => 5,
    MajorBerserk        = 43 => "Major Berserk"         => 10,
    MinorForce          = 44 => "Minor Force"           => 10,
    MajorForce          = 45 => "Major Force"           => 20,
    MajorCourage        = 46 => "Major Courage"         => 215,
    MinorCourage        = 48 => "Minor Courage"         => 430,
    MinorToughness      = 50 => "Minor Toughness"       => 10,
    Gallop              = 54 => "Gallop"                => 15,
    MinorEnervation     = 55 => "Minor Enervation"      => 10,
    MinorUncertainty    = 56 => "Minor Uncertainty"     => 1314,
    MinorLifesteal      = 57 => "Minor Lifesteal"       => 600,
    MinorMagickasteal   = 58 => "Minor Magickasteal"    => 168,
    MinorVulnerability  = 60 => "Minor Vulnerability"   => 5,
    MajorVulnerability  = 61 => "Major Vulnerability"   => 10,
}