use std::{fs::File, path::Path};

use bevy::{prelude::*, reflect::TypeRegistration};

use super::{AssetRoot, AutoCreateConfigSettings, QevyRegistry};

pub(crate) fn create_config(world: &mut World) {
    let config = world.resource::<AutoCreateConfigSettings>();
    let qevy_registry = world.resource::<QevyRegistry>();
    let asset_root = world.resource::<AssetRoot>();
    let types = world.resource::<AppTypeRegistry>();
    let types = types.read();

    let schemas: Vec<_> = qevy_registry
        .solid_classes
        .iter()
        .filter_map(|solid_class_type| {
            types
                .get(*solid_class_type)
                .map(|schema| export_type(schema))
        })
        .collect();

    let registry_save_path = Path::join(&asset_root.0, &config.save_path);
    let writer = File::create(registry_save_path).expect("could not create file");
}

fn export_type(reg: &TypeRegistration) -> (String, String) {
    let type_info = reg.type_info();
    let binding = type_info.type_path_table();
    let short_name = binding.short_path();

    println!("{:?}", short_name);

    ("".into(), "".into())
}
