use yew::prelude::*;

use crate::{function::use_animation_frame, Direction, Style};

#[derive(Properties, PartialEq)]
pub struct SpacerProps {
    #[prop_or_default]
    pub size: f32,
    #[prop_or_default]
    pub direction: Direction,
    #[prop_or_default]
    pub smooth: bool,
}

#[function_component]
pub fn Spacer(props: &SpacerProps) -> Html {
    let size = use_mut_ref(|| props.size);

    let animation_frame = use_animation_frame();
    if f32::abs(*size.borrow() - props.size) > 0.1 {
        if props.smooth {
            let mut size = size.borrow_mut();
            let diff = props.size - *size;
            *size += diff / 2.0;

            animation_frame.request();
        } else {
            *size.borrow_mut() = props.size;
        }
    } else if *size.borrow() != props.size {
        *size.borrow_mut() = props.size;
    }

    let mut style = Style::new();

    let size = size.borrow();
    if props.direction.is_vertical() {
        style.set("height", &format!("{}px", size));
    } else {
        style.set("width", &format!("{}px", size));
    }

    html! {
        <div class="garlic-spacer" {style}>
        </div>
    }
}
