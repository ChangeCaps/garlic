use web_sys::HtmlElement;
use yew::prelude::*;

use crate::Style;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DragPosition {
    pub x: i32,
    pub y: i32,
}

impl DragPosition {
    #[inline]
    pub fn new(event: &MouseEvent) -> Self {
        Self {
            x: event.client_x(),
            y: event.client_y(),
        }
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
pub struct DragBaseProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn DragArea(props: &DragBaseProps) -> Html {
    let dragged = use_state(NodeRef::default);
    let is_dragging = use_state(|| false);
    let position = use_state(DragPosition::default);

    let ondrag = use_callback(
        |event: DragEvent, (dragged, is_dragging, position)| {
            dragged.set(event.node_ref);
            is_dragging.set(true);
            position.set(event.position);
        },
        (dragged.clone(), is_dragging.clone(), position.clone()),
    );

    let onmove = use_callback(
        |new_position: DragPosition, position| {
            position.set(new_position);
        },
        position.clone(),
    );

    let ondrop = use_callback(
        |node: NodeRef, (dragged, is_dragging)| {
            if **dragged == node && **is_dragging {
                is_dragging.set(false);
            }
        },
        (dragged.clone(), is_dragging.clone()),
    );

    let context = DragContext {
        dragged: (*dragged).clone(),
        is_dragging: *is_dragging,
        ondrag,
        onmove,
        ondrop,
    };

    html! {
        <ContextProvider<DragContext> {context}>
            <ContextProvider<DragPosition> context={(*position).clone()}>
                { for props.children.iter() }
            </ContextProvider<DragPosition>>
        </ContextProvider<DragContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct DraggableProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub keep_space: bool,
    #[prop_or_default]
    pub ondrag: Callback<DragEvent>,
}

#[function_component]
pub fn Draggable(props: &DraggableProps) -> Html {
    let context = use_context::<DragContext>().expect("Draggable must be used inside DragBase");
    let position = use_context::<DragPosition>().expect("Draggable must be used inside DragBase");
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
        |event: PointerEvent, (node_ref, context)| {
            if context.is_dragged(&node_ref) {
                event.prevent_default();
                event.stop_propagation();

                context.onmove.emit(DragPosition::new(&event));
            }
        },
        (props.node_ref.clone(), context.clone()),
    );

    let onpointerup = use_callback(
        |event: PointerEvent, (node_ref, context)| {
            if context.is_dragged(&node_ref) {
                event.prevent_default();
                event.stop_propagation();

                context.ondrop.emit(node_ref.clone());
            }
        },
        (props.node_ref.clone(), context.clone()),
    );

    let mut style = Style::new();

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
            class="garlic-draggable"
            {onpointerdown}
            {onpointermove}
            {onpointerup}
            {style}
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

    let left = element.offset_left();
    let top = element.offset_top();
    let right = left + element.offset_width();
    let bottom = top + element.offset_height();

    x > left && x <= right && y > top && y <= bottom
}

#[function_component]
pub fn Droppable(props: &DroppableProps) -> Html {
    let context = use_context::<DragContext>().expect("DropArea must be used inside DragBase");
    let position = use_context::<DragPosition>().expect("DropArea must be used inside DragBase");
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

    html! {
        <div
            class="garlic-drop-area"
            ref={props.node_ref.clone()}
        >
            { for props.children.iter() }
        </div>
    }
}

#[function_component]
pub fn DragTest() -> Html {
    let color = use_state(|| "red".to_string());

    let ondragenter = use_callback(
        |_, color| {
            color.set("blue".to_string());
        },
        color.clone(),
    );

    let ondragleave = use_callback(
        |_, color| {
            color.set("red".to_string());
        },
        color.clone(),
    );

    html! {
        <div class="garlic-drag-test">
            <Droppable {ondragenter} {ondragleave}>
                <div style={format!("color: {}", *color)}>
                    { "Drop here" }
                </div>
            </Droppable>
        </div>
    }
}
