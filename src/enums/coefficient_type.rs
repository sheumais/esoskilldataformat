use std::fmt;

use crate::SkillCoef;

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
    Power,
}

impl CoefficientType {
    pub fn from_id(id: &u8) -> Option<Self> {
        match id {
            2  => Some(Self::Power),
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
            Self::Power               => "Power",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoefStat {
    Single(u8),
    Pair(u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoefTerm {
    pub stat: CoefStat,
    pub value: f32,
}

impl CoefStat {
    fn is_weapon_spell(t: u8) -> bool { matches!(t, 25 | 35) }
    fn is_resource(t: u8) -> bool { matches!(t, 4 | 29) }
    fn is_resist(t: u8) -> bool { matches!(t, 13 | 22) }

    fn is_pair(a: u8, b: u8) -> bool {
        a != b
            && ((Self::is_weapon_spell(a) && Self::is_weapon_spell(b))
                || (Self::is_resource(a) && Self::is_resource(b))
                || (Self::is_resist(a) && Self::is_resist(b)))
    }

    pub fn label(&self) -> String {
        match *self {
            CoefStat::Single(id) => CoefficientType::from_id(&id)
                .map(|t| t.as_str().to_string())
                .unwrap_or_else(|| format!("unknown({id})")),
            CoefStat::Pair(t1, t2) => match (t1, t2) {
                (25, 35) | (35, 25) => "MaxPower".to_string(),
                (4, 29) | (29, 4) => "MaxResource".to_string(),
                (22, 13) | (13, 22) => "MaxResist".to_string(),
                _ => format!(
                    "max({}, {})",
                    CoefficientType::from_id(&t1).map_or("unknown", |t| t.as_str()),
                    CoefficientType::from_id(&t2).map_or("unknown", |t| t.as_str()),
                ),
            },
        }
    }

    pub fn single_type(&self) -> Option<CoefficientType> {
        match *self {
            CoefStat::Single(id) => CoefficientType::from_id(&id),
            CoefStat::Pair(..) => None,
        }
    }
}

impl SkillCoef {
    pub fn combined_terms(&self) -> Vec<CoefTerm> {
        let terms: Vec<(u8, f32)> = [
            (self.type1, self.coef1),
            (self.type2, self.coef2),
            (self.type3, self.coef3),
            (self.type4, self.coef4),
        ]
        .into_iter()
        .filter(|(ty, coef)| *ty > 0 && *coef != 0.0)
        .collect();

        let mut used = vec![false; terms.len()];
        let mut combined = Vec::with_capacity(terms.len());

        for i in 0..terms.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            let (t1, c1) = terms[i];

            let partner = (i + 1..terms.len())
                .find(|&j| !used[j] && terms[j].1 == c1 && CoefStat::is_pair(t1, terms[j].0));

            match partner {
                Some(j) => {
                    used[j] = true;
                    combined.push(CoefTerm { stat: CoefStat::Pair(t1, terms[j].0), value: c1 });
                }
                None => combined.push(CoefTerm { stat: CoefStat::Single(t1), value: c1 }),
            }
        }
        combined
    }

    pub fn render(&self) -> Option<String> {
        let terms = self.combined_terms();
        if terms.is_empty() {
            return None;
        }
        Some(
            terms
                .iter()
                .map(|t| format!("{}×{}", t.value, t.stat.label()))
                .collect::<Vec<_>>()
                .join(" + "),
        )
    }
}