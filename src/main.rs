use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, io::Write};
use std::fs::OpenOptions;
use std::path::PathBuf;
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: usize,
    message: String,
}
struct MessageSaver
{
     path: PathBuf,
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
impl Message
{
    fn new(message: String) -> Self {
        Message { id: 0, message }
    }
}
impl MessageSaver {

    fn push_to_json(&mut self, message: String) {
        if self.path.exists() {
            let json_data = fs::read_to_string(&self.path).expect("Something went wrong reading the file");
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

            let new_message: String = format!(r#"{{"id":{},"message":"{}"}}"#, result_array.len()+1, message);
            let parsed: Message = serde_json::from_str(&new_message).unwrap();
            result_array.push(parsed);
            let result_array = serde_json::to_string_pretty(&result_array).unwrap();

            let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(&self.path).unwrap();
            file.write_all(result_array.as_bytes()).unwrap();

        }

    }

    fn delete_message(&self, name: String) {
        let json_data = fs::read_to_string(&self.path).expect("Something went wrong reading the file");
        let mut result_array = serde_json::from_str::<Vec<Message>>(&json_data)
            .or_else(|_| serde_json::from_str::<Message>(&json_data).map(|m| vec![m]))
            .unwrap_or_default();
        let mut remove_index : Vec<usize> = Vec::new();

        result_array.iter_mut().filter(|m| m.message == name)
            .for_each(|m| {remove_index.push(m.id-1)});
        remove_index.iter().for_each(|i| {
           result_array.remove(*i);
        });
        let result_array = serde_json::to_string_pretty(&result_array).unwrap();
        let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(&self.path).unwrap();
        file.write_all(result_array.as_bytes()).unwrap();

    }
    fn list_messages(&self, path: &Path)
    {

    }

    fn change_path(&mut self,  input: &str) {
        let new_path = Path::new(input).to_owned();
        if new_path.exists() {
            self.path = new_path;
        } else {
            println!("Path does not exist.");
        }
    }

    fn new(path: PathBuf) -> Self {
        MessageSaver { path }
    }
}

fn main() {
    let mut message = Message::new("Hello, world!".to_string());
    let path = Path::new("todo.json").to_owned();
    let mut message_saver = MessageSaver::new(path);
    message_saver.delete_message(message.message);

}
