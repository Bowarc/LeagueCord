pub struct Home;

pub enum Message {
    CreateGroup,
    GroupCreated(u64),
    GroupCreateError(wasm_bindgen::JsValue),
}

#[derive(Debug, PartialEq, yew::Properties)]
pub struct Props {
    // scene_switch: yew::Callback<crate::scene::>
}

impl yew::Component for Home {
    type Message = Message;

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        use gloo::console::log;
        match msg {
            Message::CreateGroup => {
                use {
                    gloo::console::log,
                    wasm_bindgen::{JsCast as _, JsValue},
                    wasm_bindgen_futures::JsFuture,
                    web_sys::{window, Request, Response},
                };

                ctx.link().send_future(async move {
                    log!("hi");

                    let request = match Request::new_with_str("/create_group") {
                        Ok(request) => request,
                        Err(e) => panic!("Failed to create group data request due to: {e:?}"),
                    };

                    let Some(window) = window() else {
                        panic!("Failed to get the window");
                    };

                    let res = match JsFuture::from(window.fetch_with_request(&request)).await {
                        Ok(res) => res,
                        Err(e) => return Message::GroupCreateError(e),
                    };

                    let Ok(resp) = res.dyn_into::<Response>() else {
                        return Message::GroupCreateError(JsValue::from_str(
                            "Failed to convert the response to a usable format",
                        ));
                    };

                    if resp.status() != 200 {
                        return Message::GroupCreateError(JsValue::from_str(&format!(
                            "Request was not succesful: {}",
                            resp.status()
                        )));
                    }

                    let resp_text_promise = match resp.text() {
                        Ok(json) => json,
                        Err(e) => return Message::GroupCreateError(e),
                    };

                    let resp_text_value = match JsFuture::from(resp_text_promise).await {
                        Ok(json) => json,
                        Err(e) => return Message::GroupCreateError(e),
                    };

                    let Some(resp_text) = resp_text_value.as_string() else {
                        return Message::GroupCreateError(JsValue::from_str(
                            "Failed to convert the received data to string",
                        ));
                    };

                    let group_id = match resp_text.parse::<u64>() {
                        Ok(v) => v,
                        Err(e) => {
                            return Message::GroupCreateError(JsValue::from_str(&format!("{e}")))
                        }
                    };

                    log!(format!("Created group id: {group_id:?}"));

                    // Message::DataReceived(group_data)

                    Message::GroupCreated(group_id)
                });
                true
            }
            Message::GroupCreated(id) => {
                use web_sys::window;
                use yew_router::prelude::RouterScopeExt as _;

                let Some(nav) = ctx.link().navigator() else {
                    panic!("Could not get the navigator");
                };

                nav.replace(&crate::Route::Group { id });

                // window().unwrap().location().reload().unwrap();

                true
            }
            _ => true,
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use {
            // yew_router::hooks::use_navigator,
            gloo::console::log,
            yew::html,
            yew_router::prelude::*,
        };

        let nav_opt = ctx.link().navigator().unwrap();
        log!(format!("{nav_opt:?}"));

        let create_group = ctx.link().callback(|_| Message::CreateGroup);

        // log!(navigator.is_some());
        html! {<>
            <div class="home">
                <p class="home_main_title">{
                    "LeagueCord, a voice chat for league of legends"
                }</p>

                <section class="home_section">
                    <h2 class="home_section_title">{
                        "Welcome"
                    }</h2>
                    <p class="home_section_text">
                        <button onclick={create_group}>{"Create a group"}</button>
                    </p>
                </section>
            </div>
        </>}
    }
}
