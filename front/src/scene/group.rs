use yew::{html, Properties};

pub struct Group;

#[derive(Debug, PartialEq, Properties)]
pub struct Props{
    pub group_id: u64
}

impl yew::Component for Group {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        let group_id= ctx.props().group_id;
        
        html! {<>
            <div class="about">
                <h1>{format!("Group {group_id}")}</h1>
            </div>
        </>}
    }
}
