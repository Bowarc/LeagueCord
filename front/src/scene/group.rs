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
    Failed(wasm_bindgen::JsValue),
}

pub enum Message {
    DataReceived(shared::GroupData),
    DataFetchError(wasm_bindgen::JsValue),
}

impl yew::Component for Group {
    type Message = Message;

    type Properties = Props;

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        use {
            gloo::console::log,
            shared::GroupData,
            wasm_bindgen::{JsCast as _, JsValue},
            wasm_bindgen_futures::JsFuture,
            web_sys::{window, Request, Response},
        };

        let id = ctx.props().group_id;
        let request = match Request::new_with_str(&format!("/group_data/{id}")) {
            Ok(request) => request,
            Err(e) => panic!("Failed to create group data request due to: {e:?}"),
        };

        let Some(window) = window() else {
            panic!("Failed to get the window");
        };

        ctx.link().send_future(async move {
            let res = match JsFuture::from(window.fetch_with_request(&request)).await {
                Ok(res) => res,
                Err(e) => return Message::DataFetchError(e),
            };

            let Ok(resp) = res.dyn_into::<Response>() else {
                return Message::DataFetchError(JsValue::from_str(
                    "Failed to convert the response to a usable format",
                ));
            };

            if resp.status() != 200 {
                return Message::DataFetchError(JsValue::from_str(&format!(
                    "Request was not succesful: {}",
                    resp.status()
                )));
            }

            let resp_text_promise = match resp.text() {
                Ok(json) => json,
                Err(e) => return Message::DataFetchError(e),
            };

            let resp_text_value = match JsFuture::from(resp_text_promise).await {
                Ok(json) => json,
                Err(e) => return Message::DataFetchError(e),
            };

            let Some(resp_text) = resp_text_value.as_string() else {
                return Message::DataFetchError(JsValue::from_str(
                    "Failed to convert the received data to string",
                ));
            };

            let group_data = match serde_json::from_str::<GroupData>(&resp_text) {
                Ok(v) => v,
                Err(e) => return Message::DataFetchError(JsValue::from_str(&format!("{e}"))),
            };

            log!(format!("Data received: {group_data:?}"));

            Message::DataReceived(group_data)
        });

        Self {
            data_fetch_state: DataFetchState::Pending,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::DataReceived(data) => {
                self.data_fetch_state = DataFetchState::Received(data);
                true
            }
            Message::DataFetchError(error) => {
                self.data_fetch_state = DataFetchState::Failed(error);
                true
            }
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use yew::html;

        let group_id = ctx.props().group_id;

        let body = match &self.data_fetch_state {
            DataFetchState::Pending => html! {<>{ "Fetching data . ."}</>},
            DataFetchState::Received(group_data) => html! {<>{ format!("{group_data:?}")} </>},
            DataFetchState::Failed(js_value) => html! {<>{format!("Error: {js_value:?}")}</>},
        };

        html! {<>
            <div class="about">
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
