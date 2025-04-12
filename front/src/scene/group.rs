pub struct Group {
    data_fetch_state: DataFetchState,
}

#[derive(Debug, PartialEq, yew::Properties)]
pub struct Props {
    pub group_id: u64,
}

pub enum DataFetchState {
    Pending,
    Received(shared::GroupData),
    Failed(DataFetchError),
}

pub enum DataFetchError {
    RequestCreation,
    Fetch,
    Parsing,
    Status(u16),
}

impl std::fmt::Display for DataFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFetchError::RequestCreation => write!(f, "Failed to create the request"),
            DataFetchError::Fetch => write!(f, "The server failed to respond"),
            DataFetchError::Parsing => write!(f, "Failed to understand the server's response"),
            DataFetchError::Status(code) => write!(
                f,
                "The server failed to handle the group data fetch request ({code})"
            ),
        }
    }
}

pub enum Message {
    Redraw,
    DataReceived(shared::GroupData),
    DataFetchError(DataFetchError),
}

impl yew::Component for Group {
    type Message = Message;

    type Properties = Props;

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        use {
            gloo::console::log,
            shared::GroupData,
            wasm_bindgen::JsCast as _,
            wasm_bindgen_futures::JsFuture,
            web_sys::{window, Request, Response},
        };

        let id = ctx.props().group_id;

        ctx.link().send_future(async move {
            let request = match Request::new_with_str(&format!("/group_data/{id}")) {
                Ok(request) => request,
                Err(e) => {
                    log!(format!(
                        "[ERROR] Failed to create group data request due to: {e:?}"
                    ));
                    return Message::DataFetchError(DataFetchError::RequestCreation);
                }
            };

            let Some(window) = window() else {
                panic!("Failed to get the window");
            };

            let res = match JsFuture::from(window.fetch_with_request(&request)).await {
                Ok(res) => res,
                Err(e) => {
                    log!(format!("[ERROR] Fetch (group data) failed due to: {e:?}"));
                    return Message::DataFetchError(DataFetchError::Fetch);
                }
            };

            let Ok(resp) = res.dyn_into::<Response>() else {
                return Message::DataFetchError(DataFetchError::Parsing);
            };

            if resp.status() != 200 {
                return Message::DataFetchError(DataFetchError::Status(resp.status()));
            }

            let resp_text_promise = match resp.text() {
                Ok(json) => json,
                Err(e) => {
                    log!(format!("[ERROR] failed to get response text due to: {e:?}"));
                    return Message::DataFetchError(DataFetchError::Parsing);
                }
            };

            let resp_text_value = match JsFuture::from(resp_text_promise).await {
                Ok(json) => json,
                Err(e) => {
                    log!(format!(
                        "[ERROR] Failed to convert response text to JsValue {e:?}"
                    ));
                    return Message::DataFetchError(DataFetchError::Parsing);
                }
            };

            let Some(resp_text) = resp_text_value.as_string() else {
                return Message::DataFetchError(DataFetchError::Parsing);
            };

            let group_data = match serde_json::from_str::<GroupData>(&resp_text) {
                Ok(v) => v,
                Err(e) => {
                    log!(format!(
                        "[ERROR] Failed to parse received group data due to: {e:?}"
                    ));
                    return Message::DataFetchError(DataFetchError::Parsing);
                }
            };

            log!(format!("Data received: {group_data:?}"));

            Message::DataReceived(group_data)
        });

        Self {
            data_fetch_state: DataFetchState::Pending,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Redraw => true,
            Message::DataReceived(data) => {
                use gloo::timers::callback::Interval;

                self.data_fetch_state = DataFetchState::Received(data);

                // set up an interval to redraw the window and update the timer
                let callback = ctx.link().callback(|_: ()| Message::Redraw);
                Interval::new(100, move || callback.emit(())).forget();
                true
            }
            Message::DataFetchError(error) => {
                use crate::component::{push_notification, Notification};

                push_notification(Notification::error(
                    "Group data fetch error",
                    vec![
                        &format!("An error occured while requesting data about group {}", ctx.props().group_id),
                        &format!("{error}")
                    ],
                    5.,
                ));

                self.data_fetch_state = DataFetchState::Failed(error);
                true
            }
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use {
            crate::time::format,
            web_time::{Duration, SystemTime},
            yew::html,
        };

        let group_id = ctx.props().group_id;

        let body = match &self.data_fetch_state {
            DataFetchState::Pending => html! {<>{ "Fetching data . ."}</>},
            DataFetchState::Received(group_data) => {
                html! {<div class="body">
                    {
                        format!(
                            "Created {} ago",
                            format(SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH +
                                        Duration::from_secs(group_data.creation_time_s_since_epoch()))
                                    .unwrap_or_else(|_| Duration::from_secs(0)),
                                1,
                            )
                        )
                    }
                    <br />
                    { format!("Member count: {}", group_data.user_count()) }
                    <br />
                    { format!("Join with: ") }
                    <a href={format!("https://discord.gg/{}", group_data.invite_code())} class="discord-join-link"> {"this link"}</a>
                </div>}
            }
            DataFetchState::Failed(error) => html! {<>{format!("Error: {}", error.to_string())}</>},
        };

        html! {<>
            <div class="group">
                <h1>{format!("Group {group_id}")}</h1>
                {
                    body
                }
            </div>
        </>}
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }
}
