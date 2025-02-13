use yew::html;

pub struct About;

impl yew::Component for About {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {<>
            <div class="about">
                <h1>{"About Me"}</h1>
                <p> { "TODO" }</p>
            </div>
        </>}
    }
}
