use std::{fs::File, path::Path};

use bevy::{prelude::*, reflect::TypeRegistration};

use super::{AssetRoot, AutoCreateConfigSettings};

pub(crate) fn create_config(world: &mut World) {
    let config = world.resource::<AutoCreateConfigSettings>();
    let asset_root = world.resource::<AssetRoot>();
    let types = world.resource::<AppTypeRegistry>();
    let types = types.read();
    let schemas = types.iter().map(export_type).collect::<Vec<_>>();

    let registry_save_path = Path::join(&asset_root.0, &config.save_path);
    let writer = File::create(registry_save_path).expect("could not create file");
}

fn export_type(reg: &TypeRegistration) -> (String, String) {
    let type_info = reg.type_info();
    let binding = type_info.type_path_table();
    let short_name = binding.short_path();


    println!("{:?}",short_name);
    

    ("".into(), "".into())
}