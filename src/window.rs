use crate::{Input, Vector2, ViewportUpdater};

pub trait Window<I: Input>: Sized {
    type Viewport: crate::Viewport<Window<I> = Self>;

    fn new(
        title: &str,
        width: usize,
        height: usize,
        debug_logging: bool,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn poll_events(&mut self) -> bool;

    fn begin_render(&mut self, clear_color: [f32; 4]);
    fn end_render(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn input(&self) -> &I;
    fn input_mut(&mut self) -> &mut I;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn set_debug_logging(&mut self, enable: bool);

    fn size_changed(&self) -> bool;

    fn create_viewport(
        &mut self,
        top_left: Vector2,
        size: Vector2,
        updater: Option<Box<dyn ViewportUpdater>>,
    ) -> usize;
    fn update_viewport(&mut self, viewport: usize, top_left: Vector2, size: Vector2);
    fn set_active_viewport(&mut self, viewport: usize);
    fn remove_viewport(&mut self, viewport: usize);
}
