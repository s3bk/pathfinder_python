use pyo3::prelude::*;
mod pa {
    pub use pathfinder_canvas::{Canvas, CanvasRenderingContext2D, Path2D, FillStyle, FillRule, CanvasFontContext, FontCollection};
    pub use pathfinder_geometry::{
        vector::Vector2F,
        rect::RectF,
        transform2d::Transform2F
    };
    pub use pathfinder_content::{
        outline::ArcDirection,
    };
}
use std::sync::Arc;

wrap!(FontCollection, pa::FontCollection);
wrap!(Font, Arc<dyn pdf_font::Font + Sync + Send>);
auto!(AutoFontCollection(Arc<pa::FontCollection>) {
    Font => font => Arc::new(pa::FontCollection::from_font(font.into_inner())),
    Vec<Font> => fonts => Arc::new(pa::FontCollection::from_fonts(fonts.into_iter().map(|f| f.into_inner()).collect())),
    FontCollection => collection => Arc::new(collection.into_inner()),
});

#[pymethods]
impl FontCollection {
    #[staticmethod]
    pub fn from_fonts(fonts: Vec<Font>) -> FontCollection {
        let mut collection = pa::FontCollection::new();
        for font in fonts {
            collection.add_font(font.into_inner());
        }

        collection.into()
    }
}

#[pymethods]
impl Font {
    #[staticmethod]
    pub fn from_file(path: &str) -> PyResult<Font> {
        let data = std::fs::read(path)?;
        let font = pdf_font::parse(&data);
        Ok(Font::from(Arc::from(font)))
    }
}
