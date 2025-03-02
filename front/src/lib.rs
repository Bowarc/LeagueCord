mod apps;
mod component;
mod scene;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, yew_router::Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/group/:id")]
    Group { id: u64 },
}
pub struct App;

impl yew::Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::prelude::Html {
        use {
            yew::html,
            yew_router::{BrowserRouter, Switch},
        };

        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> yew::Html {
    use yew::html;

    match routes {
        Route::Home => {
            let group_id = None;
            html! { <apps::HomeApp {group_id} /> }
        }
        Route::Group { id: group_id } => html! {
            html! { <apps::HomeApp {group_id} /> }
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn start() {
    yew::Renderer::<App>::new().render();
}
