use crate::{Input, Window};
use ginger::Pixel;

pub enum SampleType {
    Point,
    Linear,
}

#[derive(PartialEq, Eq)]
pub enum TextureFormatClass {
    Unsigned8_1,
    Unsigned8_4,
    Unsigned16_1,
    Unsigned32_1,
    Signed8_1,
    Signed16_1,
    Signed32_1,
    Float32_1,
    Float32_4,
}

pub trait TextureFormat {
    const CLASS: TextureFormatClass;
}

pub trait Texture1D<F: TextureFormat>: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        image: &[F],
        width: usize,
        height: usize,
        slot: usize,
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn set_slot(&mut self, new_slot: usize);

    fn set_active(&mut self);
    fn clear_active(&mut self);
}

pub trait Texture2D<F: TextureFormat>: Sized {
    type Window<I: Input>: Window<I>;

    fn new<I: Input>(
        image: &[F],
        width: usize,
        height: usize,
        slot: usize,
        sample_type: SampleType,
        window: &mut Self::Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn set_slot(&mut self, new_slot: usize);

    fn set_active(&mut self);
    fn clear_active(&mut self);

    fn update_region(&mut self, region: UpdateRegion, data: &[F]);
}

#[derive(Clone, Copy)]
pub struct UpdateRegion {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl UpdateRegion {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
        UpdateRegion {
            left,
            top,
            width,
            height,
        }
    }

    pub fn left(&self) -> usize {
        self.left
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl TextureFormat for u8 {
    const CLASS: TextureFormatClass = TextureFormatClass::Unsigned8_1;
}

impl TextureFormat for Pixel<u8> {
    const CLASS: TextureFormatClass = TextureFormatClass::Unsigned8_4;
}

impl TextureFormat for u16 {
    const CLASS: TextureFormatClass = TextureFormatClass::Unsigned16_1;
}

impl TextureFormat for u32 {
    const CLASS: TextureFormatClass = TextureFormatClass::Unsigned32_1;
}

impl TextureFormat for i8 {
    const CLASS: TextureFormatClass = TextureFormatClass::Signed8_1;
}

impl TextureFormat for i16 {
    const CLASS: TextureFormatClass = TextureFormatClass::Signed16_1;
}

impl TextureFormat for i32 {
    const CLASS: TextureFormatClass = TextureFormatClass::Signed32_1;
}

impl TextureFormat for f32 {
    const CLASS: TextureFormatClass = TextureFormatClass::Float32_1;
}

impl TextureFormat for Pixel<f32> {
    const CLASS: TextureFormatClass = TextureFormatClass::Float32_4;
}
