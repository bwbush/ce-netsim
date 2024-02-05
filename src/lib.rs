pub(crate) mod defaults;
mod msg;
mod sim_context;
mod sim_id;
mod sim_link;
mod time_ordered;

pub use self::{
    msg::HasBytesSize,
    sim_context::{SimConfiguration, SimContext},
    sim_id::SimId,
};
pub(crate) use self::{
    msg::Msg,
    sim_link::{link, SimDownLink, SimUpLink},
    time_ordered::TimeOrdered,
};
use anyhow::Result;

pub struct SimSocket<T> {
    id: SimId,
    up: SimUpLink<T>,
    down: SimDownLink<T>,
}

impl<T> SimSocket<T> {
    pub(crate) fn new(id: SimId, to_bus: SimUpLink<T>, receiver: SimDownLink<T>) -> Self {
        Self {
            id,
            up: to_bus,
            down: receiver,
        }
    }

    pub fn id(&self) -> &SimId {
        self.id()
    }
}

impl<T> SimSocket<T>
where
    T: HasBytesSize,
{
    pub fn send_to(&self, to: SimId, msg: T) -> Result<()> {
        let msg = Msg::new(self.id().clone(), to, msg);
        self.up.send(msg)
    }

    pub async fn recv(&mut self) -> Option<(SimId, T)> {
        let msg = self.down.recv().await?;

        Some((msg.from().clone(), msg.into_content()))
    }
}
