#![allow(
    non_camel_case_types,
    dead_code,
    non_snake_case,
    clippy::upper_case_acronyms
)]

use std::ffi::c_void;

use il2cpp_runtime::{il2cpp_enum_type, il2cpp_getter_property, il2cpp_value_type};
use il2cpp_runtime::prelude::*;
use il2cpp_runtime::types::List;

#[il2cpp_value_type("RPG.Client.TextID")]
pub struct RPG_Client_TextID {
    pub hash: i32,
    pub hash64: u64,
}

impl RPG_Client_TextID__Boxed {
    #[il2cpp_field(name = "hash")]
    pub fn hash(&self) -> System_Int32__Boxed {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_TeamType {
    TeamUnknow,
    TeamLight,
    TeamDark,
    TeamNeutral,
    TeamNPC,
    Count
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AliveState {
    Unknown,
    Alive,
    Limbo,
    LimboRevivable,
    Deathrattle,
    Dying,
    Died,
    WillBeDestroyed,
    Destroyed,
}


#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AvatarPropertyType {
	Unknown,
	MaxHP,
	Attack,
	Defence,
	Speed,
	CriticalChance,
	CriticalDamage,
	HealRatio,
	StanceBreakAddedRatio,
	SPRatio,
	StatusProbability,
	StatusResistance,
	PhysicalAddedRatio,
	PhysicalResistance,
	FireAddedRatio,
	FireResistance,
	IceAddedRatio,
	IceResistance,
	ThunderAddedRatio,
	ThunderResistance,
	WindAddedRatio,
	WindResistance,
	QuantumAddedRatio,
	QuantumResistance,
	ImaginaryAddedRatio,
	ImaginaryResistance,
	BaseHP,
	HPDelta,
	BaseAttack,
	AttackDelta,
	BaseDefence,
	DefenceDelta,
	HPAddedRatio,
	AttackAddedRatio,
	DefenceAddedRatio,
	BaseSpeed,
	HealTakenRatio,
	PhysicalResistanceDelta,
	FireResistanceDelta,
	IceResistanceDelta,
	ThunderResistanceDelta,
	WindResistanceDelta,
	QuantumResistanceDelta,
	ImaginaryResistanceDelta,
	AllDamageReduce,
	RelicValueExtraAdditionRatio,
	EquipValueExtraAdditionRatio,
	EquipExtraRank,
	AvatarExtraRank,
	AllDamageTypeAddedRatio,
	SpeedAddedRatio,
	SpeedDelta,
	CriticalChanceBase,
	CriticalDamageBase,
	SPRatioBase,
	HealRatioBase,
	StatusProbabilityBase,
	StatusResistanceBase,
	BreakDamageAddedRatio,
	BreakDamageAddedRatioBase,
	MaxSP,
	SpecialMaxSP,
	PhysicalPenetrate,
	FirePenetrate,
	IcePenetrate,
	ThunderPenetrate,
	WindPenetrate,
	QuantumPenetrate,
	ImaginaryPenetrate,
	AllDamageTypePenetrate,
	BreakDamageExtraAddedRatio,
	ElationDamageAddedRatio,
	ElationDamageAddedRatioBase,
	ExtraAttackAddedRatio1,
	ExtraAttackAddedRatio2,
	ExtraAttackAddedRatio3,
	ExtraAttackAddedRatio4,
	ExtraDefenceAddedRatio1,
	ExtraDefenceAddedRatio2,
	ExtraDefenceAddedRatio3,
	ExtraDefenceAddedRatio4,
	ExtraHPAddedRatio1,
	ExtraHPAddedRatio2,
	ExtraHPAddedRatio3,
	ExtraHPAddedRatio4,
	ExtraHealAddedRatio,
	ExtraAllDamageTypeAddedRatio1,
	ExtraAllDamageTypeAddedRatio2,
	ExtraAllDamageTypeAddedRatio3,
	ExtraAllDamageTypeAddedRatio4,
	ExtraAllDamageReduce,
	ExtraShieldAddedRatio,
	ExtraSpeedAddedRatio1,
	ExtraSpeedAddedRatio2,
	ExtraSpeedAddedRatio3,
	ExtraSpeedAddedRatio4,
	ExtraLuckChance,
	ExtraLuckDamage,
	ExtraFrontPowerBase,
	ExtraFrontPowerAddedRatio1,
	ExtraFrontPowerAddedRatio2,
	ExtraBackPowerBase,
	ExtraBackPowerAddedRatio1,
	ExtraBackPowerAddedRatio2,
	ExtraUltraDamageAddedRatio1,
	ExtraSkillDamageAddedRatio1,
	ExtraNormalDamageAddedRatio1,
	ExtraInsertDamageAddedRatio1,
	ExtraTotalFrontPower,
	ExtraTotalBackPower,
	ExtraDOTDamageAddedRatio1,
	ExtraHealBase,
	ExtraShieldBase,
	ExtraTotalShieldPower,
	ExtraTotalHealPower,
	ExtraTotalSpeedAddedRatio,
	ExtraEnergyBar,
	ExtraInitSP,
	ExtraElementDamageAddedRatio1,
	ExtraTotalLuckChance,
	ExtraTotalLuckChanceBase,
	ExtraTotalLuckDamage,
	ExtraTotalLuckDamageBase,
	ExtraDamageAddedRatio1,
	ExtraQuantumResonance,
	ExtraFrontPowerConvert,
	ExtraBackPowerConvert,
	ExtraLuckDamageConvert,
	ExtraLuckChanceConvert,
	ExtraHealConvert,
	ExtraShieldConvert,
	ExtraAllDamageReduceConvert,
	ExtraTotalAllDamageReduce,
	ExtraAllDamageTypeAddedRatio5
}

#[il2cpp_ref_type("RPG.GameCore.AvatarPropertyRow")]
pub struct RPG_GameCore_AvatarPropertyRow;
impl RPG_GameCore_AvatarPropertyRow {
    #[il2cpp_field(name = "IconPath")]
    pub fn IconPath(&self) -> Il2CppString {}
	
    #[il2cpp_field(name = "PropertyName")]
    pub fn PropertyName(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_ItemMainType {
    Unknown = 0,
    Virtual = 1,
    AvatarCard = 2,
    Equipment = 3, // Light Cones
    Relic = 4,     // Relics
    Usable = 5,
    Material = 6,
    Mission = 7,
    Display = 8,
    Pet = 9,
}

#[il2cpp_value_type("RPG.GameCore.FixPoint")]
pub struct RPG_GameCore_FixPoint {
    pub m_rawValue: i64,
}

#[il2cpp_ref_type("RPG.Client.ModuleManager")]
pub struct RPG_Client_ModuleManager;
impl RPG_Client_ModuleManager {
	#[il2cpp_field(name = "InventoryModule")]
    pub fn InventoryModule(&self) -> RPG_Client_InventoryModule {}
	
	#[il2cpp_field(name = "PlayerModule")]
    pub fn PlayerModule(&self) -> RPG_Client_PlayerModule {}
}

#[il2cpp_ref_type("System.Object")]
pub struct System_Object_Dummy;

#[il2cpp_ref_type("RPG.Client.GlobalVars")]
pub struct RPG_Client_GlobalVars;
impl RPG_Client_GlobalVars {
    #[il2cpp_field(name = "s_ModuleManager")]
    pub fn s_ModuleManager() -> RPG_Client_ModuleManager {}
}

#[il2cpp_ref_type("RPG.Client.TextmapStatic")]
pub struct RPG_Client_TextmapStatic;
impl RPG_Client_TextmapStatic {
    #[il2cpp_method(name = "GetText", args = ["RPG.Client.TextID", "object[]"])]
    pub fn get_text(id: &RPG_Client_TextID, replace_params: *const c_void) -> Il2CppString {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarRow")]
pub struct RPG_GameCore_AvatarRow;
impl RPG_GameCore_AvatarRow {
	#[il2cpp_field(name = "AvatarSideIconPath")]
	pub fn AvatarSideIconPath(&self) -> Il2CppString {}
	
	#[il2cpp_field(name = "AvatarID")]
	pub fn AvatarID(&self) -> System_UInt32__Boxed {}

	#[il2cpp_field(name = "AvatarName")]
	pub fn AvatarName(&self) -> RPG_Client_TextID__Boxed {}
	
	#[il2cpp_field(name = "AvatarBaseType")]
	pub fn AvatarBaseType(&self) -> RPG_GameCore_AvatarBaseType__Boxed {}
}


#[il2cpp_ref_type("RPG.Client.ItemData")]
pub struct RPG_Client_ItemData;
impl RPG_Client_ItemData {
    #[il2cpp_getter_property(property = "UID")]
    pub fn get_UID(&self) -> u32 {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicSubAffixConfigRow")]
pub struct RPG_GameCore_RelicSubAffixConfigRow;
impl RPG_GameCore_RelicSubAffixConfigRow {
    #[il2cpp_field(name = "BaseValue")]
    pub fn BaseValue(&self) -> RPG_GameCore_FixPoint__Boxed {}

    #[il2cpp_field(name = "StepValue")]
    pub fn StepValue(&self) -> RPG_GameCore_FixPoint__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicSubAffixConfigExcelTable")]
pub struct RPG_GameCore_RelicSubAffixConfigExcelTable;
impl RPG_GameCore_RelicSubAffixConfigExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint", "uint"])]
    pub fn GetData(sub_affix_group: u32, avatar_property_type: u32) -> RPG_GameCore_RelicSubAffixConfigRow {}
}

#[il2cpp_ref_type("RPG.GameCore.GamePlayStatic")]
pub struct RPG_GameCore_GamePlayStatic;
impl RPG_GameCore_GamePlayStatic {
    #[il2cpp_method(name = "CalcRelicSubAffixValue", args = ["RPG.GameCore.FixPoint", "RPG.GameCore.FixPoint", "uint", "uint"])]
    pub fn CalcRelicSubAffixValue(base_config_value: RPG_GameCore_FixPoint, step_config_value: RPG_GameCore_FixPoint, count: u32, step: u32) -> RPG_GameCore_FixPoint {}
}
#[il2cpp_ref_type("RPG.Client.RelicItemData", base(RPG_Client_ItemData))]
pub struct RPG_Client_RelicItemData;

impl RPG_Client_RelicItemData {
    #[il2cpp_getter_property(property = "RelicRow")]
    pub fn get_RelicRow(&self) -> RPG_GameCore_RelicConfigRow {}

    #[il2cpp_getter_property(property = "BelongAvatarID")]
    pub fn get_BelongAvatarID(&self) -> u32 {}

    #[il2cpp_getter_property(property = "MainAffixPropertyType")]
    pub fn get_MainAffixPropertyType(&self) -> RPG_GameCore_AvatarPropertyType {}
	
	#[il2cpp_getter_property(property = "ReforgeSubAffixes")]
    pub fn get_ReforgeSubAffixes(&self) -> Il2CppArray {}

    #[il2cpp_getter_property(property = "PreviewSubAffixList")]
    pub fn get_PreviewSubAffixList(&self) -> Il2CppArray {}

    #[il2cpp_getter_property(property = "SubAffixList")]
    pub fn get_SubAffixList(&self) -> Il2CppArray {}
    
    #[il2cpp_getter_property(property = "IsDiscard")]
    pub fn get_IsDiscard(&self) -> bool {}

    #[il2cpp_getter_property(property = "IsProtected")]
    pub fn get_IsProtected(&self) -> bool {}

    #[il2cpp_getter_property(property = "Level")]
    pub fn get_Level(&self) -> u32 {}
	
    #[il2cpp_method(name = "GetSubAffixPropertyValue", args = ["RPG.GameCore.AvatarPropertyType"])]
    pub fn GetSubAffixPropertyValue(&self, sub_affix_id: RPG_GameCore_AvatarPropertyType) -> RPG_GameCore_FixPoint__Boxed {}

    #[il2cpp_method(name = "_GetPropertyTypeByMainAffixID", args = ["uint"])]
    pub fn _GetPropertyTypeByMainAffixID(&self, main_affix_id: u32) -> RPG_GameCore_AvatarPropertyType {}

    #[il2cpp_method(name = "_GetPropertyTypeBySubAffixID", args = ["uint"])]
    pub fn _GetPropertyTypeBySubAffixID(&self, sub_affix_id: u32) -> RPG_GameCore_AvatarPropertyType {}

    #[il2cpp_method(name = "GetMainAffixPropertyValue", args = [])]
    pub fn GetMainAffixPropertyValue(&self) -> RPG_GameCore_FixPoint {}

}


#[il2cpp_ref_type("RPG.Client.EquipmentItemData", base(RPG_Client_ItemData))]
pub struct RPG_Client_EquipmentItemData;
impl RPG_Client_EquipmentItemData {
    #[il2cpp_getter_property(property = "BelongAvatarID")]
    pub fn get_BelongAvatarID(&self) -> u32 {}

    #[il2cpp_getter_property(property = "IsDiscard")]
    pub fn get_IsDiscard(&self) -> bool {}

    #[il2cpp_getter_property(property = "IsProtected")]
    pub fn get_IsProtected(&self) -> bool {}

    #[il2cpp_getter_property(property = "Level")]
    pub fn get_Level(&self) -> u32 {}

    #[il2cpp_field(name = "_Rank")]
    pub fn _Rank(&self) -> System_UInt32__Boxed {}

    #[il2cpp_getter_property(property = "Version")]
    pub fn get_Version(&self) -> u32 {}

    #[il2cpp_getter_property(property = "Promotion")]
    pub fn get_Promotion(&self) -> u32 {}

    #[il2cpp_getter_property(property = "EquipmentRow")]
    pub fn get_EquipmentRow(&self) -> RPG_GameCore_EquipmentRow {}
}

#[il2cpp_ref_type("RPG.GameCore.EquipmentRow")]
pub struct RPG_GameCore_EquipmentRow;
impl RPG_GameCore_EquipmentRow {
    #[il2cpp_field(name = "EquipmentID")]
    pub fn EquipmentID(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "EquipmentName")]
    pub fn EquipmentName(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_CombatPowerRelicRarityType {
    CombatPowerRelicRarity1,
    CombatPowerRelicRarity2,
    CombatPowerRelicRarity3,
    CombatPowerRelicRarity4,
    CombatPowerRelicRarity5,
}

#[il2cpp_ref_type("RPG.GameCore.RelicSetConfigRow")]
pub struct RPG_GameCore_RelicSetConfigRow;
impl RPG_GameCore_RelicSetConfigRow {
    #[il2cpp_field(name = "SetName")]
    pub fn SetName(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicBaseTypeRow")]
pub struct RPG_GameCore_RelicBaseTypeRow;
impl RPG_GameCore_RelicBaseTypeRow {
    #[il2cpp_field(name = "BaseTypeText")]
    pub fn BaseTypeText(&self) -> RPG_Client_TextID__Boxed {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_RelicSetType {
    Unknow,
    HEAD,
    HAND,
    BODY,
    FOOT,
    NECK,
    OBJECT
}
#[il2cpp_ref_type("RPG.Client.InventoryModule")]
pub struct RPG_Client_InventoryModule;

impl RPG_Client_InventoryModule {
    #[il2cpp_method(name = "GetRelicDataByUID", args = ["uint"])]
    pub fn get_relic_data_by_uid(&self, uid: u32) -> RPG_Client_RelicItemData {}
	
	#[il2cpp_method(name = "GetItemsByMainTypes", args = ["RPG.GameCore.ItemMainType[]"])]
    pub fn get_items_by_main_types(&self, main_types: Il2CppArray) -> List {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicDataInfoExcelTable")]
pub struct RPG_GameCore_RelicDataInfoExcelTable;
impl RPG_GameCore_RelicDataInfoExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint", "RPG.GameCore.RelicType"])]
    pub fn GetData(set_id: u32, relic_type: RPG_GameCore_RelicSetType) -> RPG_GameCore_RelicDataInfoRow {}
	
	#[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}
	
		
    #[il2cpp_method(name = "IsDataLoaded", args = [])]
    pub fn IsDataLoaded() -> bool {}

    #[il2cpp_method(name = "LoadData", args = [])]
    pub fn LoadData() {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicDataInfoRow")]
pub struct RPG_GameCore_RelicDataInfoRow;
impl RPG_GameCore_RelicDataInfoRow {
	#[il2cpp_field(name = "SetID")]
    pub fn SetID(&self) -> System_UInt32__Boxed {}
	
    #[il2cpp_field(name = "RelicName")]
    pub fn RelicName(&self) -> Il2CppString {}

    #[il2cpp_field(name = "IconPath")]
    pub fn IconPath(&self) -> Il2CppString {}
	
	#[il2cpp_field(name = "Type")]
    pub fn Type(&self) -> RPG_GameCore_RelicSetType__Boxed {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicConfigExcelTable")]
pub struct RPG_GameCore_RelicConfigExcelTable;
impl RPG_GameCore_RelicConfigExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint"])]
    pub fn GetData(id: u32) -> RPG_GameCore_RelicConfigRow {}
	
	#[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}

}

#[il2cpp_ref_type("RPG.GameCore.RelicConfigRow")]
pub struct RPG_GameCore_RelicConfigRow;
impl RPG_GameCore_RelicConfigRow {
    #[il2cpp_field(name = "ID")]
    pub fn ID(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "SetID")]
    pub fn SetID(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "Rarity")]
    pub fn Rarity(&self) -> RPG_GameCore_CombatPowerRelicRarityType__Boxed {}

    #[il2cpp_field(name = "Type")]
    pub fn Type(&self) -> RPG_GameCore_RelicSetType__Boxed {}

    #[il2cpp_field(name = "MaxLevel")]
    pub fn MaxLevel(&self) -> System_Int32__Boxed {}

    #[il2cpp_field(name = "MainAffixGroup")]
    pub fn MainAffixGroup(&self) -> System_UInt32__Boxed {}

    #[il2cpp_field(name = "SubAffixGroup")]
    pub fn SubAffixGroup(&self) -> System_UInt32__Boxed {}
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AvatarBaseType {
    Unknown = 0,
    Warrior = 1, // Destruction
    Rogue = 2,   // Hunt
    Mage = 3,    // Erudition
    Shaman = 4,  // Harmony
    Warlock = 5, // Nihility
    Knight = 6,  // Preservation
    Priest = 7,  // Abundance
    Memory = 8,  // Remembrance
    Elation = 9, // Elation
}

#[il2cpp_enum_type(i32)]
pub enum RPG_GameCore_AvatarSkillTreeAnchorType {
    None = 0,
    Point01 = 1, Point02 = 2, Point03 = 3, Point04 = 4, Point05 = 5,
    Point06 = 6, Point07 = 7, Point08 = 8, Point09 = 9, Point10 = 10,
    Point11 = 11, Point12 = 12, Point13 = 13, Point14 = 14, Point15 = 15,
    Point16 = 16, Point17 = 17, Point18 = 18, Point19 = 19, Point20 = 20,
    Point21 = 21, Point22 = 22,
}

#[il2cpp_ref_type("RPG.Client.AvatarSkillTreeData")]
pub struct RPG_Client_AvatarSkillTreeData;
impl RPG_Client_AvatarSkillTreeData {
    #[il2cpp_field(name = "SkillTreeLevels")]
    pub fn SkillTreeLevels(&self) -> System_Object_Dummy {}

    #[il2cpp_field(name = "_PointIDOfAnchorType")]
    pub fn _PointIDOfAnchorType(&self) -> System_Object_Dummy {}
}

#[il2cpp_ref_type("RPG.Client.PlayerModule")]
pub struct RPG_Client_PlayerModule;
impl RPG_Client_PlayerModule {
    #[il2cpp_getter_property(property = "PlayerData")]
    pub fn get_PlayerData(&self) -> RPG_Client_PlayerData {}
}

#[il2cpp_ref_type("RPG.Client.PlayerData")]
pub struct RPG_Client_PlayerData;
impl RPG_Client_PlayerData {
    #[il2cpp_getter_property(property = "UserID")]
    pub fn get_UserID(&self) -> u32 {}

    #[il2cpp_getter_property(property = "NickName")]
    pub fn get_NickName(&self) -> Il2CppString {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarSkillTreeExcelTable")]
pub struct RPG_GameCore_AvatarSkillTreeExcelTable;
impl RPG_GameCore_AvatarSkillTreeExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint", "uint"])]
    pub fn GetData(PointID: u32, Level: u32) -> RPG_GameCore_AvatarSkillTreeRow {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarSkillTreeRow")]
pub struct RPG_GameCore_AvatarSkillTreeRow;
impl RPG_GameCore_AvatarSkillTreeRow {
    #[il2cpp_field(name = "AnchorType")]
    pub fn AnchorType(&self) -> RPG_GameCore_AvatarSkillTreeAnchorType__Boxed {}
}

#[il2cpp_ref_type("RPG.AvatarSystem.IAvatar")]
pub struct RPG_AvatarSystem_IAvatar;

#[il2cpp_ref_type("RPG.Client.AvatarHelper")]
pub struct RPG_Client_AvatarHelper;

impl RPG_Client_AvatarHelper {
    #[il2cpp_method(name = "GetAllObtainedSpecificPathAvatars", args = [])]
    pub fn GetAllObtainedSpecificPathAvatars() -> List {}
}

#[il2cpp_ref_type("RPG.Client.AvatarExtensions")]
pub struct RPG_Client_AvatarExtensions;

impl RPG_Client_AvatarExtensions {
    #[il2cpp_method(name = "GetAvatarID", args = ["RPG.AvatarSystem.IAvatar"])]
    pub fn GetAvatarID(avatar: RPG_AvatarSystem_IAvatar) -> u32 {}

    #[il2cpp_method(name = "GetLevel", args = ["RPG.AvatarSystem.IAvatar"])]
    pub fn GetLevel(avatar: RPG_AvatarSystem_IAvatar) -> u32 {}

    #[il2cpp_method(name = "GetPromotionLevel", args = ["RPG.AvatarSystem.IAvatar"])]
    pub fn GetPromotionLevel(avatar: RPG_AvatarSystem_IAvatar) -> u32 {}

    #[il2cpp_method(name = "GetEidolonLevel", args = ["RPG.AvatarSystem.IAvatar"])]
    pub fn GetEidolonLevel(avatar: RPG_AvatarSystem_IAvatar) -> u32 {}

    #[il2cpp_method(name = "GetEnhancedID", args = ["RPG.AvatarSystem.IAvatar"])]
    pub fn GetEnhancedID(avatar: RPG_AvatarSystem_IAvatar) -> u32 {}

    #[il2cpp_method(name = "GetName", args = ["RPG.AvatarSystem.IAvatar"])]
    pub fn GetName(avatar: RPG_AvatarSystem_IAvatar) -> Il2CppString {}
	
    #[il2cpp_method(name = "GetTraceTreeLevels", args = ["RPG.AvatarSystem.IAvatar"])]
    pub fn GetTraceTreeLevels(avatar: RPG_AvatarSystem_IAvatar) -> *mut std::ffi::c_void {}
}

#[il2cpp_ref_type("System.Object")]
pub struct SystemObjectDummy;

#[il2cpp_ref_type("RPG.GameCore.EquipmentExcelTable")]
pub struct RPG_GameCore_EquipmentExcelTable;
impl RPG_GameCore_EquipmentExcelTable {
    #[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}

    #[il2cpp_method(name = "IsDataLoaded", args = [])]
    pub fn IsDataLoaded() -> bool {}

    #[il2cpp_method(name = "LoadData", args = [])]
    pub fn LoadData() {}
}

#[il2cpp_ref_type("RPG.GameCore.TextmapExcelTable")]
pub struct RPG_GameCore_TextmapExcelTable;
impl RPG_GameCore_TextmapExcelTable {
    #[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}

    #[il2cpp_method(name = "IsDataLoaded", args = [])]
    pub fn IsDataLoaded() -> bool {}

    #[il2cpp_method(name = "LoadData", args = [])]
    pub fn LoadData() {}
	
	#[il2cpp_method(name = "GetLanguage", args = [])]
    pub fn GetLanguage() -> Il2CppString {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicSetConfigExcelTable")]
pub struct RPG_GameCore_RelicSetConfigExcelTable;
impl RPG_GameCore_RelicSetConfigExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint"])]
    pub fn GetData(set_id: u32) -> RPG_GameCore_RelicSetConfigRow {}

    #[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}

    #[il2cpp_method(name = "IsDataLoaded", args = [])]
    pub fn IsDataLoaded() -> bool {}

    #[il2cpp_method(name = "LoadData", args = [])]
    pub fn LoadData() {}
}

#[il2cpp_ref_type("RPG.GameCore.RelicBaseTypeExcelTable")]
pub struct RPG_GameCore_RelicBaseTypeExcelTable;
impl RPG_GameCore_RelicBaseTypeExcelTable {
    #[il2cpp_method(name = "GetData", args = ["RPG.GameCore.RelicType"])]
    pub fn GetData(relic_type: RPG_GameCore_RelicSetType) -> RPG_GameCore_RelicBaseTypeRow {}

    #[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}

    #[il2cpp_method(name = "IsDataLoaded", args = [])]
    pub fn IsDataLoaded() -> bool {}

    #[il2cpp_method(name = "LoadData", args = [])]
    pub fn LoadData() {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarPropertyExcelTable")]
pub struct RPG_GameCore_AvatarPropertyExcelTable;
impl RPG_GameCore_AvatarPropertyExcelTable {
    #[il2cpp_method(name = "GetData", args = ["RPG.GameCore.AvatarPropertyType"])]
    pub fn GetData(property_type: RPG_GameCore_AvatarPropertyType) -> RPG_GameCore_AvatarPropertyRow {}

    #[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}

    #[il2cpp_method(name = "IsDataLoaded", args = [])]
    pub fn IsDataLoaded() -> bool {}

    #[il2cpp_method(name = "LoadData", args = [])]
    pub fn LoadData() {}
}

#[il2cpp_ref_type("RPG.GameCore.AvatarExcelTable")]
pub struct RPG_GameCore_AvatarExcelTable;
impl RPG_GameCore_AvatarExcelTable {
    #[il2cpp_method(name = "GetData", args = ["uint"])]
    pub fn GetData(avatar_id: u32) -> RPG_GameCore_AvatarRow {}
    
    #[il2cpp_method(name = "get_dataDict", args = [])]
    pub fn get_dataDict() -> *mut std::ffi::c_void {}

    #[il2cpp_method(name = "IsDataLoaded", args = [])]
    pub fn IsDataLoaded() -> bool {}

    #[il2cpp_method(name = "LoadData", args = [])]
    pub fn LoadData() {}
}