use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{Direction, Style};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DragPosition {
    pub x: i32,
    pub y: i32,
}

impl DragPosition {
    pub fn new(event: &MouseEvent) -> Self {
        Self {
            x: event.client_x(),
            y: event.client_y(),
        }
    }

    pub fn restrained(
        &self,
        mut new_position: Self,
        node_ref: &NodeRef,
        contain: bool,
        direction: Option<Direction>,
    ) -> Self {
        if contain {
            let element = node_ref.cast::<HtmlElement>().unwrap();

            let rect = element.get_bounding_client_rect();

            new_position.x = new_position.x.max(rect.left() as i32);
            new_position.x = new_position.x.min(rect.right() as i32);

            new_position.y = new_position.y.max(rect.top() as i32);
            new_position.y = new_position.y.min(rect.bottom() as i32);
        }

        match direction {
            Some(Direction::Row) => new_position.y = self.y,
            Some(Direction::Column) => new_position.x = self.y,
            _ => {}
        }

        new_position
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DragContext {
    pub dragged: NodeRef,
    pub is_dragging: bool,
    pub ondrag: Callback<DragEvent>,
    pub onmove: Callback<DragPosition>,
    pub ondrop: Callback<NodeRef>,
}

impl DragContext {
    #[inline]
    pub const fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    #[inline]
    pub fn is_dragged(&self, node: &NodeRef) -> bool {
        self.dragged == *node && self.is_dragging()
    }
}

#[doc(hidden)]
#[derive(Properties, PartialEq)]
pub struct DragAreaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
    /// The direction of the drag area.
    ///
    /// If set dragging will only be possible in the specified direction.
    pub direction: Option<Direction>,
    /// If true, dragging outside of the drag area will be prevented.
    #[prop_or_default]
    pub contain: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    /// Called when the drag starts.
    #[prop_or_default]
    pub ondrag: Callback<DragEvent>,
    /// Called when the drag moves.
    #[prop_or_default]
    pub onmove: Callback<DragEvent>,
    /// Called when the drag ends.
    #[prop_or_default]
    pub ondrop: Callback<DragEvent>,
}

#[function_component]
pub fn DragArea(props: &DragAreaProps) -> Html {
    let dragged = use_state(NodeRef::default);
    let is_dragging = use_state(|| false);
    let position = use_state(DragPosition::default);

    let ondrag = use_callback(
        |event: DragEvent, (dragged, is_dragging, position, ondrag)| {
            dragged.set(event.node_ref.clone());
            position.set(event.position);
            is_dragging.set(true);
            ondrag.emit(event);
        },
        (
            dragged.clone(),
            is_dragging.clone(),
            position.clone(),
            props.ondrag.clone(),
        ),
    );

    let onmove = use_callback(
        |new_position: DragPosition, (node_ref, dragged, position, onmove, contain, direction)| {
            let restrained = position.restrained(new_position, node_ref, *contain, *direction);

            position.set(restrained);

            let event = DragEvent {
                node_ref: (**dragged).clone(),
                position: restrained,
            };

            onmove.emit(event);
        },
        (
            props.node_ref.clone(),
            dragged.clone(),
            position.clone(),
            props.onmove.clone(),
            props.contain,
            props.direction,
        ),
    );

    let ondrop = {
        let position = position.clone();

        use_callback(
            move |node: NodeRef, (dragged, is_dragging, ondrop)| {
                if **dragged == node && **is_dragging {
                    is_dragging.set(false);

                    let event = DragEvent {
                        node_ref: (**dragged).clone(),
                        position: *position,
                    };

                    ondrop.emit(event);
                }
            },
            (dragged.clone(), is_dragging.clone(), props.ondrop.clone()),
        )
    };

    let context = DragContext {
        dragged: (*dragged).clone(),
        is_dragging: *is_dragging,
        ondrag,
        onmove,
        ondrop,
    };

    let mut style = Style::new()
        .with("width", "fit-content")
        .with("height", "fit-content");

    style.parse(&props.style);

    html! {
        <div
            class={ classes!("garlic-drag-area", props.class.clone()) }
            style={ style }
            ref={ props.node_ref.clone() }
        >
            <ContextProvider<DragContext> { context }>
                <ContextProvider<DragPosition> context={ *position }>
                    { for props.children.iter() }
                </ContextProvider<DragPosition>>
            </ContextProvider<DragContext>>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DraggableProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub keep_space: bool,
    /// Called when the element is dragged.
    #[prop_or_default]
    pub ondrag: Callback<DragEvent>,
    /// Called when the element is moved while being dragging.
    #[prop_or_default]
    pub onmove: Callback<DragEvent>,
    /// Called when the element is dropped.
    #[prop_or_default]
    pub ondrop: Callback<DragEvent>,
}

#[function_component]
pub fn Draggable(props: &DraggableProps) -> Html {
    let context = use_context::<DragContext>().expect("Draggable must be used inside DragArea");
    let position = use_context::<DragPosition>().expect("Draggable must be used inside DragArea");
    let offset = use_state(DragPosition::default);

    let onpointerdown = use_callback(
        |event: PointerEvent, (node_ref, context, offset, ondrag)| {
            if event.button() == 0 {
                event.prevent_default();
                event.stop_propagation();

                let drag_start = DragEvent {
                    position: DragPosition::new(&event),
                    node_ref: node_ref.clone(),
                };

                context.ondrag.emit(drag_start.clone());
                ondrag.emit(drag_start.clone());

                let element = node_ref.cast::<HtmlElement>().unwrap();
                element.set_pointer_capture(event.pointer_id()).unwrap();

                let x = element.offset_left();
                let y = element.offset_top();

                offset.set(DragPosition {
                    x: drag_start.position.x - x,
                    y: drag_start.position.y - y,
                });
            }
        },
        (
            props.node_ref.clone(),
            context.clone(),
            offset.clone(),
            props.ondrag.clone(),
        ),
    );

    let onpointermove = use_callback(
        |event: PointerEvent, (node_ref, context, onmove)| {
            if context.is_dragged(&node_ref) {
                event.prevent_default();
                event.stop_propagation();

                let position = DragPosition::new(&event);

                let event = DragEvent {
                    position,
                    node_ref: node_ref.clone(),
                };

                context.onmove.emit(position);
                onmove.emit(event);
            }
        },
        (
            props.node_ref.clone(),
            context.clone(),
            props.onmove.clone(),
        ),
    );

    let onpointerup = use_callback(
        |event: PointerEvent, (node_ref, context, ondrop)| {
            if context.is_dragged(&node_ref) {
                event.prevent_default();
                event.stop_propagation();

                let drag_event = DragEvent {
                    position: DragPosition::new(&event),
                    node_ref: node_ref.clone(),
                };

                context.ondrop.emit(node_ref.clone());
                ondrop.emit(drag_event);
            }
        },
        (
            props.node_ref.clone(),
            context.clone(),
            props.ondrop.clone(),
        ),
    );

    let mut style = Style::new();

    style.parse(&props.style);

    if context.is_dragged(&props.node_ref) {
        style.set("position", "absolute");
        style.set("left", format!("{}px", position.x - offset.x));
        style.set("top", format!("{}px", position.y - offset.y));
        style.set("z-index", "1000");
        style.set("cursor", "grabbing");
    } else {
        style.set("cursor", "grab");
    }

    html! {
        <div
            class={ classes!("garlic-draggable", props.class.clone()) }
            onpointerdown={ onpointerdown }
            onpointermove={ onpointermove }
            onpointerup={ onpointerup }
            style={ style }
            ref={props.node_ref.clone()}
        >
            { for props.children.iter() }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DragEvent {
    pub position: DragPosition,
    pub node_ref: NodeRef,
}

#[derive(Clone, Properties, PartialEq)]
pub struct DroppableProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub ondragenter: Callback<DragEvent>,
    #[prop_or_default]
    pub ondragleave: Callback<()>,
    #[prop_or_default]
    pub ondrag: Callback<DragEvent>,
    #[prop_or_default]
    pub ondrop: Callback<DragEvent>,
}

#[inline]
fn is_inside(node_ref: &NodeRef, x: i32, y: i32) -> bool {
    let element = node_ref.cast::<HtmlElement>().unwrap();

    let rect = element.get_bounding_client_rect();

    x >= rect.left() as i32
        && x <= rect.right() as i32
        && y >= rect.top() as i32
        && y <= rect.bottom() as i32
}

#[function_component]
pub fn Droppable(props: &DroppableProps) -> Html {
    let context = use_context::<DragContext>().expect("Droppable must be used inside DragArea");
    let position = use_context::<DragPosition>().expect("Droppable must be used inside DragArea");
    let entered = use_state(|| false);

    use_effect_with_deps(
        |(dragged, is_dragging, position, entered, props)| {
            let event = DragEvent {
                position: position.clone(),
                node_ref: dragged.clone(),
            };

            if *is_dragging {
                let inside = is_inside(&props.node_ref, position.x, position.y);

                if inside {
                    if !**entered {
                        props.ondragenter.emit(event.clone());
                        entered.set(true);
                    }

                    props.ondrag.emit(event);
                } else {
                    if **entered {
                        props.ondragleave.emit(());
                        entered.set(false);
                    }
                }
            } else {
                if **entered {
                    props.ondrop.emit(event);
                    props.ondragleave.emit(());
                    entered.set(false);
                }
            }
        },
        (
            context.dragged.clone(),
            context.is_dragging.clone(),
            position.clone(),
            entered.clone(),
            props.clone(),
        ),
    );

    let mut style = Style::new()
        .with("width", "fit-content")
        .with("height", "fit-content");

    style.parse(&props.style);

    html! {
        <div
            class={ classes!("garlic-droppable", props.class.clone()) }
            style={ style }
            ref={ props.node_ref.clone() }
        >
            { for props.children.iter() }
        </div>
    }
}
