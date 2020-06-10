use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod pa {
    pub use pathfinder_canvas::{Canvas, CanvasRenderingContext2D, Path2D, FillStyle, FillRule, CanvasFontContext};
    pub use pathfinder_geometry::{
        vector::Vector2F,
        rect::RectF,
        transform2d::Transform2F
    };
    pub use pathfinder_content::{
        outline::ArcDirection,
    };
}
use crate::{Rect, Vector, Path, AutoScale, AutoVector, Transform, Color};

wrap!(Canvas, pa::CanvasRenderingContext2D);

#[pymethods]
impl Canvas {
    #[new]
    pub fn new(size: AutoVector) -> Canvas {
        pa::Canvas::new(*size).get_context_2d(pa::CanvasFontContext::from_system_source()).into()
    }

    pub fn save(&mut self) {
        self.inner.save();
    }

    pub fn restore(&mut self) {
        self.inner.restore();
    }

    pub fn fill_rect(&mut self, rect: Rect) {
        self.inner.fill_rect(*rect);
    }

    pub fn stroke_rect(&mut self, rect: Rect) {
        self.inner.stroke_rect(*rect);
    }

    pub fn clear_rect(&mut self, rect: Rect) {
        self.inner.clear_rect(*rect);
    }

    pub fn fill_path(&mut self, path: Path, fill_rule: AutoFillRule) {
        self.inner.fill_path(path.into_inner(), *fill_rule);
    }
    pub fn stroke_path(&mut self, path: Path) {
        self.inner.stroke_path(path.into_inner());
    }
    pub fn clip_path(&mut self, path: Path, fill_rule: AutoFillRule) {
        self.inner.clip_path(path.into_inner(), *fill_rule);
    }

    pub fn rotate(&mut self, angle: f32) {
        self.inner.rotate(angle);
    }

    pub fn scale(&mut self, scale: AutoScale) {
        self.inner.scale(*scale);
    }

    pub fn translate(&mut self, offset: AutoVector) {
        self.inner.translate(*offset);
    }

    #[getter]
    pub fn get_transform(&self) -> Transform {
        Transform::from(self.inner.transform())
    }

    #[setter]
    pub fn set_transform(&mut self, transform: Transform) {
        self.inner.set_transform(&*transform);
    }

    pub fn reset_transform(&mut self) {
        self.inner.reset_transform();
    }

    #[getter]
    pub fn get_line_width(&self) -> PyResult<f32> {
        Ok(self.inner.line_width())
    }
    #[setter]
    pub fn set_line_width(&mut self, new_line_width: f32) -> PyResult<()> {
        self.inner.set_line_width(new_line_width);
        Ok(())
    }
/*
    #[getter]
    pub fn get_line_cap(&self) -> PyResult<LineCap> {
        Ok(self.inner
            .line_cap())
    }
    #[setter]
    pub fn set_line_cap(&mut self, new_line_cap: LineCap) -> PyResult<()> {
        self.inner
        .set_line_cap(new_line_cap);
        Ok(())
    }

    #[getter]
    pub fn get_line_join(&self) -> PyResult<LineJoin> {
        Ok(self.inner
            .line_join())
    }
    #[setter]
    pub fn set_line_join(&mut self, new_line_join: LineJoin) -> PyResult<()> {
        self.inner
        .set_line_join(new_line_join);
        Ok(())
    }
    */

    #[getter]
    pub fn get_miter_limit(&self) -> PyResult<f32> {
        Ok(self.inner.miter_limit())
    }
    #[setter]
    pub fn set_miter_limit(&mut self, new_miter_limit: f32) -> PyResult<()> {
        self.inner.set_miter_limit(new_miter_limit);
        Ok(())
    }

    #[getter]
    pub fn get_line_dash(&self) -> PyResult<Vec<f32>> {
        Ok(self.inner.line_dash().to_owned())
    }
    #[setter]
    pub fn set_line_dash(&mut self, new_line_dash: Vec<f32>) -> PyResult<()> {
        self.inner.set_line_dash(new_line_dash);
        Ok(())
    }

    #[getter]
    pub fn get_line_dash_offset(&self) -> PyResult<f32> {
        Ok(self.inner.line_dash_offset())
    }
    #[setter]
    pub fn set_line_dash_offset(&mut self, new_line_dash_offset: f32) -> PyResult<()> {
        self.inner.set_line_dash_offset(new_line_dash_offset);
        Ok(())
    }

    #[setter]
    pub fn set_fill_style(&mut self, new_fill_style: AutoFillStyle) -> PyResult<()> {
        self.inner.set_fill_style(new_fill_style.0);
        Ok(())
    }

    #[setter]
    pub fn set_stroke_style(&mut self, new_stroke_style: AutoFillStyle) -> PyResult<()> {
        self.inner.set_stroke_style(new_stroke_style.0);
        Ok(())
    }

    #[getter]
    pub fn get_shadow_blur(&self) -> PyResult<f32> {
        Ok(self.inner.shadow_blur())
    }
    #[setter]
    pub fn set_shadow_blur(&mut self, new_shadow_blur: f32) -> PyResult<()> {
        self.inner.set_shadow_blur(new_shadow_blur);
        Ok(())
    }


    #[getter]
    pub fn get_shadow_colort(&self) -> PyResult<Color> {
        Ok(Color::from(self.inner.shadow_color()))
    }
    #[setter]
    pub fn set_shadow_color(&mut self, new_shadow_color: Color) -> PyResult<()> {
        self.inner
        .set_shadow_color(new_shadow_color.color_u());
        Ok(())
    }

    #[getter]
    pub fn get_shadow_offset(&self) -> PyResult<Vector> {
        Ok(Vector::from(self.inner
            .shadow_offset()))
    }
    #[setter]
    pub fn set_shadow_offset(&mut self, new_shadow_offset: AutoVector) -> PyResult<()> {
        self.inner
        .set_shadow_offset(*new_shadow_offset);
        Ok(())
    }
}

auto!(AutoFillStyle(pa::FillStyle));
impl<'s> FromPyObject<'s> for AutoFillStyle {
    fn extract(ob: &'s PyAny) -> PyResult<Self> {
        if let Ok(color) = Color::extract(ob) {
            Ok(AutoFillStyle(pa::FillStyle::from(color.color_u())))
        } else {
            Err(value_error!("not a Color"))
        }
    }
}

auto!(AutoFillRule(pa::FillRule));
impl<'s> FromPyObject<'s> for AutoFillRule {
    fn extract(ob: &'s PyAny) -> PyResult<Self> {
        if let Ok(s) = <&str>::extract(ob) {
            match s {
                "even-odd" => Ok(AutoFillRule(pa::FillRule::EvenOdd)),
                "nonzero" | "winding" => Ok(AutoFillRule(pa::FillRule::Winding)),
                _ => Err(value_error!("valid are: 'even-odd' 'nonzero' or 'winding'"))
            }
        } else {
            Err(value_error!("valid are: 'even-odd' 'nonzero' or 'winding'"))
        }
    }
}
