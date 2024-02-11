use container_runtime::common::{
    process::redirect_standard_output,
    socket::{get_client_socket_stream, SocketStream},
    thread_pool::ThreadPool,
};
use log::{error, info};

use crate::container::Container;

pub struct Runner {
    pool: ThreadPool,
    output_socket: Option<Box<dyn SocketStream>>,
    output_socket_descriptor: Option<i32>,
}

impl Runner {
    pub fn new(size: usize) -> Self {
        let pool = ThreadPool::new(size);
        Runner {
            pool,
            output_socket: None,
            output_socket_descriptor: None,
        }
    }
    pub fn init_output_socket(&mut self) -> Result<(), String> {
        let mut socket_stream = get_client_socket_stream();
        self.output_socket_descriptor = Some(socket_stream.connect()?);
        self.output_socket = Some(socket_stream);
        redirect_standard_output(self.output_socket_descriptor.unwrap())
            .map_err(|e| format!("Couldn't redirect standard output {}", e))?;
        Ok(())
    }
    pub fn is_output_socket_initialized(&self) -> bool {
        self.output_socket.is_some()
    }
    pub fn get_output_socket_file_descriptor(&self) -> Result<i32, String> {
        let descriptor = self
            .output_socket_descriptor
            .ok_or(format!("Output socket is not initialized"))?;
        Ok(descriptor)
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

    pub unsafe fn start_container(&self, container: Container) -> Result<(), String> {
        println!("Before Starting a job");
        let job = Box::new(move || match container.start() {
            Ok(_) => {
                info!("Container {} was stared", container)
            }
            Err(err) => {
                info!("Couldn't start {} :{}", container, err)
            }
        });
        println!("Starting a job");
        self.start_job(job)?;
        println!("After starting a job");
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
