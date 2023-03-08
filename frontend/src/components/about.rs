use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
        <div class="container">
            <div class="anchor" id="about" />
            <div class="col center">
                <h2 class="center">{"About Me"}</h2>
                <div class="aboutme">
                    <p class="center">
                        { "I am a software developer and computer scientist based in Anaheim California. I'm currently on
                        my 4th year of my Computer Science degree at California State University Fullerton. I'm planning
                        on graduating soon. I have a passion for programming and I love to learn new things while
                        working on projects." }
                    </p>
                    <br />
                    <p class="center">
                        { "My favorite language to use is Rust, because it is an extreamly vercitile language that can be
                        used for almost any application while being fast and safe. My most proficient languages are
                        Rust, C++, and Python." }
                    </p>
                </div>
            </div>
        </div>
    }
}
