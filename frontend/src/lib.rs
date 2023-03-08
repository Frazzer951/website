mod components;

use yew::prelude::*;

use crate::components::{About, Header, Home, Projects};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <Header />

        <Home />
        <About />
        <Projects />

        </>
    }
}
