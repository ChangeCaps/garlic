use std::rc::Rc;

use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

pub struct Function {
    pub closure: Closure<dyn Fn()>,
}

pub struct UseFunctionHandle {
    pub function: Rc<Function>,
}

impl AsRef<js_sys::Function> for UseFunctionHandle {
    #[inline]
    fn as_ref(&self) -> &js_sys::Function {
        self.function.closure.as_ref().dyn_ref().unwrap()
    }
}

#[hook]
#[inline]
pub fn use_function<F>(f: F) -> UseFunctionHandle
where
    F: Fn() + 'static,
{
    let function = use_memo(
        |_| Function {
            closure: Closure::wrap(Box::new(f) as Box<dyn Fn()>),
        },
        (),
    );

    UseFunctionHandle { function }
}

pub struct AnimationFrame {
    pub function: UseFunctionHandle,
}

impl AnimationFrame {
    #[inline]
    pub fn request(&self) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(self.function.as_ref())
            .expect("garlic failed to request animation frame");
    }
}

#[hook]
#[inline]
pub fn use_animation_frame() -> AnimationFrame {
    let update = use_force_update();
    let function = use_function(move || update.force_update());

    AnimationFrame { function }
}
