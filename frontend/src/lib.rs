mod components;

use components::header::Header;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Header />
    }
}
