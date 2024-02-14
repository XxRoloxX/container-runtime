use container_runtime::common::{
    feedback_commands::FeedbackCommand,
    sockets::get_client_socket_stream,
    thread_pool::{Job, ThreadPool},
};
use log::{error, info};
use nix::unistd::Pid;

use crate::container::{Container, ContainerStartCallback};

pub struct Runner {
    pool: ThreadPool,
}

impl Runner {
    pub fn new(size: usize) -> Self {
        let pool = ThreadPool::new(size);
        Runner { pool }
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

    pub unsafe fn start_container(&mut self, container: Container) -> Result<(), String> {
        let container_id = container.id.clone();
        let mut output_socket = get_client_socket_stream();
        output_socket.connect()?;

        let send_command_cb: ContainerStartCallback = Box::from(move |pid: Pid| {
            if let Err(err) = output_socket.send_command(FeedbackCommand::ContainerStarted {
                pid: pid.as_raw() as i32,
                name: container_id,
            }) {
                error!("Couldn't send feedback to the client {}", err);
            }
        });

        let job: Job = Box::new(move || match container.start(send_command_cb) {
            Ok(_) => {
                info!("Container {} was executed", container);
            }
            Err(err) => {
                info!("Couldn't start {} :{}", container, err);
            }
        });

        self.start_job(job)?;

        Ok(())
    }

    pub fn create_container(&self, container: Container) -> Result<(), String> {
        let job = Box::new(move || match container.create() {
            Ok(_) => {
                info!("Container created ")
            }
            Err(err) => {
                error!("Couldn't create container {}", err)
            }
        });
        self.start_job(job)?;
        Ok(())
    }
}
