use serde::{Serialize,Deserialize};
use std::fs::{OpenOptions,File};
use std::io::{BufReader, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub id:usize,
    pub description:String,
    pub done:bool
}

const FILE_NAME:&str = "tasks.json";

pub fn load_tasks()->Vec<Task>{
    let mut file=File::open(FILE_NAME).unwrap_or_else(|_| File::create(FILE_NAME).unwrap());
    let mut buffer=String::new();
    file.read_to_string(&mut buffer);

    return serde_json::from_str(&buffer).unwrap_or_else(|_| Vec::new());
}

pub fn save_tasks(tasks: &Vec<Task>){
    let data=serde_json::to_string_pretty(tasks).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILE_NAME).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

pub fn add_task(description:String){
    let mut tasks=load_tasks();
    let id:usize=tasks.len()+1;
    let task:Task=Task { id: id, description: description, done: false };
    tasks.push(task);
    save_tasks(&tasks);
}

pub fn list_tasks(){
    let tasks=load_tasks();
    for task in &tasks{
        println!("[{}] {} - {}", task.id, if task.done { "✔" } else { "❌" }, task.description);
    }
}

pub fn mark_as_done(task_id:usize){
    let mut tasks=load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.done = true;
        save_tasks(&tasks);
        println!("Task {} marked as done.", task_id);
    } else {
        println!("Task ID not found.");
    }
}