use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, io::Write};
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String,
}
enum Command {
    /// Add a message
    Add { message: String },
    /// List all messages
    List,
    /// Delete a message
    Delete { id: usize },
    Save,

}
impl Message {
    fn new(message: String) -> Self {
        Self { message }
    }

    fn push_to_json(&self, path: &Path) -> Vec<Message>{
        if path.exists() {
            let json_data = fs::read_to_string(path).unwrap();
            let a = serde_json::from_str::<Vec<Message>>(&json_data).unwrap_or_default();
            println!("{}", json_data);
            Vec::new()
        }
        else {
            Vec::new()
        }
    }

    fn delete_message(&self, path: &Path) {

    }
}

fn main() {
    let value: Message = Message::new("Hello, World!".to_string());
    let path = Path::new("todo.json");
    value.push_to_json(path);
}
