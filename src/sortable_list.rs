use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{Direction, DragArea, Draggable, Droppable, Spacer, Style};

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
    let node_refs = use_mut_ref(Vec::<NodeRef>::new);
    let order = use_mut_ref(Vec::<usize>::new);
    let drag_index = use_state_eq(Option::<usize>::default);
    let is_smooth = use_state_eq(|| false);

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

    for (i, index) in order.borrow().iter().enumerate() {
        let child = props.children.iter().nth(*index).unwrap();
        let node_ref = node_refs.borrow()[i].clone();

        let ondropbefore = {
            let drag_index = drag_index.clone();

            swap_order.reform(move |_| {
                if let Some(index) = *drag_index {
                    drag_index.set(None);

                    if index < i {
                        (index, i - 1)
                    } else {
                        (index, i)
                    }
                } else {
                    (0, 0)
                }
            })
        };

        let ondropafter = {
            let drag_index = drag_index.clone();

            swap_order.reform(move |_| {
                if let Some(index) = *drag_index {
                    drag_index.set(None);

                    if index < i {
                        (index, i)
                    } else {
                        (index, i + 1)
                    }
                } else {
                    (0, 0)
                }
            })
        };

        let ondrag = {
            let drag_index = drag_index.clone();

            Callback::from(move |_| {
                drag_index.set(Some(i));
            })
        };

        let item = html! {
            <Item
                node_ref={ node_ref }
                direction={ props.direction }
                smooth={ *is_smooth && props.smooth }
                ondrag={ ondrag }
                ondropbefore={ ondropbefore }
                ondropafter={ ondropafter }
            >
                { child }
            </Item>
        };

        items.push(item);
    }

    web_sys::console::log_1(&format!("render {}", *is_smooth).into());
    is_smooth.set(drag_index.is_some());

    let mut style = Style::new();
    style.set("display", "flex");
    style.set("flex-direction", props.direction.to_string());

    html! {
        <DragArea>
            <div class="garlic-sortable-list" {style}>
                { for items }
            </div>
        </DragArea>
    }
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    #[prop_or_default]
    node_ref: NodeRef,
    #[prop_or_default]
    children: Children,
    #[prop_or_default]
    smooth: bool,
    #[prop_or_default]
    ondrag: Callback<crate::drag::DragEvent>,
    #[prop_or_default]
    ondropbefore: Callback<crate::drag::DragEvent>,
    #[prop_or_default]
    ondropafter: Callback<crate::drag::DragEvent>,
    #[prop_or_default]
    direction: Direction,
}

#[function_component]
fn Item(props: &ItemProps) -> Html {
    let node_ref = use_node_ref();
    let use_start = use_state_eq(Option::<bool>::default);
    let start_spacing = use_state_eq(|| 0.0);
    let end_spacing = use_state_eq(|| 0.0);

    let ondrag = use_callback(
        |event: crate::drag::DragEvent, (node_ref, start, end, use_start, direction)| {
            let element = event.node_ref.cast::<HtmlElement>().unwrap();

            let size = if direction.is_vertical() {
                element.offset_height()
            } else {
                element.offset_width()
            };

            if **use_start != Some(false) {
                start.set(size as f32);
                end.set(0.0);
            } else {
                start.set(0.0);
                end.set(size as f32);
            }

            let element = node_ref.cast::<HtmlElement>().unwrap();

            let middle = if direction.is_vertical() {
                element.offset_top() + element.offset_height() / 2
            } else {
                element.offset_left() + element.offset_width() / 2
            };

            let pointer = if direction.is_vertical() {
                event.position.y
            } else {
                event.position.x
            };

            if pointer < middle {
                use_start.set(Some(true));
            } else {
                use_start.set(Some(false));
            }
        },
        (
            node_ref.clone(),
            start_spacing.clone(),
            end_spacing.clone(),
            use_start.clone(),
            props.direction,
        ),
    );

    let ondragleave = use_callback(
        |_, (start_spacing, end_spacing, use_start)| {
            start_spacing.set(0.0);
            end_spacing.set(0.0);
            use_start.set(None);
        },
        (
            start_spacing.clone(),
            end_spacing.clone(),
            use_start.clone(),
        ),
    );

    let ondrop = use_callback(
        |event: crate::drag::DragEvent, (ondropbefore, ondropafter, use_start)| {
            if **use_start != Some(false) {
                ondropbefore.emit(event);
            } else {
                ondropafter.emit(event);
            }
        },
        (
            props.ondropbefore.clone(),
            props.ondropafter.clone(),
            use_start.clone(),
        ),
    );

    html! {
        <Droppable
            ondrag={ ondrag }
            ondrop={ ondrop }
            ondragleave={ ondragleave }
            node_ref={ node_ref }
        >
            <Spacer
                size={ *start_spacing }
                direction={ props.direction }
                smooth={ props.smooth }
            />

            <Draggable
                ondrag={ props.ondrag.clone() }
                node_ref={ props.node_ref.clone() }
            >
                { for props.children.iter() }
            </Draggable>

            <Spacer
                size={ *end_spacing }
                direction={ props.direction }
                smooth={ props.smooth }
            />
        </Droppable>
    }
}
