use core::fmt;
use std::{
    fmt::{Display, Formatter},
    sync::{Arc, Mutex},
};

use container_runtime::common::{
    client_request::{ClientId, ClientResponse},
    commands::feedback_commands::FeedbackCommand,
    process::kill_process,
    sockets::send_feedback,
    thread_pool::{Job, ThreadPool},
};
use log::info;
use nix::unistd::Pid;

use crate::container::{Container, ContainerCallback};

#[derive(Clone, Debug)]
pub struct RunningContainerInfo {
    pub container: Container,
    pub client_id: ClientId,
    pub pid: Pid,
}

impl Display for RunningContainerInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Container: {}, Image: {} , Client: {}, Pid {}",
            self.container.id,
            self.container.image.id,
            self.client_id.get_id(),
            self.pid
        )
    }
}

pub struct Runner {
    pool: ThreadPool,
    running_containers: Arc<Mutex<Vec<RunningContainerInfo>>>,
}

impl Runner {
    pub fn new(size: usize) -> Self {
        let pool = ThreadPool::new(size);
        Runner {
            pool,
            running_containers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn start_job(&self, job: Box<dyn FnOnce() + Send + 'static>) -> Result<(), String> {
        let sender = self
            .pool
            .sender
            .as_ref()
            .ok_or(format!("Thread pool is not configured for this runner"))?;

        sender
            .send(job)
            .map_err(|e| format!("Couldn't schedule a job {}", e))?;

        Ok(())
    }

    pub fn get_running_containers(&self) -> Vec<RunningContainerInfo> {
        self.running_containers
            .lock()
            .unwrap()
            .iter()
            .map(|info| info.clone())
            .collect()
    }

    fn add_new_running_container(
        running_containers: Arc<Mutex<Vec<RunningContainerInfo>>>,
        container: Container,
        client_id: ClientId,
        pid: Pid,
    ) {
        running_containers
            .lock()
            .unwrap()
            .push(RunningContainerInfo {
                container,
                client_id,
                pid,
            });
    }

    pub fn stop_container(&mut self, container_id: String) -> Result<(), String> {
        let running_containers = self.get_running_containers();
        let container_pid = running_containers
            .iter()
            .find(|info| info.container.id == container_id)
            .map(|info| info.pid)
            .ok_or(format!("Container {} is not running", container_id))?;
        kill_process(container_pid)?;
        Ok(())
    }

    fn remove_running_container(
        running_containers: Arc<Mutex<Vec<RunningContainerInfo>>>,
        container: Container,
    ) {
        let mut running_containers = running_containers.lock().unwrap();
        running_containers.retain(|info| info.container.id != container.id);
    }

    fn on_container_start_callback(
        pid: Pid,
        container: Container,
        client_id: ClientId,
        running_containers_on_start: Arc<Mutex<Vec<RunningContainerInfo>>>,
    ) -> Result<(), String> {
        let client_response = ClientResponse::new(
            client_id.clone(),
            FeedbackCommand::ContainerStarted {
                pid: pid.as_raw() as i32,
                name: container.id.clone(),
            },
        );

        Runner::add_new_running_container(running_containers_on_start, container, client_id, pid);
        send_feedback(client_response)
    }
    fn on_container_exit_callback(
        running_containers: Arc<Mutex<Vec<RunningContainerInfo>>>,
        pid: Pid,
        container: Container,
        client_id: ClientId,
    ) -> Result<(), String> {
        let client_response = ClientResponse::new(
            client_id,
            FeedbackCommand::ContainerExited {
                pid: pid.as_raw() as i32,
                name: container.id.clone(),
            },
        );

        Runner::remove_running_container(running_containers, container);
        send_feedback(client_response)
    }

    pub unsafe fn start_container(
        &mut self,
        container: Container,
        client_id: ClientId,
    ) -> Result<(), String> {
        let (container_on_start, container_on_exit) = (container.clone(), container.clone());
        let client_id_clone = client_id.clone();
        let running_containers_on_start = Arc::clone(&self.running_containers);
        let running_containers_on_exit = Arc::clone(&self.running_containers);

        let on_start_cb: ContainerCallback = Box::from(move |pid: Pid| {
            Runner::on_container_start_callback(
                pid,
                container_on_start,
                client_id,
                running_containers_on_start,
            )
        });

        let on_exit_cb: ContainerCallback = Box::from(move |pid: Pid| {
            Runner::on_container_exit_callback(
                running_containers_on_exit,
                pid,
                container_on_exit,
                client_id_clone,
            )
        });

        let job: Job =
            Box::new(
                move || match container.start(Some(on_start_cb), Some(on_exit_cb)) {
                    Ok(_) => {
                        info!("Container {} exited succesfully", container);
                    }
                    Err(err) => {
                        info!("Couldn't start {} :{}", container, err);
                    }
                },
            );

        self.start_job(job)?;

        Ok(())
    }
}
