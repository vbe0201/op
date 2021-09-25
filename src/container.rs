use crate::r#type::Type;

// TODO: Iterate over container elements?
pub trait Container: Type + 'static {
    fn get(&self, index: usize) -> Option<&dyn Type>;

    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Type>;

    fn push(&mut self, value: Box<dyn Type>);

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl dyn Container {
    pub fn front(&self) -> Option<&dyn Type> {
        self.get(0)
    }

    pub fn front_mut(&mut self) -> Option<&mut dyn Type> {
        self.get_mut(0)
    }

    pub fn back(&self) -> Option<&dyn Type> {
        self.get(self.len().wrapping_sub(1))
    }

    pub fn back_mut(&mut self) -> Option<&mut dyn Type> {
        self.get_mut(self.len().wrapping_sub(1))
    }
}
