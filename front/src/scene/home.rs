pub struct Home {
    group_creation_requested: bool,
}

#[derive(Debug)]
pub enum GroupCreateError {
    RequestCreation,
    Fetch,
    Parsing,
    Status(u16),
}

pub enum Message {
    CreateGroup,
    GroupCreated(u64),
    GroupCreateError(GroupCreateError),
}

#[derive(Debug, PartialEq, yew::Properties)]
pub struct Props {
    pub scene_switch: yew::Callback<crate::scene::Scene>,
}

impl yew::Component for Home {
    type Message = Message;

    type Properties = Props;

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self {
            group_creation_requested: false,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        use gloo::console::log;
        match msg {
            Message::CreateGroup => {
                use {
                    gloo::console::log,
                    wasm_bindgen::JsCast as _,
                    wasm_bindgen_futures::JsFuture,
                    web_sys::{window, Request, Response},
                };

                self.group_creation_requested = true;

                ctx.link().send_future(async move {
                    let request = match Request::new_with_str("/create_group") {
                        Ok(request) => request,
                        Err(e) => {
                            log!(format!(
                                "[ERROR] Failed to create group creation request due to: {e:?}"
                            ));
                            return Message::GroupCreateError(GroupCreateError::RequestCreation);
                        }
                    };

                    let Some(window) = window() else {
                        panic!("Failed to get the window");
                    };

                    let res = match JsFuture::from(window.fetch_with_request(&request)).await {
                        Ok(res) => res,
                        Err(e) => {
                            log!(format!("[ERROR] Fetch failed due to: {e:?}"));
                            return Message::GroupCreateError(GroupCreateError::Fetch);
                        }
                    };

                    let Ok(resp) = res.dyn_into::<Response>() else {
                        return Message::GroupCreateError(GroupCreateError::Parsing);
                    };

                    if resp.status() != 200 {
                        return Message::GroupCreateError(GroupCreateError::Status(resp.status()));
                    }

                    let resp_text_promise = match resp.text() {
                        Ok(json) => json,
                        Err(e) => {
                            log!(format!("[ERROR] Failed to get reponse text due to: {e:?}"));
                            return Message::GroupCreateError(GroupCreateError::Parsing);
                        }
                    };

                    let resp_text_value = match JsFuture::from(resp_text_promise).await {
                        Ok(json) => json,
                        Err(e) => {
                            log!(format!(
                                "[ERROR] Failed to convert response text to JsValue {e:?}"
                            ));
                            return Message::GroupCreateError(GroupCreateError::Parsing);
                        }
                    };

                    let Some(resp_text) = resp_text_value.as_string() else {
                        return Message::GroupCreateError(GroupCreateError::Parsing);
                    };

                    let group_id = match resp_text.parse::<u64>() {
                        Ok(v) => v,
                        Err(e) => {
                            log!(format!("[ERROR] Failed to parse group id {e}"));

                            return Message::GroupCreateError(GroupCreateError::Parsing);
                        }
                    };

                    log!(format!("Created group id: {group_id:?}"));

                    Message::GroupCreated(group_id)
                });
                true
            }
            Message::GroupCreated(id) => {
                use {
                    crate::component::{push_notification, Notification},
                    yew_router::prelude::RouterScopeExt as _,
                };

                push_notification(Notification::info(
                    "Group created",
                    vec!["A new group has been created with id:", &id.to_string()],
                    5.,
                ));

                let Some(nav) = ctx.link().navigator() else {
                    panic!("Could not get the navigator");
                };

                nav.replace(&crate::Route::Group { id });
                ctx.props()
                    .scene_switch
                    .emit(crate::scene::Scene::Group { group_id: id });

                true
            }
            Message::GroupCreateError(e) => {
                use crate::component::{push_notification, Notification};

                let reason_string = match e {
                    GroupCreateError::RequestCreation => "Failed to create request".to_string(),
                    GroupCreateError::Fetch => "The server failed to respond".to_string(),
                    GroupCreateError::Parsing => {
                        "Failed to understand the server's response".to_string()
                    }
                    GroupCreateError::Status(code) => {
                        format!("The server failed to handle the group creation request ({code})")
                    }
                };

                push_notification(Notification::error(
                    "Group creation error",
                    vec![
                        "An error occured while requesting a new group:",
                        &reason_string,
                    ],
                    5.,
                ));

                self.group_creation_requested = false;

                true
            }
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;
        // let nav_opt = ctx.link().navigator().unwrap();
        // log!(navigator.is_some());

        let create_group = ctx.link().callback(|_| Message::CreateGroup);

        html! {<>
            <div class="home">
                <p class="home_main_title">{
                    "LeagueCord, a voice chat for league of legends"
                }</p>

                <section class="home_section">
                    <h2 class="home_section_title">{
                        "Welcome"
                    }</h2>
                    <p class="home_section_text">{
                        if !self.group_creation_requested{
                            html!{<button onclick={create_group}>{"Create a group"}</button>}
                        }else{
                            let texts = vec!["Loading", "Loading . ", "Loading . .", "Loading . . ."];
                            let delay_ms = 333;
                            html!{<crate::component::ChangingText {texts} {delay_ms} />}
                        }
                    }</p>
                </section>
            </div>
        </>}
    }
}
