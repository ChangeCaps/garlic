use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    pub value: Option<String>,
    pub title: Option<String>,
    #[prop_or_default]
    pub oninput: Callback<String>,
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let oninput = use_callback(
        |_, (node_ref, oninput)| {
            let element = node_ref.cast::<HtmlInputElement>().unwrap();
            oninput.emit(element.value());
        },
        (props.node_ref.clone(), props.oninput.clone()),
    );

    html! {
        <input
            class={ classes!("garlic-text-input", props.class.clone()) }
            style={ props.style.clone() }
            value={ props.value.clone() }
            title={ props.title.clone() }
            type="text"
            oninput={ oninput }
            ref={ props.node_ref.clone() }
        />
    }
}
