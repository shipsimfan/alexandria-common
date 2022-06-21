use crate::{Input, Window};

pub trait ConstantBuffer<T>: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        initial_data: T,
        slot: usize,
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn set_data(&mut self, new_data: T) -> Result<(), Box<dyn std::error::Error>>;
    fn set_slot(&mut self, new_slot: usize);

    fn set_active(&mut self);
    fn clear_active(&mut self);
}
