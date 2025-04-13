pub struct About;

#[derive(Debug, PartialEq, yew::Properties)]
pub struct Props {
    pub scene_switch: yew::Callback<crate::scene::Scene>,
}

impl yew::Component for About {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;

        let scene_switch = ctx
            .props()
            .scene_switch
            .reform(move |_| crate::scene::Scene::Contact);

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
                <p>
                    { "For security issues, please contact me through my email in the " }
                        <a onclick={ scene_switch }>{ "Contact tab" }</a>
                    { "." }
                </p>
            </div>
        </>}
    }
}
