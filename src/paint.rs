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

auto!(AutoPaint(paint::Paint) {
    Color => color => paint::Paint::from_color(color.color_u()),
    Paint => paint => paint.into_inner(),
});
