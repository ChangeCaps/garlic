use web_sys::HtmlElement;
use yew::prelude::*;

use crate::Interval;

#[derive(Properties, PartialEq)]
pub struct DetectResizeProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub onresize: Callback<()>,
}

#[function_component]
pub fn DetectResize(props: &DetectResizeProps) -> Html {
    let size = use_mut_ref(|| (0, 0));
    let interval = use_state(|| None);

    use_effect_with_deps(
        move |(node_ref, onresize)| {
            let node_ref = node_ref.clone();
            let onresize = onresize.clone();
            let size = size.clone();

            let frame = move || {
                if let Some(element) = node_ref.cast::<HtmlElement>() {
                    let rect = element.get_bounding_client_rect();

                    let (width, height) = (rect.width() as i32, rect.height() as i32);
                    if *size.borrow() != (width, height) {
                        *size.borrow_mut() = (width, height);
                        onresize.emit(());
                    }
                }
            };

            interval.set(Some(Interval::new(frame)));
        },
        (props.node_ref.clone(), props.onresize.clone()),
    );

    Html::default()
}
