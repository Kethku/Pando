use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Blackboard {
    pub lookup: HashMap<TypeId, Box<dyn Any>>,
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, value: T) {
        self.lookup.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.lookup
            .get(&TypeId::of::<T>())
            .map(|value| value.downcast_ref::<T>())
            .flatten()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.lookup
            .get_mut(&TypeId::of::<T>())
            .map(|value| value.downcast_mut::<T>())
            .flatten()
    }

    pub fn get_or_insert_with<T: 'static, F: FnOnce() -> T>(&mut self, f: F) -> &T {
        if !self.contains::<T>() {
            self.insert(f());
        }
        self.get::<T>().unwrap()
    }

    pub fn get_or_default<T: 'static + Default>(&mut self) -> &T {
        self.get_or_insert_with(Default::default)
    }

    pub fn get_mut_or_insert_with<T: 'static, F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
        if !self.contains::<T>() {
            self.insert(f());
        }
        self.get_mut::<T>().unwrap()
    }

    pub fn get_mut_or_default<T: 'static + Default>(&mut self) -> &mut T {
        self.get_mut_or_insert_with(Default::default)
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.lookup
            .remove(&TypeId::of::<T>())
            .map(|value| *value.downcast().unwrap())
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.lookup.contains_key(&TypeId::of::<T>())
    }
}
