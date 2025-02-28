use yew::{html, Context, Html};
use yew_router::{BrowserRouter, Routable, Switch};

mod scene;
mod component;
mod apps;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}    
pub struct App;

impl yew::Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::prelude::Html {
        html!{
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <apps::HomeApp /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn start() {
    yew::Renderer::<App>::new().render();
}
