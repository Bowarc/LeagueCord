pub struct GroupNotFound;

impl yew::Component for GroupNotFound {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;

        html! {<div class="group_not_found">
            <h3>{"The requested group no longer exists"}</h3>
            <p>
                { "Click " }
                <a href="/" class="redirection-link">{ "here" }</a>
                { " to go back to the home page" }
            </p>
        </div>}
    }
}
