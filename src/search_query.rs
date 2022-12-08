use yew::prelude::*;

use crate::TextInput;

#[derive(Properties, PartialEq)]
pub struct SearchQueryProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    pub query: Option<UseStateHandle<String>>,
    #[prop_or_else(|| Some(String::from("Search")))]
    pub title: Option<String>,
    #[prop_or_else(|| Some(String::from("Search")))]
    pub placeholder: Option<String>,
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
pub fn SearchQuery(props: &SearchQueryProps) -> Html {
    let oninput = use_callback(
        |event: String, (query, oninput)| {
            if let Some(query) = query {
                query.set(event.clone());
            }

            oninput.emit(event);
        },
        (props.query.clone(), props.oninput.clone()),
    );

    let onkeypress = use_callback(
        |event: KeyboardEvent, (query, onkeypress, oninput)| {
            if event.key() == "Escape" {
                if let Some(query) = query {
                    query.set(String::new());
                }

                oninput.emit(String::new());
            }

            onkeypress.emit(event);
        },
        (
            props.query.clone(),
            props.onkeypress.clone(),
            props.oninput.clone(),
        ),
    );

    html! {
        <TextInput
            class={ classes!("garlic-search-query", props.class.clone()) }
            style={ props.style.clone() }
            node_ref={ props.node_ref.clone() }
            value={ props.query.as_ref().map(|q| (**q).clone()) }
            title={ props.title.clone() }
            placeholder={ props.placeholder.clone() }
            oninput={ oninput }
            onkeypress={ onkeypress }
            onkeydown={ props.onkeydown.clone() }
            onkeyup={ props.onkeyup.clone() }
            onfocus={ props.onfocus.clone() }
            onblur={ props.onblur.clone() }
        />
    }
}
