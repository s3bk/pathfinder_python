use pyo3::prelude::*;

use pathfinder_renderer::paint;
use crate::{Color};

wrap!(Paint, paint::Paint);

#[pymethods]
impl Paint {
    #[new]
    pub fn new(paint: AutoPaint) -> Paint {
        Paint::from(paint.0)
    }
}

auto!(AutoPaint(paint::Paint));
impl<'s> FromPyObject<'s> for AutoPaint {
    fn extract(ob: &'s PyAny) -> PyResult<Self> {
        if let Ok(color) = Color::extract(ob) {
            Ok(AutoPaint(paint::Paint::from_color(color.color_u())))
        } else {
            Ok(AutoPaint(Paint::extract(ob)?.inner))
        }
    }
}
