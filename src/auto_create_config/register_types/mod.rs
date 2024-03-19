use std::any::TypeId;

use bevy::reflect::reflect_trait;

pub mod solid_classes;

#[reflect_trait]
pub trait QevyEntityConfig {
    fn get_base_classes(&self) -> Vec<TypeId> {
        vec![]
    }

    fn get_export_string(&self) -> &str;
}
