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

auto!(AutoVector(Vector2F));
impl<'s> FromPyObject<'s> for AutoVector {
    fn extract(ob: &'s PyAny) -> PyResult<Self> {
        if let Ok((x, y)) = <(f32, f32)>::extract(ob) {
            Ok(AutoVector(Vector2F::new(x, y)))
        } else {
            Ok(AutoVector(*Vector::extract(ob)?))
        }
    }
}

auto!(AutoScale(Vector2F));
impl<'s> FromPyObject<'s> for AutoScale {
    fn extract(ob: &'s PyAny) -> PyResult<Self> {
        if let Ok((x, y)) = <(f32, f32)>::extract(ob) {
            Ok(AutoScale(Vector2F::new(x, y)))
        } else if let Ok(x) = <f32>::extract(ob) {
            Ok(AutoScale(Vector2F::splat(x)))
        } else {
            Ok(AutoScale(*Vector::extract(ob)?))
        }
    }
}

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
