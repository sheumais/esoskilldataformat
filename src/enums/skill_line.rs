use std::fmt;


macro_rules! skill_lines {
    (
        $(
            $variant:ident = $id:expr => $name:expr
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum SkillLine {
            $(
                $variant,
            )*
        }

        impl SkillLine {
            pub fn from_id(id: &u32) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }
        }

        impl fmt::Display for SkillLine {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

skill_lines! {
    OneHandAndShield = 29 => "One Hand and Shield",
    TwoHanded = 30 => "Two Handed",
    DualWield = 31 => "Dual Wield",
    Bow = 32 => "Bow",
    DestructionStaff = 33 => "Destruction Staff",
    RestorationStaff = 34 => "Restoration Staff",

    LightArmour = 24 => "Light Armour",
    MediumArmour = 25 => "Medium Armour",
    HeavyArmour = 26 => "Heavy Armour",

    AedricSpear = 22 => "Aedric Spear",
    DawnsWrath = 27 => "Dawn's Wrath",
    RestoringLight = 28 => "Restoring Light",

    ArdentFlame = 35 => "Ardent Flame",
    DraconicPower = 36 => "Draconic Power",
    EarthenHeart = 37 => "Earthen Heart",

    Assassination = 38 => "Assassination",
    Shadow = 39 => "Shadow",
    Siphoning = 40 => "Siphoning",

    DarkMagic = 41 => "Dark Magic",
    DaedricSummoning = 42 => "Daedric Summoning",
    StormCalling = 43 => "Storm Calling",

    AnimalCompanions = 127 => "Animal Companions",
    GreenBalance = 128 => "Green Balance",
    WintersEmbrace = 129 => "Winter's Embrace",

    GraveLord = 131 => "Grave Lord",
    BoneTyrant = 132 => "Bone Tyrant",
    LivingDeath = 133 => "Living Death",

    HeraldOfTheTome = 218 => "Herald of the Tome",
    SoldierOfApocrypha = 219 => "Soldier of Apocrypha",
    CurativeRuneforms = 220 => "Curative Runeforms",

    MagesGuild = 44 => "Mages Guild",
    FightersGuild = 45 => "Fighters Guild",
    Undaunted = 55 => "Undaunted",
    PsijicOrder = 130 => "Psijic Order",
    ThievesGuild = 117 => "Thieves Guild",
    DarkBrotherhood = 118 => "Dark Brotherhood",

    Assault = 48 => "Assault",
    Support = 67 => "Support",
    Emperor = 71 => "Emperor",

    Orc = 52 => "Orc",
    HighElf = 56 => "High Elf",
    WoodElf = 57 => "Wood Elf",
    Khajiit = 58 => "Khajiit",
    Imperial = 59 => "Imperial",
    Breton = 60 => "Breton",
    Redguard = 62 => "Redguard",
    Argonian = 63 => "Argonian",
    DarkElf = 64 => "Dark Elf",
    Nord = 65 => "Nord",

    Werewolf = 50 => "Werewolf",
    Vampire = 51 => "Vampire",
    SoulMagic = 72 => "Soul Magic",
    Legerdemain = 111 => "Legerdemain",
    Scrying = 155 => "Scrying",
    Excavation = 157 => "Excavation",

    Provisioning = 76 => "Provisioning",
    Alchemy = 77 => "Alchemy",
    Enchanting = 78 => "Enchanting",
    Blacksmithing = 79 => "Blacksmithing",
    Woodworking = 80 => "Woodworking",
    Clothing = 81 => "Clothing",
    Jewellery = 141 => "Jewellery",

    CompanionArdentWarrior = 174 => "Companion Ardent Warrior",
    CompanionDraconicArmorBastian = 175 => "Companion Draconic Armor (Bastian)",
    CompanionRadiatingHeartBastian = 176 => "Companion Radiating Heart (Bastian)",
    CompanionDeadlyAssassin = 177 => "Companion Deadly Assassin",
    CompanionLivingShade = 178 => "Companion Living Shade",
    CompanionSoulThief = 179 => "Companion Soul Thief",
    CompanionTwoHanded = 180 => "Companion Two Handed",
    CompanionOneHandAndShield = 181 => "Companion One Hand and Shield",
    CompanionDualWield = 182 => "Companion Dual Wield",
    CompanionBow = 183 => "Companion Bow",
    CompanionDestructionStaff = 184 => "Companion Destruction Staff",
    CompanionRestorationStaff = 185 => "Companion Restoration Staff",
    CompanionLightArmor = 186 => "Companion Light Armor",
    CompanionMediumArmor = 187 => "Companion Medium Armor",
    CompanionHeavyArmor = 188 => "Companion Heavy Armor",
    CompanionFightersGuild = 189 => "Companion Fighters Guild",
    CompanionMagesGuild = 190 => "Companion Mages Guild",
    CompanionUndaunted = 191 => "Companion Undaunted",
    CompanionImperial = 192 => "Companion Imperial",
    CompanionDarkElf = 193 => "Companion Dark Elf",
    CompanionLightningCaller = 196 => "Companion Lightning Caller",
    CompanionMischievousCaster = 197 => "Companion Mischievous Caster",
    CompanionPlayfulSchemer = 198 => "Companion Playful Schemer",
    CompanionKhajiitEmber = 199 => "Companion Khajiit (Ember)",
    CompanionBlazingMight = 200 => "Companion Blazing Might",
    CompanionBrilliantShield = 201 => "Companion Brilliant Shield",
    CompanionHealingGrace = 202 => "Companion Healing Grace",
    CompanionBreton = 203 => "Companion Breton",

    CompanionBeastsOfTheHunt = 241 => "Companion Beasts of the Hunt",
    CompanionWintersBite = 242 => "Companion Winter's Bite",
    CompanionVerdantGrowth = 243 => "Companion Verdant Growth",

    CompanionScholarOfApocrypha = 246 => "Companion Scholar of Apocrypha",
    CompanionQuillKnight = 247 => "Companion Quill Knight",
    CompanionRevitalizingResearcher = 248 => "Companion Revitalizing Researcher",

    CompanionRedguard = 249 => "Companion Redguard",
    CompanionArgonian = 250 => "Companion Argonian",

    CompanionWarriorsBanishment = 260 => "Companion Warrior's Banishment",
    CompanionRemedyOfAtonement = 261 => "Companion Remedy of Atonement",
    CompanionGuardiansCommitment = 262 => "Companion Guardian's Commitment",
    CompanionKhajiitZerithVar = 263 => "Companion Khajiit (Zerith-var)",
    CompanionRadiatingHeartTanlorin = 264 => "Companion Radiating Heart (Tanlorin)",
    CompanionDraconicArmorTanlorin = 265 => "Companion Draconic Armor (Tanlorin)",
    CompanionEmpathicFighter = 266 => "Companion Empathic Fighter",
    CompanionHighElf = 267 => "Companion High Elf",

    VengeanceArdentFlame = 297 => "Vengeance Ardent Flame",
    VengeanceDraconicPower = 298 => "Vengeance Draconic Power",
    VengeanceEarthenHeart = 299 => "Vengeance Earthen Heart",
    VengeanceAssassination = 300 => "Vengeance Assassination",
    VengeanceShadow = 301 => "Vengeance Shadow",
    VengeanceSiphoning = 302 => "Vengeance Siphoning",
    VengeanceAedricSpear = 303 => "Vengeance Aedric Spear",
    VengeanceDawnsWrath = 304 => "Vengeance Dawn's Wrath",
    VengeanceRestoringLight = 305 => "Vengeance Restoring Light",
    VengeanceDaedricSummoning = 306 => "Vengeance Daedric Summoning",
    VengeanceDarkMagic = 307 => "Vengeance Dark Magic",
    VengeanceStormCalling = 308 => "Vengeance Storm Calling",
    VengeanceAnimalCompanions = 309 => "Vengeance Animal Companions",
    VengeanceGreenBalance = 310 => "Vengeance Green Balance",
    VengeanceWintersEmbrace = 311 => "Vengeance Winter's Embrace",
    VengeanceGraveLord = 312 => "Vengeance Grave Lord",
    VengeanceBoneTyrant = 313 => "Vengeance Bone Tyrant",
    VengeanceLivingDeath = 314 => "Vengeance Living Death",
    VengeanceCurativeRuneforms = 315 => "Vengeance Curative Runeforms",
    VengeanceSoldierOfApocrypha = 316 => "Vengeance Soldier of Apocrypha",
    VengeanceHeraldOfTheTome = 317 => "Vengeance Herald of the Tome",

    VengeanceTwoHanded = 319 => "Vengeance Two Handed",
    VengeanceOneHandAndShield = 320 => "Vengeance One Hand and Shield",
    VengeanceDualWield = 321 => "Vengeance Dual Wield",
    VengeanceBow = 322 => "Vengeance Bow",
    VengeanceDestructionStaff = 323 => "Vengeance Destruction Staff",
    VengeanceRestorationStaff = 324 => "Vengeance Restoration Staff",

    VengeanceAssault = 325 => "Vengeance Assault",
    VengeanceSupport = 326 => "Vengeance Support",

    VengeanceFightersGuild = 330 => "Vengeance Fighters Guild",
    VengeanceMagesGuild = 331 => "Vengeance Mages Guild",
    VengeanceUndaunted = 332 => "Vengeance Undaunted",

    PreU49DraconicPower = 334 => "Pre-U49 Draconic Power",

    VengeanceSoldier = 339 => "Vengeance Soldier",
    VengeanceVanguard = 340 => "Vengeance Vanguard",
    VengeanceBattleMedic = 341 => "Vengeance Battle Medic",
    VengeanceScout = 342 => "Vengeance Scout",

    ClassMasteryDragonknight = 351 => "Dragonknight Class Mastery",
    ClassMasteryArcanist = 352 => "Arcanist Class Mastery",
    ClassMasteryNecromancer = 353 => "Necromancer Class Mastery",
    ClassMasteryWarden = 354 => "Warden Class Mastery",
    ClassMasteryTemplar = 355 => "Templar Class Mastery",
    ClassMasteryNightblade = 356 => "Nightblade Class Mastery",
    ClassMasterySorcerer = 357 => "Sorcerer Class Mastery",
}

impl SkillLine {
    pub fn is_vengeance(&self) -> bool {
        matches!(self, Self::VengeanceArdentFlame | Self::VengeanceDraconicPower | Self::VengeanceEarthenHeart | Self::VengeanceAssassination | Self::VengeanceShadow | Self::VengeanceSiphoning | Self::VengeanceAedricSpear | Self::VengeanceDawnsWrath | Self::VengeanceRestoringLight | Self::VengeanceDaedricSummoning | Self::VengeanceDarkMagic | Self::VengeanceStormCalling | Self::VengeanceAnimalCompanions | Self::VengeanceGreenBalance | Self::VengeanceWintersEmbrace | Self::VengeanceGraveLord | Self::VengeanceBoneTyrant | Self::VengeanceLivingDeath | Self::VengeanceCurativeRuneforms | Self::VengeanceSoldierOfApocrypha | Self::VengeanceHeraldOfTheTome | Self::VengeanceTwoHanded | Self::VengeanceOneHandAndShield | Self::VengeanceDualWield | Self::VengeanceBow | Self::VengeanceDestructionStaff | Self::VengeanceRestorationStaff | Self::VengeanceAssault | Self::VengeanceSupport | Self::VengeanceFightersGuild | Self::VengeanceMagesGuild | Self::VengeanceUndaunted | Self::VengeanceSoldier | Self::VengeanceVanguard | Self::VengeanceBattleMedic | Self::VengeanceScout)
    }

    pub fn is_companion(&self) -> bool {
        matches!(self, Self::CompanionArdentWarrior | Self::CompanionDraconicArmorBastian | Self::CompanionRadiatingHeartBastian |Self::CompanionDeadlyAssassin | Self::CompanionLivingShade | Self::CompanionSoulThief |Self::CompanionTwoHanded | Self::CompanionOneHandAndShield | Self::CompanionDualWield | Self::CompanionBow | Self::CompanionDestructionStaff | Self::CompanionRestorationStaff | Self::CompanionLightArmor | Self::CompanionMediumArmor | Self::CompanionHeavyArmor |Self::CompanionFightersGuild | Self::CompanionMagesGuild | Self::CompanionUndaunted |Self::CompanionImperial | Self::CompanionDarkElf | Self::CompanionHighElf | Self::CompanionBreton | Self::CompanionKhajiitEmber | Self::CompanionRedguard | Self::CompanionArgonian | Self::CompanionKhajiitZerithVar |Self::CompanionLightningCaller | Self::CompanionMischievousCaster | Self::CompanionPlayfulSchemer |Self::CompanionBlazingMight | Self::CompanionBrilliantShield | Self::CompanionHealingGrace |Self::CompanionBeastsOfTheHunt | Self::CompanionWintersBite | Self::CompanionVerdantGrowth |Self::CompanionScholarOfApocrypha | Self::CompanionQuillKnight | Self::CompanionRevitalizingResearcher |Self::CompanionWarriorsBanishment | Self::CompanionRemedyOfAtonement | Self::CompanionGuardiansCommitment |Self::CompanionRadiatingHeartTanlorin | Self::CompanionDraconicArmorTanlorin | Self::CompanionEmpathicFighter)
    }

    pub fn is_class_mastery(&self) -> bool {
        matches!(self, Self::ClassMasteryDragonknight | Self::ClassMasteryArcanist | Self::ClassMasteryNecromancer | Self::ClassMasteryWarden | Self::ClassMasteryTemplar | Self::ClassMasteryNightblade | Self::ClassMasterySorcerer)
    }
}