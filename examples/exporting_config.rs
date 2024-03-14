use bevy::prelude::*;
use bevy_xpbd_3d::{components::LayerMask, plugins::PhysicsPlugins, prelude::PhysicsLayer};
use qevy::CustomPhysicsLayer;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            qevy::MapAssetLoaderPlugin::<Layer>::default(),
            PhysicsPlugins::default(),
            qevy::auto_create_config::AutoCreateConfigPlugin::default(),
        ))
        .run();
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
