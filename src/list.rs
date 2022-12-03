use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{Direction, DragArea, Draggable, Droppable, Style};

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub direction: Direction,
}

#[function_component]
pub fn List(props: &ListProps) -> Html {
    let mut style = Style::new();

    style.set("display", "flex");

    match props.direction {
        Direction::Column => {
            style.set("flex-direction", "column");
        }
        Direction::Row => {
            style.set("flex-direction", "row");
        }
    }

    let node_refs = use_mut_ref(Vec::<NodeRef>::new);
    let area_refs = use_mut_ref(Vec::<NodeRef>::new);

    let index = use_state_eq(Option::<usize>::default);
    let ondrag = use_callback(
        |new_index: usize, index| {
            index.set(Some(new_index));
        },
        index.clone(),
    );

    use_effect_with_deps(
        |(node_refs, area_refs, index)| {
            if let Some(index) = **index {
                let node_ref = node_refs.borrow()[index].clone();
                let area_ref = area_refs.borrow()[index].clone();

                let node = node_ref.cast::<HtmlElement>();
                let area = area_ref.cast::<HtmlElement>();

                if let (Some(node), Some(area)) = (node, area) {
                    let node_width = node.offset_width();
                    let node_height = node.offset_height();

                    let area_style = area.style();
                    area_style
                        .set_property("width", &format!("{}px", node_width))
                        .unwrap();
                    area_style
                        .set_property("height", &format!("{}px", node_height))
                        .unwrap();
                }
            }
        },
        (node_refs.clone(), area_refs.clone(), index.clone()),
    );

    let mut node_refs = node_refs.borrow_mut();
    let mut area_refs = area_refs.borrow_mut();

    node_refs.resize_with(props.children.len(), Default::default);
    area_refs.resize_with(props.children.len() + 1, Default::default);

    let children = props.children.iter().enumerate().map(|(i, child)| {
        let node_ref = node_refs[i].clone();
        let area_ref = area_refs[i].clone();

        let ondrag = if *index != Some(i) {
            ondrag.reform(move |_| i)
        } else {
            Callback::default()
        };

        html! {
            <>
                <Droppable ondrag={ondrag}>
                    <Draggable node_ref={node_ref}>
                        {child}
                    </Draggable>
                </Droppable>
                <Droppable node_ref={area_ref}/>
            </>
        }
    });

    html! {
        <DragArea>
            <div
                class="garlic-list"
                {style}
            >
                { for children }
            </div>
        </DragArea>
    }
}
