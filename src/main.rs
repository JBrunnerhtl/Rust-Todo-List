use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, io::Write};
use std::fs::OpenOptions;
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: usize,
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
        Self { message, id: 0 }

    }

    fn push_to_json(&self, path: &Path){
        if path.exists() {
            let json_data = fs::read_to_string(path).expect("Something went wrong reading the file");
            let mut result_array : Vec<Message> = Vec::new();
            let mut result_single: Message;
            if serde_json::from_str::<Message>(&json_data).is_ok() {
                result_single = serde_json::from_str::<Message>(&json_data).unwrap();
                println!("{}", result_single.message);
                result_array.push(result_single);
            }
            else if serde_json::from_str::<Vec<Message>>(&json_data).is_ok() {

                result_array = serde_json::from_str::<Vec<Message>>(&json_data).unwrap_or_default();
                result_array.iter().for_each(|msg|{
                    println!("{}", msg.message);
                })
            }

            let new_message: String = format!(r#"{{"id":{},"message":"{}"}}"#, result_array.len()+1, self.message);
            let parsed: Message = serde_json::from_str(&new_message).unwrap();
            result_array.push(parsed);
            let result_array = serde_json::to_string_pretty(&result_array).unwrap();

            let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(path).unwrap();
            file.write_all(result_array.as_bytes()).unwrap();

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
