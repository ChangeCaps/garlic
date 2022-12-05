use std::mem;

use crate::function::Function;

#[derive(Debug, PartialEq)]
pub struct Interval {
    id: i32,
}

impl Interval {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let function = Function::new(f);
        let id = web_sys::window()
            .unwrap()
            .set_interval_with_callback(&function.into_js())
            .expect("garlic failed to set interval");

        Self { id }
    }

    pub fn forget(self) {
        mem::forget(self);
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        web_sys::window()
            .unwrap()
            .clear_interval_with_handle(self.id);
    }
}
