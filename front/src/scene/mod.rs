mod about;
mod contatct;
mod group;
mod group_not_found;
mod home;
mod not_found;

pub use about::About;
pub use contatct::Contact;
pub use group::Group;
pub use group_not_found::GroupNotFound;
pub use home::Home;
pub use not_found::NotFound;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    Home,
    About,
    Contact,
    Group { group_id: u64 },
    GroupNotFound,
    NotFound, // 404
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
            Scene::About => {
                let scene_switch = ctx.link().callback(crate::app::Message::SwitchScene);
                html! {<><About {scene_switch} /></>}
            }
            Scene::Contact => html! {<><Contact /></>},
            Scene::NotFound => html! {<><NotFound /></>},
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
            Scene::NotFound => write!(f, "Not found"), // Should never be accessible by a nav button
        }
    }
}
