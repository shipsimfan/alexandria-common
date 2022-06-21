use crate::{Input, Window};
use ginger::Image;

pub trait Texture1D: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        image: &[f32],
        slot: usize,
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn set_slot(&mut self, new_slot: usize);

    fn set_active(&mut self);
    fn clear_active(&mut self);
}

pub trait Texture2D: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        image: &Image<f32>,
        slot: usize,
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn set_slot(&mut self, new_slot: usize);

    fn set_active(&mut self);
    fn clear_active(&mut self);
}
