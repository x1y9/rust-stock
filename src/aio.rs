use std::{sync::mpsc::{Sender, channel}};

#[derive(Clone)]
pub struct Executor {
    task_sender: Sender<Task>,
}
pub enum Task {
    Println(String),
    Exit,

}

impl Executor {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        std::thread::spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(task) => {
                        match task {
                            Task::Println(string) => println!("{}", string),
                            Task::Exit => return
                        }
                    },
                    Err(_) => {
                        return;
                    }
                }
            }
        });
        Executor { task_sender: sender }
    }

    pub fn println(&self, string: String) {
        self.task_sender.send(Task::Println(string)).unwrap()
    }
}

