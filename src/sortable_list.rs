use std::{cell::RefCell, rc::Rc};

use web_sys::{DomRect, HtmlElement};
use yew::prelude::*;

use crate::{
    use_animation_frame, AnimationFrame, DetectResize, Direction, DragArea, Draggable, Order, Style,
};

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
    #[prop_or_default]
    pub onorder: Callback<Order>,
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

    fn get_offset(&self, index: usize) -> f32 {
        if index == self.to {
            1.0 - self.time
        } else if index == self.from {
            self.time
        } else {
            0.0
        }
    }

    fn update(&mut self, frame: AnimationFrame) {
        if self.time > 0.001 {
            self.time *= 0.8;

            frame.request();
        } else {
            self.time = 0.0;
        }
    }
}

fn resize_child_state(
    node_refs: &mut Vec<NodeRef>,
    order: &mut Order,
    positions: &mut Vec<(f32, f32)>,
    len: usize,
) -> bool {
    if node_refs.len() == len {
        return false;
    }

    node_refs.resize_with(len, Default::default);
    positions.resize_with(len, Default::default);
    order.resize(len);

    true
}

fn get_drag_size(drag: Option<usize>, node_refs: &[NodeRef], direction: Direction) -> f32 {
    if let Some(index) = drag {
        let node_ref = node_refs[index].clone();
        let element = node_ref.cast::<HtmlElement>().unwrap();

        match direction {
            Direction::Row => element.offset_width() as f32,
            Direction::Column => element.offset_height() as f32,
        }
    } else {
        0.0
    }
}

fn get_offset(node_ref: &NodeRef) -> (f32, f32) {
    if let Some(element) = node_ref.cast::<HtmlElement>() {
        (element.offset_left() as f32, element.offset_top() as f32)
    } else {
        (0.0, 0.0)
    }
}

fn expand_size(width: &mut f32, height: &mut f32, rect: &DomRect, direction: Direction) {
    match direction {
        Direction::Row => {
            *width += rect.width() as f32;
            *height = height.max(rect.height() as f32);
        }
        Direction::Column => {
            *width = width.max(rect.width() as f32);
            *height += rect.height() as f32;
        }
    }
}

fn layout_items(
    order: &Order,
    node_refs: &[NodeRef],
    positions: &mut [(f32, f32)],
    direction: Direction,
    drag: Option<usize>,
    slide: &Option<Slide>,
    props: &SortableListProps,
) -> (f32, f32) {
    let (mut x, mut y) = get_offset(&props.node_ref);
    let drag_size = get_drag_size(drag, node_refs, props.direction);

    let mut width = 0.0f32;
    let mut height = 0.0f32;
    for (i, &o) in order.iter().enumerate() {
        let Some(element) = node_refs[o].cast::<HtmlElement>() else {
            continue;
        };

        let rect = element.get_bounding_client_rect();

        expand_size(&mut width, &mut height, &rect, direction);

        if let Some(slide) = slide {
            let offset = slide.get_offset(i) * drag_size;
            match direction {
                Direction::Row => x += offset,
                Direction::Column => y += offset,
            }
        }

        positions[o] = (x, y);

        if Some(i) == drag {
            continue;
        }

        match direction {
            Direction::Row => x += rect.width() as f32,
            Direction::Column => y += rect.height() as f32,
        }
    }

    (width, height)
}

fn hovered_index(
    event: &crate::drag::DragEvent,
    node_refs: &[NodeRef],
    order: &Order,
    drag: Option<usize>,
    direction: Direction,
) -> usize {
    let mut index = order.len();

    for (i, &o) in order.iter().enumerate() {
        if Some(i) == drag {
            continue;
        }

        let Some(element) = node_refs[o].cast::<HtmlElement>() else {
            continue;
        };

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

    index
}

fn render_items(
    node_refs: &[NodeRef],
    positions: &[(f32, f32)],
    order: &Order,
    update: &UseForceUpdateHandle,
    drag: &UseStateHandle<Option<usize>>,
    slide: &Rc<RefCell<Option<Slide>>>,
    props: &SortableListProps,
) -> Vec<Html> {
    let mut items = Vec::with_capacity(props.children.len());
    for (o, child) in props.children.iter().enumerate() {
        let node_ref = node_refs[o].clone();
        let (x, y) = positions[o];
        let i = order.iter().position(|&i| i == o).unwrap();

        let update = update.clone();
        let onresize = Callback::from(move |_| update.force_update());

        let drag = drag.clone();
        let slide = slide.clone();

        let ondrag = Callback::from(move |_| {
            drag.set(Some(i));
            slide.borrow_mut().replace(Slide::new(i));
        });

        let style = Style::new()
            .with("position", "absolute")
            .with("user-select", "none")
            .with("-webkit-user-select", "none")
            .with("-moz-user-select", "none")
            .with("-ms-user-select", "none")
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
    items
}

#[function_component]
pub fn SortableList(props: &SortableListProps) -> Html {
    let node_refs = use_mut_ref(Vec::<NodeRef>::new);
    let order = use_mut_ref(Order::new);
    let positions = use_mut_ref(Vec::<(f32, f32)>::new);
    let slide = use_mut_ref(Option::<Slide>::default);

    let drag = use_state_eq(Option::<usize>::default);

    let update = use_force_update();
    let frame = use_animation_frame();

    if resize_child_state(
        &mut node_refs.borrow_mut(),
        &mut order.borrow_mut(),
        &mut positions.borrow_mut(),
        props.children.len(),
    ) {
        update.force_update();
    }

    if let Some(slide) = slide.borrow_mut().as_mut() {
        slide.update(frame);
    }

    let (width, height) = layout_items(
        &order.borrow(),
        &node_refs.borrow(),
        &mut positions.borrow_mut(),
        props.direction,
        *drag,
        &slide.borrow(),
        props,
    );

    let items = render_items(
        &node_refs.borrow(),
        &positions.borrow(),
        &order.borrow(),
        &update,
        &drag,
        &slide,
        props,
    );

    let onmove = {
        let slide = slide.clone();

        use_callback(
            move |event: crate::drag::DragEvent, (node_refs, order, drag, direction)| {
                let index = hovered_index(
                    &event,
                    &node_refs.borrow(),
                    &order.borrow(),
                    **drag,
                    *direction,
                );

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

    let ondrop = use_callback(
        move |_, (order, drag, onorder)| {
            let slide = slide.borrow_mut().take().unwrap();

            let mut order = order.borrow_mut();
            order.swap_move(drag.unwrap(), slide.to);

            onorder.emit(order.clone());

            drag.set(None);
        },
        (order.clone(), drag.clone(), props.onorder.clone()),
    );

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
