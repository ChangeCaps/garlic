use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{Direction, DragArea, DragPosition, Draggable, Droppable, Spacer, Style};

#[derive(Properties, PartialEq)]
pub struct SortableListProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub direction: Direction,
    #[prop_or_else(|| true)]
    pub smooth: bool,
    pub onorder: Option<Callback<Vec<usize>>>,
}

#[function_component]
pub fn SortableList(props: &SortableListProps) -> Html {
    let order = use_mut_ref(Vec::<usize>::new);
    let node_refs = use_mut_ref(Vec::<NodeRef>::new);
    let drag_index = use_state_eq(Option::<usize>::default);
    let hover_index = use_state_eq(Option::<usize>::default);
    let hover_space = use_state_eq(|| 0.0);
    let first = use_mut_ref(|| true);

    let update = use_force_update();
    let swap_order = use_callback(
        move |(index, new_index), (order, onorder)| {
            if index == new_index {
                return;
            }

            let mut order = order.borrow_mut();
            let i = order.remove(index);
            order.insert(new_index, i);

            if let Some(onorder) = onorder {
                onorder.emit(order.clone());
            }

            update.force_update();
        },
        (order.clone(), props.onorder.clone()),
    );

    if order.borrow().len() != props.children.len() {
        let mut order = order.borrow_mut();
        order.clear();

        for i in 0..props.children.len() {
            order.push(i);
        }
    }

    (node_refs.borrow_mut()).resize_with(props.children.len(), NodeRef::default);
    let mut items = Vec::with_capacity(props.children.len());

    let is_first = *first.borrow();
    *first.borrow_mut() = drag_index.is_none();

    let is_smooth = props.smooth && drag_index.is_some() && !is_first;

    let ondragleave = {
        let hover_index = hover_index.clone();

        Callback::from(move |_| {
            hover_index.set(None);
        })
    };

    let spacer_ondrop = {
        let drag_index = drag_index.clone();
        let hover_index = hover_index.clone();
        let swap_order = swap_order.clone();

        Callback::from(move |_| {
            if let (Some(drag_index), Some(hover_index)) = (*drag_index, *hover_index) {
                if hover_index > drag_index {
                    swap_order.emit((drag_index, hover_index - 1));
                } else {
                    swap_order.emit((drag_index, hover_index));
                }
            }

            drag_index.set(None);
            hover_index.set(None);
        })
    };

    for (i, index) in order.borrow().iter().enumerate() {
        let node_ref = node_refs.borrow()[*index].clone();
        let child = props.children.iter().nth(*index).unwrap();

        let onhover = {
            let hover_index = hover_index.clone();

            Callback::from(move |is_before| {
                if is_before {
                    hover_index.set(Some(i));
                } else {
                    hover_index.set(Some(i + 1));
                }
            })
        };

        let ondrag = {
            let drag_index = drag_index.clone();
            let hover_index = hover_index.clone();
            let hover_space = hover_space.clone();
            let direction = props.direction;

            Callback::from(move |event: crate::drag::DragEvent| {
                drag_index.set(Some(i));
                hover_index.set(Some(i + 1));

                let element = event.node_ref.cast::<HtmlElement>().unwrap();
                let size = size(&element, direction);

                hover_space.set(size);
            })
        };

        let spacer_onhover = {
            let hover_index = hover_index.clone();

            Callback::from(move |_| {
                hover_index.set(Some(i));
            })
        };

        let space = if *hover_index == Some(i) {
            *hover_space
        } else {
            0.0
        };

        let item = html! {
            <>
                <Droppable
                    ondrag={ spacer_onhover }
                    ondrop={ spacer_ondrop.clone() }
                    ondragleave={ ondragleave.clone() }
                >
                    <Spacer
                        direction={ props.direction }
                        size={ space }
                        smooth={ is_smooth }
                    />
                </Droppable>
                <Item
                    onhover={ onhover }
                    ondrag={ ondrag }
                    ondrop={ spacer_ondrop.clone() }
                    direction={ props.direction }
                    node_ref={ node_ref.clone() }
                >
                    { child }
                </Item>
            </>
        };

        items.push(item);
    }

    let spacer_onhover = {
        let hover_index = hover_index.clone();
        let i = props.children.len();

        Callback::from(move |_| {
            hover_index.set(Some(i));
        })
    };

    let space = if *hover_index == Some(props.children.len()) {
        *hover_space
    } else {
        0.0
    };

    let mut style = Style::new();
    style.set("display", "flex");
    style.set("flex-direction", props.direction.to_string());

    html! {
        <DragArea>
            <div class="garlic-sortable-list" {style}>
                { for items }

                <Droppable
                    ondrag={ spacer_onhover }
                    ondrop={ spacer_ondrop.clone() }
                >
                    <Spacer
                        direction={ props.direction }
                        size={ space }
                    />
                </Droppable>
            </div>
        </DragArea>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub direction: Direction,
    #[prop_or_default]
    pub smooth: bool,
    #[prop_or_default]
    pub onhover: Callback<bool>,
    #[prop_or_default]
    pub ondrag: Callback<crate::drag::DragEvent>,
    #[prop_or_default]
    pub ondrop: Callback<crate::drag::DragEvent>,
}

fn position(position: DragPosition, direction: Direction) -> f32 {
    match direction {
        Direction::Row => position.x as f32,
        Direction::Column => position.y as f32,
    }
}

fn size(element: &HtmlElement, direction: Direction) -> f32 {
    match direction {
        Direction::Row => element.offset_width() as f32,
        Direction::Column => element.offset_height() as f32,
    }
}

fn middle(elemeent: &HtmlElement, direction: Direction) -> f32 {
    let rect = elemeent.get_bounding_client_rect();

    match direction {
        Direction::Row => rect.x() as f32 + rect.width() as f32 / 2.0,
        Direction::Column => rect.y() as f32 + rect.height() as f32 / 2.0,
    }
}

#[function_component]
pub fn Item(props: &ItemProps) -> Html {
    let node_ref = use_node_ref();

    let onhover = use_callback(
        |event: crate::drag::DragEvent, (node_ref, props)| {
            let droppable = node_ref.cast::<HtmlElement>().unwrap();

            let position = position(event.position, props.direction);
            let middle = middle(&droppable, props.direction);

            let is_before = position < middle;
            props.onhover.emit(is_before);
        },
        (node_ref.clone(), props.clone()),
    );

    let mut style = Style::new();

    style.set("display", "block");

    html! {
        <Droppable
            ondrag={ onhover }
            node_ref={ node_ref.clone() }
        >
            <Draggable
                ondrag={ props.ondrag.clone() }
                ondrop={ props.ondrop.clone() }
                node_ref={ props.node_ref.clone() }
            >
                <div class="garlic-sortable-list-item" style={ style }>
                    { props.children.clone() }
                </div>
            </Draggable>
        </Droppable>
    }
}
