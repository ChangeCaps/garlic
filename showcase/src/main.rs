use garlic::*;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <SortableList>
            { "A" }
            { "B" }
            { "C" }
            { "D" }
            { "E" }
        </SortableList>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
