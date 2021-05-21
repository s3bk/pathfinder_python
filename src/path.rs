use pyo3::prelude::*;
use pyo3::{
    types::PyString,
    class::PyObjectProtocol,
};

use pathfinder_canvas::Path2D;
use pathfinder_content::{
    outline::ArcDirection,
};
use crate::{AutoVector, Transform, Rect};

wrap!(Path, Path2D);

#[pymethods]
impl Path {
    #[new]
    pub fn new() -> Path {
        Path::from(Path2D::new())
    }

    #[text_signature = "($self)"]
    pub fn close(&mut self) {
        self.inner.close_path();
    }

    #[text_signature = "($self, to: Vector)"]
    pub fn move_to(&mut self, to: AutoVector) {
        self.inner.move_to(*to);
    }

    #[text_signature = "($self, to: Vector)"]
    pub fn line_to(&mut self, to: AutoVector) {
        self.inner.line_to(*to);
    }

    #[text_signature = "($self, ctrl: Vector, to: Vector)"]
    pub fn quadratic_curve_to(&mut self, ctrl: AutoVector, to: AutoVector) {
        self.inner.quadratic_curve_to(*ctrl, *to);
    }

    #[text_signature = "($self, ctrl0: Vector, ctrl1: Vector, to: Vector)"]
    pub fn bezier_curve_to(&mut self, ctrl0: AutoVector, ctrl1: AutoVector, to: AutoVector) {
        self.inner.bezier_curve_to(*ctrl0, *ctrl1, *to);
    }

    #[text_signature = "($self, center: Vector, radius: float, start_angle: float, end_angle: float, clockwise: bool)"]
    pub fn arc(&mut self, center: AutoVector, radius: f32, start_angle: f32, end_angle: f32, clockwise: bool) {
        let direction = match clockwise {
            false => ArcDirection::CCW,
            true => ArcDirection::CW
        };
        self.inner.arc(*center, radius, start_angle, end_angle, direction);
    }

    #[text_signature = "($self, center: Vector, radius: float)"]
    pub fn circle(&mut self, center: AutoVector, radius: f32) {
        self.inner.arc(*center, radius, 0., 2.0 * std::f32::consts::PI, ArcDirection::CCW);
    }

    #[text_signature = "($self, ctrl: Vector, to: Vector, radius: float)"]
    pub fn arc_to(&mut self, ctrl: AutoVector, to: AutoVector, radius: f32) {
        self.inner.arc_to(*ctrl, *to, radius);
    }

    #[text_signature = "($self, rect: Rect)"]
    pub fn rect(&mut self, rect: Rect) {
        self.inner.rect(*rect);
    }

    #[text_signature = "($self, center: Vector, axes: Vector, rotation: float, start_angle: float, end_angle: float, clockwise: bool)"]
    pub fn ellipse(&mut self, center: AutoVector, axes: AutoVector, rotation: f32) {
        self.inner.ellipse(*center, *axes, rotation, 0.0, 2.0 * std::f32::consts::PI);
    }

    #[text_signature = "($self, center: Vector, axes: Vector, rotation: float, start_angle: float, end_angle: float)"]
    pub fn ellipse_section(&mut self, center: AutoVector, axes: AutoVector, rotation: f32, start_angle: f32, end_angle: f32) {
        self.inner.ellipse(*center, *axes, rotation, start_angle, end_angle);
    }

    #[text_signature = "($self, rect: Rect, transform: Transform)"]
    pub fn add_path(&mut self, path: Path, transform: &Transform) {
        self.inner.add_path(path.inner, &**transform)
    }
}
#[pyproto]
impl PyObjectProtocol for Path {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }
}
