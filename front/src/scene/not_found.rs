pub struct NotFound;

impl yew::Component for NotFound {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;

        html! {<div class="not-found">
            <h1>{ "404" }</h1>
            <p>
                { "The page you requested has not been found, click " }
                <a href="/">{ "here"}</a>
                { " to go back to the main page." }
            </p>
        </div>}
    }
}
