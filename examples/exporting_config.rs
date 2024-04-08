use bevy::prelude::*;
use qevy::auto_create_config::register_types::{
    entities::QevyRegisterSolidClass,
    properties::{QevyProperty, ReflectQevyProperty},
    QevyEntity, ReflectQevyEntity,
};
use qevy_derive::QevyEntity;
use bevy::reflect::ReflectMut;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            qevy::MapAssetLoaderPlugin::default(),
            qevy::auto_create_config::AutoCreateConfigPlugin::default(),
        ))
        .register_qevy_entity::<TestSolidClass>()
        .register_qevy_entity::<APointClass>()
        .register_qevy_entity::<TestBaseClass>()
        .register_type::<EnumTestFlag>()
        .register_type::<EnumTestChoices>()
        .run();
}

/// This is a simple testing class, showcasing the different property types.
#[derive(Reflect, QevyEntity)]
#[reflect(QevyEntity, Default)]
#[qevy_entity(entity_type = "Point")]
struct APointClass {
    /// odwoidiawdiwj
    test_string: String,
    test_usize: usize,
    test_bool: bool,
    test_f32: f32,
    test_f64: f64,
    test_i32: i32,
    test_i64: i64,
    test_u32: u32,
    test_u64: u64,
    test_flag: EnumTestFlag,
    test_choices: EnumTestChoices,
    // this is a base class, and won't be included in the fgd!
    test_base_class: TestBaseClass,
}

impl Default for APointClass {
    fn default() -> Self {
        Self {
            test_string: "HELLO WORLD!".to_string(),
            test_usize: 69,
            test_bool: true,
            test_f32: 69.420,
            test_f64: 420.69,
            test_i32: Default::default(),
            test_i64: Default::default(),
            test_u32: Default::default(),
            test_u64: Default::default(),
            test_flag: EnumTestFlag::EnumVariantTest,
            test_choices: EnumTestChoices::EnumVariantTest,
            test_base_class: TestBaseClass,
        }
    }
}

#[derive(Reflect, Default, QevyProperty)]
#[reflect(QevyProperty, Default)]
#[qevy_property(property_type = "flags")]
enum EnumTestFlag {
    #[default]
    #[qevy_property(selected_by_default = true)]
    Test,
    #[qevy_property(selected_by_default = false)]
    EnumVariantTest,
    TestTestTest,
}

#[derive(Reflect, Default, QevyProperty)]
#[reflect(QevyProperty, Default)]
#[qevy_property(property_type = "choices")]
enum EnumTestChoices {
    #[default]
    #[qevy_property(selected_by_default = true)]
    Test,
    #[qevy_property(key_override = "VariantTest!!!!")]
    EnumVariantTest,
    AnotherEnumVariant,
}

#[derive(Reflect, Default, QevyEntity)]
#[reflect(QevyEntity, Default)]
#[qevy_entity(entity_type = "Solid")]
struct TestSolidClass;

#[derive(Reflect, Default, QevyEntity)]
#[reflect(QevyEntity, Default)]
#[qevy_entity(entity_type = "Base")]
struct TestBaseClass;
