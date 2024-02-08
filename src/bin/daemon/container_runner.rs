use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::container::Container;

pub struct ContainerRunner {
    pub containers: Vec<Worker>,
    pub sender: Option<mpsc::Sender<Job>>,
}

impl ContainerRunner {
    pub fn new(size: usize) -> ContainerRunner {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ContainerRunner {
            containers: workers,
            sender: Some(sender),
        }
    }

    // pub unsafe fn create_container(&self, container: Container) {
    //     let job = Box::new(move || match container.create() {
    //         Ok(_) => {
    //             println!("Container {} was created", container)
    //         }
    //         Err(err) => {
    //             println!("Couldn't create {} :{}", container, err)
    //         }
    //     });
    //
    //     self.sender.as_ref().unwrap().send(job).unwrap();
    // }

    pub unsafe fn start_container(&self, container: Container) {
        let job = Box::new(move || match container.start() {
            Ok(_) => {
                println!("Container {} was stared", container)
            }
            Err(err) => {
                println!("Couldn't start {} :{}", container, err)
            }
        });

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                }
                Err(e) => {
                    println!("{}", e);
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ContainerRunner {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.containers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
