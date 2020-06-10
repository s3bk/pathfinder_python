use pyo3::prelude::*;

mod pa {
    pub use pathfinder_renderer::scene::{Scene, DrawPath};
}
use crate::{AutoPaint, Path, Rect};

wrap!(Scene, pa::Scene);

#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Scene {
        pa::Scene::new().into()
    }

    #[getter]
    pub fn get_view_box(&self) -> Rect {
        self.inner.view_box().into()
    }
    #[setter]
    pub fn set_view_box(&mut self, r: Rect) {
        self.inner.set_view_box(r.into())
    }

    #[text_signature = "($self, path: Path)"]
    pub fn draw(&mut self, path: Path, paint: AutoPaint) {
        let paint_id = self.inner.push_paint(&paint);
        self.inner.push_draw_path(pa::DrawPath::new(path.into_inner().into_outline(), paint_id));
    }
}