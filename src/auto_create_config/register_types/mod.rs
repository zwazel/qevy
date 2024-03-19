use std::any::{Any, TypeId};

use bevy::{prelude::*, reflect::GetTypeRegistration};

use super::QevyRegistry;

pub trait QevyRegisterSolidClass {
    fn register_solid_class<T: GetTypeRegistration + Any>(&mut self) -> &mut Self;
}

impl QevyRegisterSolidClass for App {
    fn register_solid_class<T: GetTypeRegistration + Any>(&mut self) -> &mut Self {
        let registry = self.world.resource_mut::<AppTypeRegistry>();
        registry.write().register::<T>();

        let mut registry = self.world.resource_mut::<QevyRegistry>();
        registry.solid_classes.push(TypeId::of::<T>());

        self
    }
}
