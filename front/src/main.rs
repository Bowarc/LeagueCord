mod app;
mod component;
mod scene;
mod utils;
mod time;

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
            <Switch<Route> render={
                |route: Route|
                match route {
                    Route::Home => {
                        let scenes = vec![
                            Scene::Home,
                            Scene::About,
                            Scene::Contact
                        ];
                        let default_scene_index = 0;
                        
                        html! { <app::App {scenes} {default_scene_index} /> }
                    }
                    Route::Group { id: group_id } => {
                        let scenes = vec![
                            Scene::Home,
                            Scene::Group{ group_id },
                            Scene::About,
                            Scene::Contact
                        ];
                        let default_scene_index = 1;
                        html! { <app::App {scenes} {default_scene_index} /> }
                    }
                    Route::GroupNotFound => {
                        let scenes = vec![
                            Scene::Home,
                            Scene::GroupNotFound,
                            Scene::About,
                            Scene::Contact
                        ];
                        let default_scene_index = 1;
                        html! { <app::App {scenes} {default_scene_index} /> }
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
