use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Luke";
    let my_object = MyObject {
        username: name.to_string(),
        favorite_language: "Rust".to_string(),
    };

    log!("My name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    html! {
    <h1> {"Hello World!"} </h1>
    }
}
