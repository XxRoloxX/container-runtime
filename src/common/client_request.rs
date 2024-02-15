use super::runtime_commands::ContainerCommand;

pub struct ClientId(pub String);

impl ClientId {
    pub fn new(id: String) -> ClientId {
        ClientId(rand::random::<u64>().to_string() + &id)
    }
}

pub struct ClientRequest {
    pub command: ContainerCommand,
    pub client_id: ClientId,
}
