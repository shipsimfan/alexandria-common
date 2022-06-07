use crate::{Input, Window};
use ginger::Image;

pub trait Texture: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        image: &Image<f32>,
        slot: usize,
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn set_slot(&mut self, new_slot: usize);

    fn set_active(&mut self);
    fn set_active_compute(&mut self);
    fn set_active_compute_rw(&mut self);
}
