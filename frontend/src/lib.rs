use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = style!(
        r#"
            color: red;
        "#
    ).unwrap();

    html! {
        <h1 class={stylesheet}> {"Hello World!"} </h1>
    }
}
