use auto_create_config::QevyRegistry;
use bevy::asset::io::Reader;
use bevy::asset::AsyncReadExt;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_xpbd_3d::components::{LayerMask, PhysicsLayer};
use std::collections::BTreeMap;
use std::fmt::Debug;
use thiserror::Error;

mod build;
pub mod components;
pub mod conversions;
mod load;
pub mod auto_create_config;

#[derive(Debug, Asset, TypePath)]
pub struct MapAsset {
    geomap: Option<shambler::GeoMap>,
    texture_sizes: BTreeMap<String, (u32, u32)>,
    material_handles: BTreeMap<String, Handle<StandardMaterial>>,
}

impl MapAsset {
    pub fn get_texture_names_with_size(&self) -> BTreeMap<&str, (u32, u32)> {
        let mut names: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
        for (texture_name, (width, height)) in &self.texture_sizes {
            names.insert(texture_name.as_str(), (*width, *height));
        }
        names
    }
}

#[derive(Debug, Error)]
pub enum MapAssetLoaderError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    type Asset = MapAsset;
    type Settings = ();
    type Error = MapAssetLoaderError;
    /// loads the .map file into a MapAsset
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            if let Ok(map) = std::str::from_utf8(&bytes)
                .expect("invalid utf8")
                .parse::<shalrath::repr::Map>()
            {
                let geomap = Some(shambler::GeoMap::new(map.clone()));
                let mut map = MapAsset {
                    geomap: geomap,
                    texture_sizes: BTreeMap::new(),
                    material_handles: BTreeMap::new(),
                };
                load::load_map(&mut map, load_context).await;
                return Ok(map);
            }
            Err(MapAssetLoaderError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid map",
            )))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}

pub trait CustomPhysicsLayer:
    PhysicsLayer + Default + std::marker::Sync + std::marker::Send + Debug + 'static
{
    /// "flag" represents the position of the layer in the bitfield, 0-indexed.
    fn from_flag(flag: u32) -> Self;

    fn get_default_filters() -> impl Into<LayerMask> {
        Self::default()
    }

    fn get_default_memberships() -> impl Into<LayerMask> {
        Self::default()
    }
}

#[derive(Event)]
pub struct PostBuildMapEvent {
    pub map: Entity,
}

#[derive(Default, Resource)]
pub struct PostMapBuildHook {
    pub system: Option<SystemId>,
}

pub struct MapAssetLoaderPlugin<L: CustomPhysicsLayer> {
    _marker: std::marker::PhantomData<L>,
}

impl<L: CustomPhysicsLayer> Default for MapAssetLoaderPlugin<L> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<L: CustomPhysicsLayer> Plugin for MapAssetLoaderPlugin<L> {
    fn build(&self, app: &mut App) {
        app.init_asset::<MapAsset>()
            .init_resource::<PostMapBuildHook>()
            .init_resource::<QevyRegistry>()
            .init_asset_loader::<MapAssetLoader>()
            .add_event::<PostBuildMapEvent>()
            .add_systems(PreUpdate, load::handle_loaded_map_system::<L>);
    }
}
