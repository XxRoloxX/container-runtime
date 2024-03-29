use serde::{Deserialize, Serialize};

use super::{
    commands::{feedback_commands::FeedbackCommand, runtime_commands::ContainerCommand},
    sockets::SOCKETS_PATH,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientId(pub String);

impl ClientId {
    pub fn new() -> ClientId {
        let client_id = rand::random::<u64>().to_string();
        let socket_path = format!("{}/{}", SOCKETS_PATH, client_id);
        ClientId(socket_path)
    }
    pub fn get_id(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientRequest {
    pub command: ContainerCommand,
    pub client_id: ClientId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientResponse {
    pub client_id: ClientId,
    pub command: FeedbackCommand,
}

impl ClientResponse {
    pub fn new(client_id: ClientId, command: FeedbackCommand) -> ClientResponse {
        ClientResponse { client_id, command }
    }
}

impl ClientRequest {
    pub fn new(command: ContainerCommand) -> ClientRequest {
        ClientRequest {
            command,
            client_id: ClientId::new(),
        }
    }
    pub fn get_client_id(&self) -> &ClientId {
        &self.client_id
    }
}
