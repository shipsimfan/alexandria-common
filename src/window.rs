use crate::Input;

pub trait Window<I: Input>: Sized {
    fn new(title: &str, width: usize, height: usize, debug_logging: bool) -> Result<Self, Box<dyn std::error::Error>>;

    fn poll_events(&mut self) -> bool;

    fn begin_render(&mut self, clear_color: [f32; 4]);
    fn end_render(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn input(&self) -> &I;
    fn input_mut(&mut self) -> &mut I;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn set_debug_logging(&mut self, enable: bool);
}
