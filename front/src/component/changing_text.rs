use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub texts: Vec<&'static str>,
    pub delay_ms: u32,
}
pub struct ChangingText {
    current_index: usize,
}
pub enum Message {
    Test,
    CreateGroup,
    GroupCreated(u64),
    GroupCreateError(wasm_bindgen::JsValue),
}
impl yew::Component for ChangingText {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        let callback = ctx.link().callback(|_: ()| ());
        gloo_timers::callback::Interval::new(ctx.props().delay_ms, move || callback.emit(()))
            .forget();
        Self { current_index: 0 }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.current_index += 1;
        self.current_index %= ctx.props().texts.len();
        true
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;
        // let nav_opt = ctx.link().navigator().unwrap();
        // log!(navigator.is_some());

        html! {<>{{
            ctx.props().texts.get(self.current_index)
        }}</>}
    }
}
