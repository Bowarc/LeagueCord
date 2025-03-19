pub struct GroupNotFound;

impl yew::Component for GroupNotFound {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;

        html! {<>
            <h1>{"The requested group was not found"}</h1>
            { "TODO" }
        </>}
    }
}
