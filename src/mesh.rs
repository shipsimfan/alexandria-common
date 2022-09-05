use crate::{Input, Window};

pub trait Mesh<V>: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        vertices: &[V],
        indices: &[u32],
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn update_vertices<I: Input>(
        vertices: &[V],
        window: &mut Self::Window<I>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn update_indices<I: Input>(
        indices: &[u32],
        window: &mut Self::Window<I>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    fn render(&mut self);
}

pub trait LineMesh<V>: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        vertices: &[V],
        strip: bool,
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn render(&mut self);
}
