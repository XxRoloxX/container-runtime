use container_runtime::common::{
    feedback_commands::FeedbackCommand,
    sockets::get_client_socket_stream,
    thread_pool::{Job, ThreadPool},
};
use log::info;
use nix::unistd::Pid;

use crate::container::{Container, ContainerCallback};

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
        let (start_container_id, exit_container_id) = (container.id.clone(), container.id.clone());

        let on_start_cb: ContainerCallback = Box::from(move |pid: Pid| {
            let mut socket = get_client_socket_stream();
            socket.connect()?;
            socket.send_command(FeedbackCommand::ContainerStarted {
                pid: pid.as_raw() as i32,
                name: start_container_id,
            })
        });

        let on_exit_cb: ContainerCallback = Box::from(move |pid: Pid| {
            let mut socket = get_client_socket_stream();
            socket.connect()?;
            socket.send_command(FeedbackCommand::ContainerExited {
                pid: pid.as_raw() as i32,
                name: exit_container_id,
            })
        });

        let job: Job =
            Box::new(
                move || match container.start(Some(on_start_cb), Some(on_exit_cb)) {
                    Ok(_) => {
                        info!("Container {} was executed", container);
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
