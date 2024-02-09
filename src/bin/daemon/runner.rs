use container_runtime::common::thread_pool::ThreadPool;

use crate::container::Container;

pub struct Runner {
    pool: ThreadPool,
}

impl Runner {
    pub fn new(size: usize) -> Self {
        let pool = ThreadPool::new(size);
        Runner { pool }
    }

    pub unsafe fn start_container(&self, container: Container) {
        let job = Box::new(move || match container.start() {
            Ok(_) => {
                println!("Container {} was stared", container)
            }
            Err(err) => {
                println!("Couldn't start {} :{}", container, err)
            }
        });

        self.pool.sender.as_ref().unwrap().send(job).unwrap();
    }
}
