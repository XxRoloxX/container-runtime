use container_runtime::common::thread_pool::ThreadPool;
use log::{error, info};

use crate::container::Container;

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

    pub unsafe fn start_container(&self, container: Container) -> Result<(), String> {
        let job = Box::new(move || match container.start() {
            Ok(_) => {
                info!("Container {} was stared", container)
            }
            Err(err) => {
                info!("Couldn't start {} :{}", container, err)
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
