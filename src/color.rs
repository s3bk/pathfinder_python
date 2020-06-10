use pyo3::prelude::*;

use pathfinder_color::{ColorF, ColorU};

#[pyclass]
#[derive(Clone)]
pub struct Color {
    rgba: (f32, f32, f32, f32)
}

#[pymethods]
impl Color {
    #[staticmethod]
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { rgba: (r, g, b, a) }
    }
}

impl Color {
    pub fn color_u(&self) -> ColorU {
        self.color_f().to_u8()
    }
    pub fn color_f(&self) -> ColorF {
        let (r, g, b, a) = self.rgba;
        ColorF::new(r, g, b, a)
    }
    pub fn white() -> Color {
        ColorF::white().into()
    }
}

impl From<ColorU> for Color {
    fn from(color: ColorU) -> Self {
        color.to_f32().into()
    }
}
impl From<ColorF> for Color {
    fn from(c: ColorF) -> Self {
        Color {
            rgba: (c.r(), c.g(), c.b(), c.a())
        }
    }
}
