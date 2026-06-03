use std::fmt;

macro_rules! ability_types {
    (
        $(
            $variant:ident = $id:expr => $name:expr
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum AbilityType {
            $(
                $variant = $id,
            )*
        }

        impl AbilityType {
            pub fn from_id(id: &u8) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn to_id(&self) -> u8 {
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

ability_types! {
    None = 0 => "None",
    Damage = 1 => "Damage",
    Heal = 2 => "Heal",
    Resurrect = 3 => "Resurrect",
    Blink = 4 => "Blink",
    Bonus = 5 => "Bonus",
    RegisterTrigger = 6 => "Register trigger",
    SetTarget = 7 => "Set target",
    Threat = 8 => "Threat",
    Stun = 9 => "Stun",
    Snare = 10 => "Snare",
    Silence = 11 => "Silence",
    RemoveType = 12 => "Remove type",
    SetCooldown = 13 => "Set cooldown",
    CombatResource = 14 => "Combat resource",
    DamageShield = 15 => "Damage shield",
    MovePosition = 16 => "Move position",
    Knockback = 17 => "Knockback",
    Charge = 18 => "Charge",
    Immunity = 19 => "Immunity",
    Intercept = 20 => "Intercept",
    Reflection = 21 => "Reflection",
    AreaEffect = 22 => "Area effect",
    Deprecated2 = 23 => "Deprecated 2",
    CreateInventoryItem = 24 => "Create inventory item",
    DamageLimit = 25 => "Damage limit",
    AreaTeleport = 26 => "Area teleport",
    Fear = 27 => "Fear",
    Trauma = 28 => "Trauma",
    Stealth = 29 => "Stealth",
    SeeStealth = 30 => "See stealth",
    Flight = 31 => "Flight",
    Disorient = 32 => "Disorient",
    Stagger = 33 => "Stagger",
    SlowFall = 34 => "Slow fall",
    Jump = 35 => "Jump",
    SiegeClusterAreaEffect = 36 => "Siege cluster area effect",
    Summon = 37 => "Summon",
    Mount = 38 => "Mount",
    InteractRefusalOverride = 39 => "Interact refusal override",
    DeflectAttack = 40 => "Deflect attack",
    NonExistent = 41 => "Non-existent",
    NoKill = 42 => "No kill",
    NoAggro = 43 => "No aggro",
    Dispel = 44 => "Dispel",
    Vampire = 45 => "Vampire",
    CreateInteractable = 46 => "Create interactable",
    ModifyCooldown = 47 => "Modify cooldown",
    Levitate = 48 => "Levitate",
    Pacify = 49 => "Pacify",
    ActionList = 50 => "Action list",
    Interrupt = 51 => "Interrupt",
    Block = 52 => "Block",
    OffBalance = 53 => "Off balance",
    Exhausted = 54 => "Exhausted",
    ModifyDuration = 55 => "Modify duration",
    Dodge = 56 => "Dodge",
    ShowNon = 57 => "Show non",
    Misdirect = 58 => "Misdirect",
    FreeCast = 59 => "Free cast",
    SiegeCreate = 60 => "Siege create",
    SiegeAreaEffect = 61 => "Siege area effect",
    Defend = 62 => "Defend",
    FreeInteract = 63 => "Free interact",
    ChangeAppearance = 64 => "Change appearance",
    AttackerReflect = 65 => "Attacker reflect",
    AttackerIntercept = 66 => "Attacker intercept",
    Disarm = 67 => "Disarm",
    Parry = 68 => "Parry",
    PathLine = 69 => "Path line",
    DoubleFire = 70 => "Double fire",
    FireProc = 71 => "Fire proc",
    Leap = 72 => "Leap",
    Reveal = 73 => "Reveal",
    SiegePackUp = 74 => "Siege pack up",
    Recall = 75 => "Recall",
    GrantAbility = 76 => "Grant ability",
    Hide = 77 => "Hide",
    SetHotbar = 78 => "Set hotbar",
    NoLockPick = 79 => "No lock pick",
    FillSoulGem = 80 => "Fill soul gem",
    SoulGemResurrect = 81 => "Soul gem resurrect",
    DespawnOverride = 82 => "Despawn override",
    UpdateDeathDialog = 83 => "Update death dialog",
    Deprecated4 = 84 => "Deprecated 4",
    ClientFx = 85 => "Client FX",
    AvoidDeath = 86 => "Avoid death",
    NonCombatBonus = 87 => "Non-combat bonus",
    NoSeeTarget = 88 => "No see target",
    Deprecated = 89 => "Deprecated",
    SetPersonality = 90 => "Set personality",
    Basic = 91 => "Basic",
    RewindTime = 92 => "Rewind time",
    LightHeavyAttackOverride = 93 => "Light/Heavy attack override",
    DerivedStatCache = 94 => "Derived stat cache",
    AvAReach = 95 => "AvA reach",
    RandomBranch = 96 => "Random branch",
    MountBlock = 97 => "Mount block",
    Deprecated3 = 98 => "Deprecated 3",
    HardDismount = 99 => "Hard dismount",
    LinkTarget = 100 => "Link target",
    CustomTargetArea = 101 => "Custom target area",
    DamageTransfer = 102 => "Damage transfer",
    DisableItemSets = 103 => "Disable item sets",
    FollowWaypointPath = 104 => "Follow waypoint path",
    SetAimAtTarget = 105 => "Set aim at target",
    FaceTarget = 106 => "Face target",
    LosMovePosition = 107 => "LOS move position",
    DisableClientTurning = 108 => "Disable client turning",
    DamageImmune = 109 => "Damage immune",
    StopMoving = 110 => "Stop moving",
    ResourceTap = 111 => "Resource tap",
    HotbarSlotOverride = 112 => "Hotbar slot override",
    Repair = 113 => "AvA object heal",
    PreventHealing = 114 => "Reduce healing from others",
    PlayerFlight = 115 => "Player flight",
    PauseCooldown = 116 => "Pause cooldown",
    DisableGameplayMechanics = 117 => "Disable gameplay mechanics",
    ModifyStackCount = 118 => "Modify stack count",
}

impl fmt::Display for AbilityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
