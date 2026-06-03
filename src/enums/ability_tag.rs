/// Found in list6b
macro_rules! ability_tags {
    (
        $(
            $variant:ident = $id:expr => $name:expr
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum AbilityTag {
            $(
                $variant = $id,
            )*
        }

        impl AbilityTag {
            pub fn from_id(id: &u16) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn to_id(&self) -> u16 {
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
        }
    };
}

ability_tags! {
    UIAffectingPassive = 0 => "UIAffectingPassive", // lockpicking, guard interactions, cyrodiil fighter's guild quests
    MundusBoon = 5 => "MundusBoon",
    DestructionStaffAbility = 15 => "DestructionStaffAbility",
    SkillLineDarkMagicAbility = 31 => "SkillLineDarkMagicAbility",
    BashDamageAbility = 50 => "BashDamageAbility",
    HeavyAttackStaff = 68 => "HeavyAttackStaff",
    HealingOnlyAbility = 85 => "HealingOnlyAbility", // for heals that don't do damage
    StealthRelated = 95 => "StealthRelated",
    PoisonDebuff = 97 => "PoisonDebuff",
    // Undodgable = 122, // unconfident
    PullAbility = 131 => "PullAbility", // both towards you and towards target
    Interrupts = 133 => "Interrupts",
    EnemyDurationAbility = 138 => "EnemyDurationAbility",
    EnemyAOEAbility = 139 => "EnemyAOEAbility",
    EnemyCastTimeAbility = 140 => "EnemyCastTimeAbility",
    HeavyAttack = 144 => "HeavyAttack",
    TwoHandedAbility = 154 => "TwoHandedAbility",
    HeavyAttackWeaponsAndMelee = 148 => "HeavyAttackWeaponsAndMelee",
    DeprecatedSiegeWeapon = 157 => "DeprecatedSiegeWeapon",
    EnchantmentPoisonEffects = 161 => "EnchantmentPoisonEffects",
    DiscoveredWhileDisguised = 168 => "DiscoveredWhileDisguised",
    Terrified = 172 => "Terrified",
    RoleplayDisguise = 175 => "RoleplayDisguise",
    SkillLineArdentFlameAbility = 180 => "SkillLineArdentFlameAbility",
    SkillLineDraconicPowerAbility = 181 => "SkillLineDraconicPowerAbility",
    SkillLineDualWieldAbility = 182 => "SkillLineDualWieldAbility",
    WeaponAbilityWallOfElements = 183 => "WeaponAbilityWallOfElements",
    EnemyLightAttackWithCooldown = 184 => "EnemyLightAttackWithCooldown",
    WeaponAbilityVolley = 185 => "WeaponAbilityVolley",
    SkillLineStormCallingAbility = 186 => "SkillLineStormCallingAbility",
    LightAndHeavyAttacksNoOverload = 187 => "LightAndHeavyAttacksNoOverload",
    SkillLineAedricSpearAbility = 188 => "SkillLineAedricSpearAbility",
    SkillLineDawnsWrathAbility = 189 => "SkillLineDawnsWrathAbility",
    DeprecatedCausesDisorient = 194 => "DeprecatedCausesDisorient",
    WeaponAbility = 195 => "WeaponAbility",
    SkillLineBowAbility = 197 => "SkillLineBowAbility",
    ShiftingStandard = 198 => "ShiftingStandard",
    SkillLineShadowAbility = 202 => "SkillLineShadowAbility",
    SkillLineSiphoningAbility = 203 => "SkillLineSiphoningAbility",
    SkillLineAssassinationAbility = 204 => "SkillLineAssassinationAbility",
    WerewolfUltimate = 205 => "WerewolfUltimate",
    SkillLineSupportAbility = 208 => "SkillLineSupportAbility",
    ShadowSilkAbility = 218 => "ShadowSilkAbility",
    BoneShieldAbility = 219 => "BoneShieldAbility",
    WallOfFrostAbility = 222 => "WallOfFrostAbility",
    UnblockableDisable = 221 => "UnblockableDisable",
    WallOfStormsAbility = 223 => "WallOfStormsAbility",
    ChanneledHeavyAttackCausesDot = 226 => "ChanneledHeavyAttackCausesDot",
    SoulSiphonAbility = 227 => "SoulSiphonAbility",
    SharedOrbShardsCooldown = 230 => "SharedOrbShardsCooldown",
    RegenerationAbility = 232 => "RegenerationAbility",
    StrifeFunnelHealth = 233 => "StrifeFunnelHealth",
    UndauntedSpider = 235 => "UndauntedSpider",
    IncapDamageBonus = 242 => "IncapDamageBonus",
    SkillLineOneHandAndShieldAbility = 244 => "SkillLineOneHandAndShieldAbility",
    SkillLineRestorationStaffAbility = 249 => "SkillLineRestorationStaffAbility",
    Synergy = 252 => "Synergy",
    WeaponAbilityPuncture = 260 => "WeaponAbilityPuncture",
    WeaponAbilityTwinSlashes = 261 => "WeaponAbilityTwinSlashes",
    WeaponAbilityElementalTouch = 263 => "WeaponAbilityElementalTouch",
    WerewolfSynergy = 266 => "WerewolfSynergy",
    GrantMechanicSynergy = 267 => "GrantMechanicSynergy",
    BlinkLeapTeleportPull = 268 => "BlinkLeapTeleportPull",
    SelfArmourIncreaseAbility = 269 => "SelfArmourIncreaseAbility",
    SkillLineAssaultAbility = 280 => "SkillLineAssaultAbility",
    GrimFocusAbility = 286 => "GrimFocusAbility",
    DamageOverTimeAbility = 307 => "DamageOverTimeAbility",
    HeavyAttackAll = 319 => "HeavyAttackAll",
    Mutilate = 320 => "Mutilate",
    ClassSkill = 324 => "ClassSkill",
    ExecuteScalingAbility = 326 => "ExecuteScalingAbility",
    SkillLineVampireAbility = 334 => "SkillLineVampireAbility",
    ClassPassive = 335 => "ClassPassive",
    WeaponPassive = 337 => "WeaponPassive",
    ArmourAbility = 338 => "ArmourAbility",
    ArmourPassive = 339 => "ArmourPassive",
    SoulAbility = 340 => "SoulAbility",
    SoulPassive = 341 => "SoulPassive",
    SkillLineWerewolfAbility = 342 => "SkillLineWerewolfAbility",
    SkillLineFightersGuildAbility = 346 => "SkillLineFightersGuildAbility",
    SkillLineMagesGuildPassive = 349 => "SkillLineMagesGuildPassive",
    SkillLineUndauntedAbility = 350 => "SkillLineUndauntedAbility",
    SkillLineUndauntedPassive = 351 => "SkillLineUndauntedPassive",
    SkillLineAssaultPassive = 353 => "SkillLineAssaultPassive",
    SkillLineSupportPassive = 355 => "SkillLineSupportPassive",
    SkillLineRacialPassive = 357 => "SkillLineRacialPassive",
    ChampionPoint = 359 => "ChampionPoint",
    SkillLineLegerdemainPassive = 361 => "SkillLineLegerdemainPassive",
    CyrodiilStatBonus = 367 => "CyrodiilStatBonus",
    Sigil = 376 => "Sigil",
    SelfHealShieldAbility = 379 => "SelfHealShieldAbility",
    StealthDetection = 380 => "StealthDetection",
    WeaponabilityStampede = 381 => "WeaponabilityStampede",
    WeaponAbilityLowSlash = 382 => "WeaponAbilityLowSlash",
    WeaponAbilityFlurry = 385 => "WeaponAbilityFlurry",
    RakkhatPads = 392 => "RakkhatPads",
    CrowdControlImmunity = 420 => "CrowdControlImmunity",
    PetAttack = 433 => "PetAttack",
    EnvironmentalTrap = 437 => "EnvironmentalTrap",
    GuardAbility = 445 => "GuardAbility",
    BarrierAbility = 452 => "BarrierAbility",
    DiseasedSpores = 458 => "DiseasedSpores",
    WeaponAbilityShieldWall = 481 => "WeaponAbilityShieldWall",
    NegativeEffect = 484 => "NegativeEffect",
    Purge = 490 => "Purge",
    WardenFrostGate = 501 => "WardenFrostGate",
    Teleport = 510 => "Teleport",
    SkillLineWintersEmbraceAbility = 513 => "SkillLineWintersEmbraceAbility",
    SkillLineGreenBalanceAbility = 514 => "SkillLineGreenBalanceAbility",
    SkillLineAnimalCompanionsAbility = 515 => "SkillLineAnimalCompanionsAbility",
    WardenBearAbility = 563 => "WardenBearAbility",
    BlinkChargeLeapTeleportPull = 531 => "BlinkChargeLeapTeleportPull",
    WeaponAbilityTwinSlashes2 = 593 => "WeaponAbilityTwinSlashes2",
    WeaponAbilityVolley2 = 609 => "WeaponAbilityVolley2",
    Reviving = 616 => "Reviving",
    TeleportFailsafe = 619 => "TeleportFailsafe",
    ContinuousAttackPassive = 624 => "ContinuousAttackPassive",
    WeaponAbilityReverseSlash = 626 => "WeaponAbilityReverseSlash",
    WeaponAbilityBlessingOfProtection = 633 => "WeaponAbilityBlessingOfProtection",
    WeaponAbilityWhirlwind = 635 => "WeaponAbilityWhirlwind",
    WeaponAbilityBladeCloak = 636 => "WeaponAbilityBladeCloak",
    WeaponAbilityDefensiveStance = 639 => "WeaponAbilityDefensiveStance",
    CloudrestUpstairsMechanicsFromMinis = 701 => "CloudrestUpstairsMechanicsFromMinis",
    SloadsSemblanceSet = 735 => "SloadsSemblanceSet",
    AOEWithPhysics = 752 => "AOEWithPhysics", // siege, necrotic orb, deep fissure
    WeaponAbilityUppercut = 759 => "WeaponAbilityUppercut",
    SkillLineGraveLordAbility = 782 => "SkillLineGraveLordAbility",
    SkillLineBoneTyrantAbility = 783 => "SkillLineBoneTyrantAbility",
    SkillLineLivingDeathAbility = 784 => "SkillLineLivingDeathAbility",
    HeavyAttackOverload = 785 => "HeavyAttackOverload",
    CreatesCorpsesOnDeath = 789 => "CreatesCorpsesOnDeath",
    PuzzleCubeMovement = 793 => "PuzzleCubeMovement",
    ConsumesCorpses = 833 => "ConsumesCorpses",
    PlayerTransformationAbility = 838 => "PlayerTransformationAbility",
    SummonAnchors = 852 => "SummonAnchors",
    PuzzleCube = 857 => "PuzzleCube",
    BloodScionAbility = 957 => "BloodScionAbility",
    IncreasedXPAPGain = 989 => "IncreasedXPAPGain",
    TetherBetweenTwoPlayersMechanic = 996 => "TetherBetweenTwoPlayersMechanic", // mind link from coral aerie and arcane conveyance from LC
    ArmourReduction = 1002 => "ArmourReduction", // doesn't include major/minor breach
    BahseiFightMechanics = 1006 => "BahseiFightMechanics",
    Brittle = 1008 => "Brittle",
    TransformationEffect = 1010 => "TransformationEffect",
    IncreasedXPAPFavourGain = 1015 => "IncreasedXPAPFavourGain",
    WeaponAbilityDefensiveStance2 = 1032 => "WeaponAbilityDefensiveStance2",
    Rattled = 1048 => "Rattled",
    SkillLineHeraldOfTheTomeAbility = 1107 => "SkillLineHeraldOfTheTomeAbility",
    SkillLineSoldierOfApocryphaAbility = 1108 => "SkillLineSoldierOfApocryphaAbility",
    SkillLineCurativeRuneformsAbility = 1109 => "SkillLineCurativeRuneformsAbility",
    InfiniteArchiveDefensiveVision = 1126 => "InfiniteArchiveDefensiveVision",
    InfiniteArchiveVision = 1128 => "InfiniteArchiveVision",
    WeaponAbilityPowerBash = 1145 => "WeaponAbilityPowerBash",
    PlayerTransformationBig = 1149 => "PlayerTransformationBig",
    ArcaneKnot = 1151 => "ArcaneKnot",
    KazpianAbility = 1188 => "KazpianAbility",
    BalefulDarkness = 1205 => "BalefulDarkness",
    OpulentOrdealMechanics = 1227 => "OpulentOrdealMechanics",
}
