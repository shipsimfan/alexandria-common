use crate::{Format, Input, Window};

pub trait Shader: Sized {
    type Window<I: Input>: Window<I>;

    fn new<S: AsRef<str>, I: Input>(
        code: S,
        vertex_layout: &[(&str, Format)],
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn set_active(&mut self);
    fn clear_active(&mut self);
}
