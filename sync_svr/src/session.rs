use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SyncMessage(pub String);

pub fn udp_svr(addr: &str) {}
