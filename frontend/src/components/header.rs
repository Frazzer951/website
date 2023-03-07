use material_yew::{
    top_app_bar_fixed::{MatTopAppBarActionItems, MatTopAppBarTitle},
    MatButton, MatTopAppBarFixed,
};
use yew::prelude::*;

struct ButtonData {
    text: String,
    link: String,
}

#[function_component]
pub fn Header() -> Html {
    let buttons = vec![
        ButtonData {
            text: "Home".to_string(),
            link: "#home".to_string(),
        },
        ButtonData {
            text: "About".to_string(),
            link: "#about".to_string(),
        },
        ButtonData {
            text: "Projects".to_string(),
            link: "#projects".to_string(),
        },
    ];

    html! {
        <MatTopAppBarFixed>
            <MatTopAppBarTitle>
                <div class="app-title">
                    <h4>{"LUKE ELTISTE"}</h4>
                </div>
            </MatTopAppBarTitle>

            { for buttons.iter().map(|button| html!{
            <MatTopAppBarActionItems>
                <a class="action-item" href={button.link.clone()}>
                    <MatButton label={button.text.clone()}/>
                </a>
            </MatTopAppBarActionItems>
            })
            }

        </MatTopAppBarFixed>
    }
}
