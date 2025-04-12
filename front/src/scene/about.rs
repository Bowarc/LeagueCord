pub struct About;

impl yew::Component for About {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;

        html! {<>
            <div class="about">
                <h1>{"About"}</h1>
                // <p>{ "TODO" }</p>
                <p>{ "Temporary voice group system for players." }</p>
                <h3>{ "Current state: " }</h3>
                <p>{ "Most of the basic logic is done, interface is still lacking" }</p>
                <p>
                    { "If you have any specfic feature idea, please file an issue " }
                    <a href="https://github.com/bowarc/leaguecord/issues">{ "here" }</a>
                    { "." }
                </p>
                <p>{ "For security issues, please contact me through my email in the Contact tab." }</p>
            </div>
        </>}
    }
}
