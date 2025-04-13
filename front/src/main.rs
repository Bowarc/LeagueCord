mod app;
mod component;
mod scene;
mod time;
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
    #[at("/group_not_found")]
    GroupNotFound,
}

#[yew::function_component]
fn Router() -> yew::Html {
    use {
        scene::Scene,
        yew::html,
        yew_router::{BrowserRouter, Switch},
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={ |route: Route| {
                let (scenes, default_scene_index) = match route {
                    Route::Home => {
                        (vec![
                            Scene::Home,
                            Scene::About,
                            Scene::Contact
                        ],0)
                    }
                    Route::Group { id: group_id } => {
                        (vec![
                            Scene::Home,
                            Scene::Group{ group_id },
                            Scene::About,
                            Scene::Contact
                        ],1)
                    }
                    Route::GroupNotFound => {
                        (vec![
                            Scene::GroupNotFound,
                        ],0)
                    }
                    Route::NotFound => {
                        (vec![
                            Scene::NotFound
                        ],0)
                    },
                };
                html! { <app::App {scenes} {default_scene_index} /> }
            }} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Router>::new().render();
}
