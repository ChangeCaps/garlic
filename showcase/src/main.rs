use garlic::*;
use yew::prelude::*;

#[function_component]
pub fn Button() -> Html {
    let counter = use_state(|| 0);
    let on_click = use_callback(
        move |_, counter| {
            counter.set(**counter + 1);
        },
        counter.clone(),
    );

    html! {
        <div>
            <button onpointerdown={ on_click }>{"Click me!"}</button>
            { *counter }
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <SortableList direction={ Direction::Row }>
                <img src="https://i.imgur.com/u1d1yCc.png"/>
                <img src="https://i.imgur.com/VgiZEvT.png"/>
                <img src="https://i.imgur.com/JQIbF9R.png"/>
                <img src="https://i.imgur.com/nRri93R.png"/>
                <Button/>
                <Button/>
                <Button/>
                <Button/>
            </SortableList>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
