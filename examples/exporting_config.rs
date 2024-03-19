use bevy::prelude::*;
use bevy_xpbd_3d::{components::LayerMask, plugins::PhysicsPlugins, prelude::PhysicsLayer};
use qevy::{
    auto_create_config::register_types::{
        solid_classes::{QevyRegisterSolidClass, QevySolidClass},
        QevyEntityConfig, ReflectQevyEntityConfig,
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
        .register_solid_class::<TestSolidClass>()
        .run();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, QevyEntityConfig, Default)]
struct TestSolidClass;

impl QevyEntityConfig for TestSolidClass {
    fn get_export_string(&self) -> &str {
        "TestSolidClass"
    }
}

impl QevySolidClass for TestSolidClass {}

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
