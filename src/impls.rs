use std::any::Any;

pub use super::{Type, TypeRef, TypeMut};

macro_rules! impl_type_for {
    ($type:ty) => {
        // SAFETY: `Type::any` and `Type::any_mut` implementations both return `self`.
        unsafe impl Type for $type {
            fn any(&self) -> &dyn Any {
                self
            }

            fn any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn type_ref(&self) -> TypeRef<'_> {
                TypeRef::Value(self)
            }

            fn type_mut(&mut self) -> TypeMut<'_> {
                TypeMut::Value(self)
            }

            fn clone_value(&self) -> Box<dyn Type> {
                Box::new(self.clone())
            }

            fn try_set(&mut self, value: Box<dyn Type>) -> Result<(), Box<dyn Type>> {
                *self = *value.downcast()?;
                Ok(())
            }
        }
    };
}

impl_type_for!(bool);
impl_type_for!(i8);
impl_type_for!(u8);
impl_type_for!(i16);
impl_type_for!(u16);
impl_type_for!(i32);
impl_type_for!(u32);
impl_type_for!(i64);
impl_type_for!(u64);
impl_type_for!(f32);
impl_type_for!(f64);
impl_type_for!(isize);
impl_type_for!(usize);
impl_type_for!(i128);
impl_type_for!(u128);
impl_type_for!(String);
