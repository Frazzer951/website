mod components;

use yew::prelude::*;

use crate::components::{header::Header, home::Home};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <Header />

        <Home />
        </>
    }
}
