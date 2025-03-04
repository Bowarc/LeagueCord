pub enum Message {
    SwitchScene(crate::scene::Scene),
}

pub struct App {
    current_scene: crate::scene::Scene,
}

#[derive(Debug, PartialEq, yew::Properties)]
pub struct Props {
    pub scenes: Vec<crate::scene::Scene>,
    pub default_scene_index: usize,
}

impl yew::Component for App {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let scenes = &ctx.props().scenes;
        let current_scene = *scenes
            .get(ctx.props().default_scene_index)
            .or_else(|| scenes.first())
            .unwrap();

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
        use {crate::component::NotificationManager, js_sys::Date, yew::html};

        let scenes = ctx.props().scenes.clone();

        html! {
            <div id="global">
            <div id="header">
                <a class="header_item" href="http://github.com/Bowarc/leaguecord">
                    <img src="/resources/github.webp" alt="Github icon" class="icon"/>
                </a>
                <div id="scene_list" class="header_item">{
                    scenes.into_iter().map(|scene|{
                        html!{
                            <button class={format!("scene_button{}", if self.current_scene == scene {" current"} else {""})} onclick={ctx.link().callback(move |_| Message::SwitchScene(scene))}>
                                { format!("{scene}") }
                            </button>
                        }
                    }).collect::<Vec<yew::virtual_dom::VNode>>()
                }</div>
            </div>
            <div id="content">
                {
                    self.current_scene.html(ctx)
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
