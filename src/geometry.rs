use pyo3::prelude::*;
use pathfinder_geometry::{
    vector::Vector2F,
    rect::RectF,
    transform2d::Transform2F
};

wrap!(Vector, Vector2F);

#[pymethods]
impl Vector {
    #[new]
    pub fn new(x: f32, y: f32) -> Vector {
        Vector::from(Vector2F::new(x, y))
    }
}

auto!(AutoVector(Vector2F) {
    (f32, f32) => (x, y) => Vector2F::new(x, y),
    Vector => v => v.into_inner(),
});

auto!(AutoScale(Vector2F) {
    (f32, f32) => (x, y) => Vector2F::new(x, y),
    f32 => x => Vector2F::splat(x),
    Vector => v => v.into_inner(),
});

wrap!(Rect, RectF);

#[pymethods]
impl Rect {
    #[new]
    pub fn new(origin: AutoVector, size: AutoVector) -> Rect {
        Rect::from(RectF::new(*origin, *size))
    }

    #[staticmethod]
    pub fn from_points(origin: AutoVector, lower_right: AutoVector) -> Rect {
        Rect::from(RectF::from_points(*origin, *lower_right))
    }
}

wrap!(Transform, Transform2F);

#[pymethods]
impl Transform {
    #[new]
    pub fn new() -> Transform {
        Transform2F::default().into()
    }

    #[text_signature = "($self, translation: Vector)"]
    pub fn translated(&self, translation: AutoVector) -> Transform {
        Transform::from(Transform2F::from_translation(*translation) * **self)
    }

    #[text_signature = "($self, scale: Vector or float)"]
    pub fn scaled(&self, translation: AutoScale) -> Transform {
        Transform::from(Transform2F::from_translation(*translation) * **self)
    }

    #[text_signature = "($self, rotation: float)"]
    pub fn rotated(&self, rotation: f32) -> Transform {
        Transform::from(Transform2F::from_rotation(rotation) * **self)
    }
}
