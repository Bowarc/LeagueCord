mod app;
mod component;
mod scene;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, yew_router::Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/group/:id")]
    Group { id: u64 },
}

#[yew::function_component]
fn Router() -> yew::Html {
    use {
        yew::html,
        yew_router::{BrowserRouter, Switch},
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={
                |route: Route|
                match route {
                    Route::Home => {
                        let group_id = None;
                        let scenes = Vec::new();
                        html! { <app::App {group_id} {scenes} /> }
                    }
                    Route::Group { id: group_id } => {
                        let scenes = Vec::new();
                        html! { <app::App {group_id} {scenes} /> }
                    }
                    Route::NotFound => html! { <h1>{ "404" }</h1> },
                }
            } />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Router>::new().render();
}
