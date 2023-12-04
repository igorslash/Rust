use std::sync::{Arc, Mutex};

pub struct Task {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) status: TaskStatus,
    // Дополнительные поля задачи
}

pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    // Другие возможные статусы
}

pub struct TaskList {
    pub(crate) tasks: Arc<Mutex<Vec<Task>>>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn add_task(&self, task: Task) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
    }

    pub fn remove_task(&self, task_id: u32) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| task.id != task_id);
    }

    // Другие методы для обработки задач
}