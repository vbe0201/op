use crate::{r#type::Type};

pub trait PropertyClass: Type + 'static {
    fn property(&self, name: &str) -> Option<&dyn Type>;

    fn property_mut(&mut self, name: &str) -> Option<&mut dyn Type>;
}

pub trait PropertyAs: PropertyClass {
    fn property_as<T: Type>(&self, name: &str) -> Option<&T> {
        self.property(name).and_then(|p| p.downcast_ref())
    }

    fn property_as_mut<T: Type>(&mut self, name: &str) -> Option<&mut T> {
        self.property_mut(name).and_then(|p| p.downcast_mut())
    }
}

impl PropertyAs for dyn PropertyClass {}

impl<P: PropertyClass> PropertyAs for P {}
