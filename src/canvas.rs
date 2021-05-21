use pyo3::prelude::*;
mod pa {
    pub use pathfinder_canvas::{
        Canvas, CanvasRenderingContext2D, Path2D,
        FillStyle, FillRule, CanvasFontContext,
        TextAlign, TextBaseline,
    };
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

use std::sync::Arc;

#[pyclass]
pub struct Canvas {
    inner: pa::CanvasRenderingContext2D
}

#[pymethods]
impl Canvas {
    #[new]
    pub fn new(size: AutoVector) -> Canvas {
        Canvas {
            inner: pa::Canvas::new(*size).get_context_2d(pa::CanvasFontContext::from_system_source())
        }
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
    pub fn get_line_width(&self) -> f32 {
        self.inner.line_width()
    }
    #[setter]
    pub fn set_line_width(&mut self, new_line_width: f32) {
        self.inner.set_line_width(new_line_width);
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
    pub fn get_miter_limit(&self) -> f32 {
        self.inner.miter_limit()
    }
    #[setter]
    pub fn set_miter_limit(&mut self, new_miter_limit: f32) {
        self.inner.set_miter_limit(new_miter_limit);
    }

    #[getter]
    pub fn get_line_dash(&self) -> Vec<f32> {
        self.inner.line_dash().to_owned()
    }
    #[setter]
    pub fn set_line_dash(&mut self, new_line_dash: Vec<f32>) {
        self.inner.set_line_dash(new_line_dash);
    }

    #[getter]
    pub fn get_line_dash_offset(&self) -> f32 {
        self.inner.line_dash_offset()
    }
    #[setter]
    pub fn set_line_dash_offset(&mut self, new_line_dash_offset: f32) {
        self.inner.set_line_dash_offset(new_line_dash_offset);
    }

    #[setter]
    pub fn set_fill_style(&mut self, new_fill_style: AutoFillStyle) {
        self.inner.set_fill_style(new_fill_style.0);
    }

    #[setter]
    pub fn set_stroke_style(&mut self, new_stroke_style: AutoFillStyle) {
        self.inner.set_stroke_style(new_stroke_style.0);
    }

    #[getter]
    pub fn get_shadow_blur(&self) -> f32 {
        self.inner.shadow_blur()
    }
    #[setter]
    pub fn set_shadow_blur(&mut self, new_shadow_blur: f32) {
        self.inner.set_shadow_blur(new_shadow_blur);
    }


    #[getter]
    pub fn get_shadow_colort(&self) -> Color {
        Color::from(self.inner.shadow_color())
    }
    #[setter]
    pub fn set_shadow_color(&mut self, new_shadow_color: Color) {
        self.inner.set_shadow_color(new_shadow_color.color_u());
    }

    #[getter]
    pub fn get_shadow_offset(&self) -> Vector {
        Vector::from(self.inner.shadow_offset())
    }
    #[setter]
    pub fn set_shadow_offset(&mut self, new_shadow_offset: AutoVector) {
        self.inner.set_shadow_offset(*new_shadow_offset);
    }
}

/*
// Text
#[pymethods]
impl Canvas {
    pub fn fill_text(&mut self, string: &str, position: AutoVector) {
        self.inner.fill_text(string, *position);
    }

    pub fn stroke_text(&mut self, string: &str, position: AutoVector) {
        self.inner.stroke_text(string, *position);
    }

    #[getter]
    pub fn get_font_size(&self) -> f32 {
        self.inner.font_size()
    }
    #[setter]
    pub fn set_font_size(&mut self, font_size: f32) {
        self.inner.set_font_size(font_size)
    }
  
    #[getter]
    pub fn get_font(&self) -> FontCollection {
        (*self.inner.font()).clone().into()
    }
    #[setter]
    pub fn set_font(&mut self, font_collection: AutoFontCollection) {
        self.inner.set_font(font_collection.into_inner());
    }

    #[setter]
    pub fn set_text_align(&mut self, align: &str) -> PyResult<()> {
        let align = match align {
            "left" => pa::TextAlign::Left,
            "center"=> pa::TextAlign::Center,
            "right" => pa::TextAlign::Right,
            _ => return Err(value_error!("('left' | 'center' | 'right')")),
        };
        self.inner.set_text_align(align);
        Ok(())
    }
}
*/

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
