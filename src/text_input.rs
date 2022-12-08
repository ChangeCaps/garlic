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
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub oninput: Callback<String>,
    #[prop_or_default]
    pub onkeypress: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeyup: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>,
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
            placeholder={ props.placeholder.clone() }
            type="text"
            autofocus={ props.autofocus }
            oninput={ oninput }
            onkeypress={ props.onkeypress.clone() }
            onkeydown={ props.onkeydown.clone() }
            onkeyup={ props.onkeyup.clone() }
            onfocus={ props.onfocus.clone() }
            onblur={ props.onblur.clone() }
            ref={ props.node_ref.clone() }
        />
    }
}
