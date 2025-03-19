mod about;
mod contatct;
mod group;
mod group_not_found;
mod home;

pub use about::About;
pub use contatct::Contact;
pub use group::Group;
pub use group_not_found::GroupNotFound;
pub use home::Home;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    Home,
    About,
    Contact,
    Group { group_id: u64 },
    GroupNotFound,
}

impl Scene {
    pub fn html(&self, ctx: &yew::Context<crate::app::App>) -> yew::virtual_dom::VNode {
        use yew::html;

        match self {
            Scene::Home => {
                let scene_switch = ctx.link().callback(crate::app::Message::SwitchScene);
                html! {<><Home {scene_switch}/></>}
            }
            Scene::Group { group_id } => html! {<>
                <Group {group_id}/>
            </>},
            Scene::GroupNotFound => html! {<>
                <GroupNotFound />
            </>},
            Scene::About => html! {<><About /></>},
            Scene::Contact => html! {<><Contact /></>},
        }
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Home => write!(f, "Home"),
            Scene::Group { .. } => write!(f, "Group"),
            Scene::GroupNotFound { .. } => write!(f, "Group not found"),
            Scene::About => write!(f, "About"),
            Scene::Contact => write!(f, "Contact"),
        }
    }
}
