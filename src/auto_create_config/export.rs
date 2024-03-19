use std::{fs::File, io::Write, path::Path};

use bevy::prelude::*;

use super::{
    register_types::ReflectQevyEntityConfig, AssetRoot, AutoCreateConfigSettings, QevyRegistry,
};

pub(crate) fn create_config(world: &mut World) {
    let config = world.resource::<AutoCreateConfigSettings>();
    let qevy_registry = world.resource::<QevyRegistry>();
    let asset_root = world.resource::<AssetRoot>();
    let types = world.resource::<AppTypeRegistry>();
    let types = types.read();

    let registry_save_path = Path::join(&asset_root.0, &config.save_path);
    let mut writer = File::create(registry_save_path).expect("could not create file");

    let config_type_registrations: Vec<_> = qevy_registry
        .qevy_entities
        .iter()
        .filter_map(|solid_class_type| types.get(*solid_class_type))
        .collect();

    for config_type_registration in config_type_registrations {
        let reflect_default = config_type_registration.data::<ReflectDefault>().unwrap();
        let value: Box<dyn Reflect> = reflect_default.default();
        let reflect_entity_config = config_type_registration
            .data::<ReflectQevyEntityConfig>()
            .unwrap();
        let entity_config = reflect_entity_config.get(&*value).unwrap();
        let config_string =
            entity_config.get_export_string(config_type_registration, &*types) + "\n";

        writer
            .write_all(config_string.as_bytes())
            .expect("could not write to file");
    }
}
