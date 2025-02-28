use yew::Properties;

use {
    js_sys::Date,
    yew::{html, Context, Html},
};

use crate::component;
use crate::scene;
use crate::utils;

pub enum Message {
    SwitchScene(Scene),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Scene {
    Home,
    About,
    Contact,
    Group { group_id: u64 },
}

pub struct HomeApp {
    current_scene: Scene,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub group_id: Option<u64>,
}

impl yew::Component for HomeApp {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let current_scene = if let Some(group_id) = ctx.props().group_id {
            Scene::Group { group_id }
        } else {
            Scene::Home
        };

        Self {
            current_scene
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SwitchScene(scene) => {
                self.current_scene = scene;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let scenes = if let Some(group_id) = ctx.props().group_id {
            vec![
                Scene::Home,
                Scene::Group { group_id },
                Scene::About,
                Scene::Contact,
            ]
        } else {
            vec![Scene::Home, Scene::About, Scene::Contact]
        };
        html! {
            <div id="global">
            <div id="header">
                <a class="header_item" href="http://github.com/Bowarc/storage_server">
                    <img src="resources/github.webp" alt="Github icon" class="icon"/>
                </a>
                <div id="scene_list" class="header_item">{
                    scenes.into_iter().map(|scene|{
                        html!{
                            <button class={format!("scene_button{}", if self.current_scene == scene {" current"} else{""})} onclick={ctx.link().callback(move |_| Message::SwitchScene(scene))}>
                                { format!("{scene}") }
                            </button>
                        }
                    }).collect::<Vec<yew::virtual_dom::VNode>>()
                }</div>
            </div>
            <div id="content">
                {
                    self.current_scene.html()
                }
                <component::NotificationManager />
            </div>
            <footer>
                { format!("Rendered: {}", String::from(Date::new_0().to_string())) }
            </footer>
            </div>
        }
    }
}

impl Scene {
    fn html(&self) -> yew::virtual_dom::VNode {
        match self {
            Scene::Home => {
                html! {<><scene::Home /></>}
            }
            Scene::Group { group_id } => html!{<>
                <scene::Group {group_id}/>
            </>},
            Scene::About => html! {<><scene::About /></>},
            Scene::Contact => html! {<><scene::Contact /></>},
        }
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Home => write!(f, "Home"),
            Scene::Group { .. } => write!(f, "Group"),
            Scene::About => write!(f, "About"),
            Scene::Contact => write!(f, "Contact"),
        }
    }
}
