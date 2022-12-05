use garlic::*;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <SortableList direction={ Direction::Row } contain=true>
            <img src="https://i.imgur.com/u1d1yCc.png"/>
            <img src="https://i.imgur.com/VgiZEvT.png"/>
            <img src="https://i.imgur.com/JQIbF9R.png"/>
            <img src="https://i.imgur.com/nRri93R.png"/>
        </SortableList>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
