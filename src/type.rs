use std::any::Any;

use crate::{class::PropertyClass, container::Container};

// TODO: Enum? How to deal with generic #{repr()]?
pub enum TypeRef<'ty> {
    Class(&'ty dyn PropertyClass),
    Container(&'ty dyn Container),
    Value(&'ty dyn Type),
}

pub enum TypeMut<'ty> {
    Class(&'ty mut dyn PropertyClass),
    Container(&'ty mut dyn Container),
    Value(&'ty mut dyn Type),
}

// SAFETY: `any` and `any_mut` must always return `self`.
pub unsafe trait Type: Any + Sync + Send {
    fn any(&self) -> &dyn Any;

    fn any_mut(&mut self) -> &mut dyn Any;

    fn type_ref(&self) -> TypeRef<'_>;

    fn type_mut(&mut self) -> TypeMut<'_>;

    fn clone_value(&self) -> Box<dyn Type>;

    fn try_set(&mut self, value: Box<dyn Type>) -> Result<(), Box<dyn Type>>;
}

impl dyn Type {
    pub fn is<T: Type>(&self) -> bool {
        self.any().is::<T>()
    }

    pub fn downcast<T: Type>(self: Box<dyn Type>) -> Result<Box<T>, Box<dyn Type>> {
        match self.is::<T>() {
            true => unsafe {
                let ptr = Box::into_raw(self);
                Ok(Box::from_raw(ptr as *mut T))
            },
            false => Err(self),
        }
    }

    pub fn downcast_ref<T: Type>(&self) -> Option<&T> {
        self.any().downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Type>(&mut self) -> Option<&mut T> {
        self.any_mut().downcast_mut::<T>()
    }
}

impl Clone for Box<dyn Type> {
    fn clone(&self) -> Self {
        self.clone_value()
    }
}

// TODO: PartialEq, Debug, Display?
