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
}

pub struct HomeApp {
    current_scene: Scene,
}

impl yew::Component for HomeApp {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_scene: Scene::Home,
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
        html! {
            <div id="global">
            <div id="header">
                <a class="header_item" href="http://github.com/Bowarc/storage_server">
                    <img src="resources/github.webp" alt="Github icon" class="icon"/>
                </a>
                <div id="scene_list" class="header_item">{
                    [ Scene::Home, Scene::About, Scene::Contact ].iter().map(|scene|{
                        html!{
                            <button class={format!("scene_button{}", if &self.current_scene == scene {" current"} else{""})} onclick={ctx.link().callback(|_| Message::SwitchScene(*scene))}>
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
            Scene::About => html! {<><scene::About /></>},
            Scene::Contact => html! {<><scene::Contact /></>},
        }
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Home => write!(f, "Home"),
            Scene::About => write!(f, "About"),
            Scene::Contact => write!(f, "Contact"),
        }
    }
}

