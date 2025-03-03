pub enum Message {
    SwitchScene(crate::scene::Scene),
}

pub struct App {
    current_scene: crate::scene::Scene,
}

#[derive(Debug, PartialEq, yew::Properties)]
pub struct Props {
    pub group_id: Option<u64>,
    pub scenes: Vec<crate::scene::Scene>,
}

impl yew::Component for App {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use crate::scene::Scene;
        let current_scene = if let Some(group_id) = ctx.props().group_id {
            Scene::Group { group_id }
        } else {
            Scene::Home
        };

        Self { current_scene }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SwitchScene(scene) => {
                self.current_scene = scene;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        use {
            crate::{component::NotificationManager, scene::Scene},
            js_sys::Date,
            yew::html,
        };

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
                <a class="header_item" href="http://github.com/Bowarc/leaguecord">
                    <img src="/resources/github.webp" alt="Github icon" class="icon"/>
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
                <NotificationManager />
            </div>
            <footer>
                { format!("Rendered: {}", String::from(Date::new_0().to_string())) }
            </footer>
            </div>
        }
    }
}
