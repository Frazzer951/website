use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
        <div class="container">
            <div class="anchor" id="about" />
            <div class="col">
                <h1>{"About"}</h1>
            </div>
        </div>
    }
}
