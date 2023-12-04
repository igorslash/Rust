use std::sync::Arc;
use std::thread;
use crate::Task_target::{Task, TaskList, TaskStatus};

fn main() {
    let task_list = Arc::new(TaskList::new());

    // Создание нескольких потоков для добавления задач
    let task_list_clone = Arc::clone(&task_list);
    let thread1 = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(100));
        let task1 = Task { id: 1, name: "Task 1".to_string(),
            description: "Description 1".to_string(), status: TaskStatus::Todo };
        task_list_clone.add_task(task1);
    });

    let task_list_clone = Arc::clone(&task_list);
    let thread2 = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(500));
        let task2 = Task { id: 2, name: "Task 2".to_string(),
            description: "Description 2".to_string(), status: TaskStatus::Todo };
        task_list_clone.add_task(task2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    // Обработка задач из списка
    let tasks = task_list.tasks.lock().unwrap();
    for task in &*tasks {
        println!("Task: {}", task.name);
    }
}