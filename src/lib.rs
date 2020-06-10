use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pathfinder_view::{Config};

macro_rules! value_error {
    ($($t:tt)*) => (
        pyo3::exceptions::ValueError::py_err(format!($($t)*))
    )
}

macro_rules! wrap {
    ($name:ident, $inner:ty) => {
        #[pyclass]
        #[derive(Clone)]
        pub struct $name {
            inner: $inner
        }
        impl std::convert::From<$inner> for $name {
            fn from(inner: $inner) -> Self {
                $name { inner }
            }
        }
        impl std::convert::Into<$inner> for $name {
            fn into(self) -> $inner {
                self.inner
            }
        }
        impl std::ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.inner
            }
        }
        impl $name {
            pub fn into_inner(self) -> $inner {
                self.inner
            }
        }
    };
}

macro_rules! auto {
    ($name:ident ($inner:ty)) => {
        pub struct $name($inner);
        impl std::convert::From<$inner> for $name {
            fn from(inner: $inner) -> Self {
                $name(inner)
            }
        }
        impl std::convert::Into<$inner> for $name {
            fn into(self) -> $inner {
                self.0
            }
        }
        impl std::ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.0
            }
        }

    };
}

mod window;
use window::*;

mod canvas;
use canvas::*;

mod color;
use color::*;

mod geometry;
use geometry::*;

mod paint;
use paint::*;

mod path;
use path::*;

mod scene;
use scene::*;

#[pymodule]
/// A Python module implemented in Rust.
fn pathfinder(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Path>()?;
    m.add_class::<Transform>()?;
    m.add_class::<Rect>()?;
    m.add_class::<Vector>()?;
    m.add_class::<Paint>()?;
    m.add_class::<Scene>()?;
    m.add_class::<Color>()?;
    m.add_class::<Window>()?;
    m.add_class::<Canvas>()?;
    m.add_wrapped(wrap_pyfunction!(show)).unwrap();
    
    Ok(())
}

/*
#[pyfunction]
pub fn show(_py: Python, scene: Scene, zoom: bool, pan: bool, level: &str, background_color: Option<Color>) -> PyResult<Window> {
    use pathfinder_renderer::gpu::options::{RendererLevel, RendererOptions};
    let level = match level {
        "d3d9" => RendererLevel::D3D9,
        "d3d11" => RendererLevel::D3D11,
        _ => return Err(value_error!("level must be one of 'd3d9' or 'd3d11'"))
    };
    let options = RendererOptions {
        level,
        background_color: background_color.map(|c| c.color_f())
    };
    Ok(Window::new(scene, Config { zoom, pan, options }))
}
*/

#[pyfunction(scene, zoom="true", pan="true", transparent="false", borders="true", background="Color::white()")]
fn show(_py: Python, scene: AutoScene, zoom: bool, pan: bool, transparent: bool, borders: bool, background: Color) -> PyResult<Window> {
    Ok(Window::new(scene, Config { zoom, pan, transparent, borders, background: background.color_f() }))
}

#[derive(Debug)]
enum Message {
    UpdateScene(SharedScene),
    Close
}
