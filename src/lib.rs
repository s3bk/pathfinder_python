use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyList;
use pathfinder_view::{Config};
use std::marker::PhantomData;

macro_rules! value_error {
    ($($t:tt)*) => (
        pyo3::exceptions::PyValueError::new_err(format!($($t)*))
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
    ($name:ident ($inner:ty) $({ $t0:ty => $p0:pat => $e0:expr, $($t:ty => $p:pat => $e:expr, )* })*) => {
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
        impl $name {
            pub fn into_inner(self) -> $inner {
                self.0
            }
        }

        $(
            impl<'s> FromPyObject<'s> for $name {
                fn extract(ob: &'s PyAny) -> PyResult<Self> {
                    if let Ok($p0) = <$t0>::extract(ob) {
                        Ok($name($e0))
                    }
                    $(else if let Ok($p) = <$t>::extract(ob) {
                        Ok($name($e))
                    })* else {
                        Err(value_error!(concat!(
                            "expected (",
                            stringify!($t0),
                            $(" | ", stringify!($t)),*
                        )))
                    }
                }
            }
        )*
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

//mod font;
//use font::*;


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
//    m.add_class::<Font>()?;
//    m.add_class::<FontCollection>()?;
    m.add_wrapped(wrap_pyfunction!(show)).unwrap();
    
    Ok(())
}


#[derive(Debug)]
enum Message {
    UpdateScene(SharedScene),
    Close
}
