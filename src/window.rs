use pyo3::prelude::*;

mod pa {
    pub use pathfinder_geometry::{
        vector::Vector2F,
    };
    pub use pathfinder_renderer::{
        scene::{Scene, DrawPath}
    };
}
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use pathfinder_view::{Interactive, Emitter, Context, Config, ElementState, KeyEvent, KeyCode, DEFAULT_SCALE};

use pathfinder_resources::embedded::EmbeddedResourceLoader;
use pathfinder_renderer::{
    gpu::options::RendererLevel
};

use crate::{Message, Scene, Canvas, Color};

struct WindowInner {
    sender: Mutex<Emitter<Message>>,
    thread: Option<JoinHandle<()>>,
}
impl WindowInner {
    fn send(&self, msg: Message) {
        self.sender.lock().unwrap().send(msg);
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Window {
    inner: Arc<WindowInner>
}
#[pymethods]
impl Window {
    #[text_signature = "($self, scene: Scene)"]
    pub fn update(&mut self, scene: Scene) {
        self.inner.send(Message::UpdateScene(SharedScene(scene.into())));
    }

    #[text_signature = "($self, scene: Scene)"]
    pub fn close(&mut self) {
        self.inner.send(Message::Close);
    }
}

#[pyfunction(scene, zoom="true", pan="true", transparent="false", borders="true", background="Color::white()")]
fn show(_py: Python, scene: Scene, zoom: bool, pan: bool, transparent: bool, borders: bool, background: Color) -> PyResult<Window> {
    let (tx, rx) = channel();
    let thread = std::thread::spawn(move || {
        let view = View { scene: SharedScene(scene.into()), _winit_is_broken: tx };
        let config = Config {
            zoom,
            pan,
            transparent,
            borders,
            background: background.color_f(),
            resource_loader: Box::new(EmbeddedResourceLoader),
            render_level: RendererLevel::D3D9
        };
        pathfinder_view::show(view, config)
    });
    let sender = rx.recv().unwrap();

    Ok(Window {
        inner: Arc::new(WindowInner { sender: Mutex::new(sender), thread: Some(thread) })
    })
}

impl Drop for WindowInner {
    fn drop(&mut self) {
        self.send(Message::Close);
        self.thread.take().unwrap().join();
    }
}

use std::fmt;
pub struct SharedScene(pa::Scene);
impl fmt::Debug for SharedScene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SharedScene")
    }
}

struct View {
    scene: SharedScene,
    _winit_is_broken: Sender<Emitter<Message>>,
}
impl Interactive for View {
    type Event = Message;

    fn scene(&mut self, ctx: &mut Context) -> pa::Scene { 
        self.scene.0.clone()
    }
    fn char_input(&mut self, ctx: &mut Context, input: char) {}
    fn text_input(&mut self, ctx: &mut Context, input: String) {}
    fn keyboard_input(&mut self, ctx: &mut Context, event: &mut KeyEvent) {
        match (event.state, event.modifiers.ctrl, event.keycode) {
            (ElementState::Pressed, true, KeyCode::Plus) => ctx.zoom_by(0.2),
            (ElementState::Pressed, true, KeyCode::Minus) => ctx.zoom_by(-0.2),
            (ElementState::Pressed, true, KeyCode::Key0) => ctx.set_zoom(DEFAULT_SCALE),
            (ElementState::Pressed, true, KeyCode::Q) => ctx.close(),
            (ElementState::Pressed, _, KeyCode::Escape) => ctx.close(),
            _ => return
        }
        event.cancel();
    }
    fn mouse_input(&mut self, ctx: &mut Context, page: usize, pos: pa::Vector2F, state: ElementState) {}
    fn exit(&mut self, ctx: &mut Context) {}
    fn title(&self) -> String { "A fantastic window!".into() }
    fn event(&mut self, ctx: &mut Context, event: Self::Event) {
        match event {
            Message::UpdateScene(scene) => {
                self.scene = scene;
                ctx.request_redraw();
            }
            Message::Close => ctx.close(),
        }
    }
    fn init(&mut self, ctx: &mut Context, emitter: Emitter<Message>) {
        self._winit_is_broken.send(emitter).unwrap();
    }
    fn idle(&mut self, ctx: &mut Context) {}
}
