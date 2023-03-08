use yew::prelude::*;
use yew_feather::{Github, Linkedin};

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="container">
            <div class="anchor" id="home" />
            <div class="row">
                <div class="col start">
                    <h1>{"Luke Eltiste"}</h1>
                    <h4>{"Computer Scientist"}</h4>
                    <h4>{"Programmer"}</h4>
                </div>
                <div class="col">
                    <a class="link" href={"https://github.com/frazzer951"}> <Github size="48" />
                    </a>

                    <a class="link" href={"https://www.linkedin.com/in/luke-eltiste/"}> <Linkedin size="48" />
                    </a>
                </div>
            </div>
        </div>
    }
}
