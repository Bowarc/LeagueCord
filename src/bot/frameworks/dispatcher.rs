use serenity::{
    all::{Context, Framework, FullEvent},
    async_trait,
    prelude::TypeMapKey,
};

pub struct Dispatcher;

pub struct DispatcherData {
    event_sender: std::sync::mpsc::Sender<(Context, FullEvent)>,
}

impl DispatcherData {
    pub fn new(event_sender: std::sync::mpsc::Sender<(Context, FullEvent)>) -> Self {
        Self { event_sender }
    }
}

impl TypeMapKey for DispatcherData {
    type Value = DispatcherData;
}

#[async_trait]
impl Framework for Dispatcher {
    async fn dispatch(&self, ctx: Context, event: FullEvent) {
        let mut data_write = ctx.data.write().await;

        let Some(data_mut) = data_write.get_mut::<DispatcherData>() else {
            warn!("No dispatcher was set up, ignoring event");
            return;
        };

        if let Err(e) = data_mut.event_sender.send((ctx.clone(), event)) {
            error!("Failed to dispatch event due to: {e}");
        }
    }
}
