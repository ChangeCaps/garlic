use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{function::AnimationFrame, DetectResize, Direction, DragArea, Draggable, Style};

#[derive(Properties, PartialEq)]
pub struct SortableListProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub direction: Direction,
    #[prop_or_default]
    pub contain: bool,
}

#[function_component]
pub fn SortableList(props: &SortableListProps) -> Html {
    let node_refs = use_mut_ref(Vec::<NodeRef>::new);
    let order = use_mut_ref(Vec::<usize>::new);
    let positions = use_mut_ref(Vec::<(f32, f32)>::new);
    let slide = use_mut_ref(Option::<Slide>::default);

    let drag = use_state_eq(Option::<usize>::default);

    let update = use_force_update();
    let frame = {
        let update = update.clone();
        AnimationFrame::new(move || update.force_update())
    };

    if node_refs.borrow().len() != props.children.len() {
        let mut node_refs = node_refs.borrow_mut();
        for i in node_refs.len()..props.children.len() {
            node_refs.push(NodeRef::default());
            order.borrow_mut().push(i);
            positions.borrow_mut().push((0.0, 0.0));
        }

        update.force_update();
    }

    let drag_size = if let Some(index) = *drag {
        let node_ref = node_refs.borrow()[index].clone();
        let element = node_ref.cast::<HtmlElement>().unwrap();

        match props.direction {
            Direction::Row => element.offset_width() as f32,
            Direction::Column => element.offset_height() as f32,
        }
    } else {
        0.0
    };

    let mut x = 0.0;
    let mut y = 0.0;

    if let Some(element) = props.node_ref.cast::<HtmlElement>() {
        x = element.offset_left() as f32;
        y = element.offset_top() as f32;
    }

    if let Some(slide) = slide.borrow_mut().as_mut() {
        if slide.time > 0.001 {
            slide.time *= 0.8;

            frame.request();
        } else {
            slide.time = 0.0;
        }
    }

    let mut width = 0.0f32;
    let mut height = 0.0f32;
    for (i, &o) in order.borrow().iter().enumerate() {
        if let Some(element) = node_refs.borrow()[o].cast::<HtmlElement>() {
            let rect = element.get_bounding_client_rect();

            match props.direction {
                Direction::Row => {
                    width += rect.width() as f32;
                    height = height.max(rect.height() as f32);
                }
                Direction::Column => {
                    width = width.max(rect.width() as f32);
                    height += rect.height() as f32;
                }
            }

            if let Some(slide) = slide.borrow().as_ref() {
                let mut offset = 0.0;

                if slide.to == i {
                    offset += (1.0 - slide.time) * drag_size;
                }

                if slide.from == i {
                    offset += slide.time * drag_size;
                }

                match props.direction {
                    Direction::Row => x += offset,
                    Direction::Column => y += offset,
                }
            }

            positions.borrow_mut()[o] = (x, y);

            if Some(i) == *drag {
                continue;
            }

            match props.direction {
                Direction::Row => x += rect.width() as f32,
                Direction::Column => y += rect.height() as f32,
            }
        }
    }

    let mut items = Vec::with_capacity(props.children.len());
    for (o, child) in props.children.iter().enumerate() {
        let node_ref = node_refs.borrow()[o].clone();
        let (x, y) = positions.borrow()[o];
        let i = order.borrow().iter().position(|&i| i == o).unwrap();

        let onresize = {
            let update = update.clone();
            Callback::from(move |_| update.force_update())
        };

        let ondrag = {
            let drag = drag.clone();
            let slide = slide.clone();

            Callback::from(move |_| {
                drag.set(Some(i));
                slide.borrow_mut().replace(Slide::new(i));
            })
        };

        let style = Style::new()
            .with("position", "absolute")
            .with("left", format!("{}px", x))
            .with("top", format!("{}px", y));

        let child = html! {
            <Draggable
                class="garlic-list-item"
                style={ style }
                ondrag={ ondrag }
                node_ref={ node_ref.clone() }
            >
                <DetectResize node_ref={ node_ref } onresize={ onresize }/>
                { child }
            </Draggable>
        };

        items.push(child);
    }

    let onmove = {
        let slide = slide.clone();
        let update = update.clone();

        use_callback(
            move |event: crate::drag::DragEvent, (node_refs, order, drag, direction)| {
                let mut index = order.borrow().len();

                for (i, &o) in order.borrow().iter().enumerate() {
                    if Some(i) == **drag {
                        continue;
                    }

                    if let Some(element) = node_refs.borrow()[o].cast::<HtmlElement>() {
                        let rect = element.get_bounding_client_rect();

                        let position = match direction {
                            Direction::Row => event.position.x,
                            Direction::Column => event.position.y,
                        };

                        let middle = match direction {
                            Direction::Row => rect.left() + rect.width() / 2.0,
                            Direction::Column => rect.top() + rect.height() / 2.0,
                        };

                        if (position as f32) < middle as f32 {
                            index = i;
                            break;
                        }
                    }
                }

                let mut slide = slide.borrow_mut();
                let slide = slide.as_mut().unwrap();

                if slide.to != index {
                    slide.slide(index);

                    update.force_update();
                }
            },
            (
                node_refs.clone(),
                order.clone(),
                drag.clone(),
                props.direction,
            ),
        )
    };

    let ondrop = {
        let slide = slide.clone();

        use_callback(
            move |_, (order, drag)| {
                let slide = slide.borrow_mut().take().unwrap();

                let to = if drag.unwrap() < slide.to {
                    slide.to - 1
                } else {
                    slide.to
                };

                let mut order = order.borrow_mut();
                let index = order.remove(drag.unwrap());
                order.insert(to, index);

                drag.set(None);
            },
            (order.clone(), drag.clone()),
        )
    };

    let mut style = Style::new()
        .with("width", format!("{}px", width))
        .with("height", format!("{}px", height));

    style.parse(&props.style);

    html! {
        <DragArea
            class={ classes!("garlic-sortable-list", props.class.clone()) }
            style={ style }
            onmove={ onmove }
            ondrop={ ondrop }
            direction={ props.contain.then_some(props.direction) }
            contain={ props.contain }
            node_ref={ props.node_ref.clone() }
        >
            { for items }
        </DragArea>
    }
}

struct Slide {
    to: usize,
    from: usize,
    time: f32,
}

impl Slide {
    fn new(index: usize) -> Self {
        Self {
            to: index,
            from: index,
            time: 0.0,
        }
    }

    fn slide(&mut self, to: usize) {
        self.from = self.to;
        self.to = to;
        self.time = 1.0;
    }
}
