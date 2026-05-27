use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoefficientType {
    MaximumMagicka,
    MaximumHealth,
    HealthRecovery,
    SpellResistance,
    PhysicalResistance,
    SpellDamage,
    MaximumStamina,
    WeaponDamage,
}

impl CoefficientType {
    pub fn from_id(id: &u8) -> Option<Self> {
        match id {
            4  => Some(Self::MaximumMagicka),
            7  => Some(Self::MaximumHealth),
            8  => Some(Self::HealthRecovery),
            13 => Some(Self::SpellResistance),
            22 => Some(Self::PhysicalResistance),
            25 => Some(Self::SpellDamage),
            29 => Some(Self::MaximumStamina),
            35 => Some(Self::WeaponDamage),
            _  => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MaximumMagicka      => "Maximum Magicka",
            Self::MaximumHealth       => "Maximum Health",
            Self::HealthRecovery      => "Health Recovery",
            Self::SpellResistance     => "Spell Resistance",
            Self::PhysicalResistance  => "Physical Resistance",
            Self::SpellDamage         => "Spell Damage",
            Self::MaximumStamina      => "Maximum Stamina",
            Self::WeaponDamage        => "Weapon Damage",
        }
    }
}

impl fmt::Display for CoefficientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}