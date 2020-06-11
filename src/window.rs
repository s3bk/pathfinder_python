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
use std::sync::Arc;
use pathfinder_view::{Interactive, Emitter, Context, Config, ElementState, KeyEvent, KeyCode, DEFAULT_SCALE};

use crate::{Message, Scene, Canvas};

auto!(AutoScene(pa::Scene) {
    Canvas => canvas => canvas.into_inner().into_canvas().into_scene(),
    Scene => scene => scene.into_inner(),
});

struct WindowInner {
    sender: Emitter<Message>,
    thread: Option<JoinHandle<()>>,
}

#[pyclass]
#[derive(Clone)]
pub struct Window {
    inner: Arc<WindowInner>
}
#[pymethods]
impl Window {
    #[text_signature = "($self, scene: Scene)"]
    pub fn update(&mut self, scene: AutoScene) {
        self.inner.sender.send(Message::UpdateScene(SharedScene(scene.into())));
    }

    #[text_signature = "($self, scene: Scene)"]
    pub fn close(&mut self) {
        self.inner.sender.send(Message::Close);
    }
}
impl Window {
    pub fn new(scene: AutoScene, config: Config) -> Window{
        let (tx, rx) = channel();
        let view = View { scene: SharedScene(scene.into()), _winit_is_broken: tx };
        let thread = std::thread::spawn(move || pathfinder_view::show(view, config));
        let sender = rx.recv().unwrap();

        Window {
            inner: Arc::new(WindowInner { sender, thread: Some(thread) })
        }
    }
}
impl Drop for WindowInner {
    fn drop(&mut self) {
        self.sender.send(Message::Close);
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

    fn scene(&mut self, nr: usize) -> pa::Scene { 
        self.scene.0.clone()
    }
    fn num_pages(&self) -> usize { 1 }

    fn char_input(&mut self, ctx: &mut Context, input: char) {}
    fn text_input(&mut self, ctx: &mut Context, input: String) {}
    fn keyboard_input(&mut self, ctx: &mut Context, event: &mut KeyEvent) {
        match (event.state, event.modifiers.ctrl, event.keycode) {
            (ElementState::Pressed, true, KeyCode::Add) => ctx.zoom_by(0.2),
            (ElementState::Pressed, true, KeyCode::Subtract) => ctx.zoom_by(-0.2),
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
                ctx.update_scene();
            }
            Message::Close => ctx.close(),
        }
    }
    fn init(&mut self, ctx: &mut Context, emitter: Emitter<Message>) {
        self._winit_is_broken.send(emitter).unwrap();
    }
    fn idle(&mut self, ctx: &mut Context) {}
}
