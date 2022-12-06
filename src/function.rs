use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

pub struct Function {
    pub closure: Closure<dyn FnMut()>,
}

impl Function {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Self {
            closure: Closure::new(f),
        }
    }

    pub fn once<F>(f: F) -> Self
    where
        F: FnOnce() + 'static,
    {
        Self {
            closure: Closure::once(f),
        }
    }

    pub fn into_js(self) -> js_sys::Function {
        self.closure.into_js_value().unchecked_into()
    }
}

impl AsRef<js_sys::Function> for Function {
    fn as_ref(&self) -> &js_sys::Function {
        self.closure.as_ref().unchecked_ref()
    }
}

pub struct AnimationFrame {
    pub function: Function,
}

impl AnimationFrame {
    #[inline]
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce() + 'static,
    {
        Self {
            function: Function::once(f),
        }
    }

    #[inline]
    pub fn request(self) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(&self.function.into_js())
            .expect("garlic failed to request animation frame");
    }
}

#[hook]
pub fn use_animation_frame() -> AnimationFrame {
    let update = use_force_update();
    AnimationFrame::new(move || update.force_update())
}
