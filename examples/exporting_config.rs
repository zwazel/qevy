use bevy::prelude::*;
use bevy_xpbd_3d::{components::LayerMask, plugins::PhysicsPlugins, prelude::PhysicsLayer};
use qevy::{
    auto_create_config::register_types::{
        base_classes::QevyRegisterBaseClass, entities::QevyRegisterSolidClass, QevyEntityConfig,
        QevyEntityType, ReflectQevyEntityConfig,
    },
    CustomPhysicsLayer,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            qevy::MapAssetLoaderPlugin::<Layer>::default(),
            PhysicsPlugins::default(),
            qevy::auto_create_config::AutoCreateConfigPlugin::default(),
        ))
        .register_qevy_entity::<AnotherSolidClass>()
        .register_qevy_entity::<TestSolidClass>()
        .register_qevy_entity::<APointClass>()
        .register_qevy_base_class::<TestBaseClass>()
        .run();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, QevyEntityConfig, Default)]
struct AnotherSolidClass;

impl QevyEntityConfig for AnotherSolidClass {
    fn get_entity_type(&self) -> &QevyEntityType {
        &QevyEntityType::Solid
    }

    fn get_base_classes(&self) -> Vec<std::any::TypeId> {
        vec![std::any::TypeId::of::<TestBaseClass>()]
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, QevyEntityConfig, Default)]
struct APointClass;

impl QevyEntityConfig for APointClass {
    fn get_entity_type(&self) -> &QevyEntityType {
        &QevyEntityType::Point
    }

    fn get_base_classes(&self) -> Vec<std::any::TypeId> {
        vec![std::any::TypeId::of::<TestBaseClass>()]
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, QevyEntityConfig, Default)]
struct TestSolidClass;

impl QevyEntityConfig for TestSolidClass {
    fn get_entity_type(&self) -> &QevyEntityType {
        &QevyEntityType::Solid
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, QevyEntityConfig, Default)]
struct TestBaseClass;

impl QevyEntityConfig for TestBaseClass {
    fn get_entity_type(&self) -> &QevyEntityType {
        &QevyEntityType::Base
    }
}

#[derive(PhysicsLayer, Reflect, Default, Debug)]
enum Layer {
    #[default]
    Ground,
    WallRunnable,
    Player,
    Unknown,
}

impl CustomPhysicsLayer for Layer {
    fn from_flag(flag: u32) -> Self {
        match flag {
            0 => Layer::Ground,
            1 => Layer::WallRunnable,
            2 => Layer::Player,
            _ => Layer::Unknown,
        }
    }

    fn get_default_filters() -> impl Into<LayerMask> {
        [Self::Ground, Self::WallRunnable]
    }

    fn get_default_memberships() -> impl Into<LayerMask> {
        [Self::Ground, Self::WallRunnable]
    }
}
