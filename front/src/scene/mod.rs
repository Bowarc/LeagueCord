mod about;
pub use about::About;
mod contatct;
pub use contatct::Contact;
mod group;
pub use group::Group;
mod home;
pub use home::Home;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scene {
    Home,
    About,
    Contact,
    Group { group_id: u64 },
}

impl Scene {
    pub fn html(&self) -> yew::virtual_dom::VNode {
        use yew::html;

        match self {
            Scene::Home => {
                html! {<><Home /></>}
            }
            Scene::Group { group_id } => html! {<>
                <Group {group_id}/>
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
            Scene::About => write!(f, "About"),
            Scene::Contact => write!(f, "Contact"),
        }
    }
}
