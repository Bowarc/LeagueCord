use yew::html;

pub struct Home;


impl yew::Component for Home {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {

        html! {<>
            <div class="home">
                <p class="home_main_title">{
                    "LeagueCord, a voice chat for league of legends"
                }</p>

                <section class="home_section">
                    <h2 class="home_section_title">{
                        "Welcome"
                    }</h2>
                    <p class="home_section_text">
                        { "TODO" }
                    </p>
                </section>
            </div>
        </>}
    }
}
