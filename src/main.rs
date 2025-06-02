
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, io::Write};
use std::fs::OpenOptions;
use std::path::PathBuf;
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
    ChangePath {path_buf: PathBuf},
    Save

}
impl Message
{
    fn new(message: String) -> Self {
        Message { id: 0, message }
    }
}
impl MessageSaver {

    fn push_to_json(&mut self, message: String) {

            let mut result_array = self.get_json_data();
            let new_message: String = format!(r#"{{"id":{},"message":"{}"}}"#, result_array.len()+1, message);
            let parsed: Message = serde_json::from_str(&new_message).unwrap();
            result_array.push(parsed);
            let result_array = serde_json::to_string_pretty(&result_array).unwrap();

            let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(&self.path).unwrap();
            file.write_all(result_array.as_bytes()).unwrap();

    }

    fn delete_message(&self, name: String) {
        let mut result_array = self.get_json_data();
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
    fn list_messages(&self)
    {
        let json_data = self.get_json_data();
        if json_data.is_empty() {
            println!("No messages found.");
        } else {
            for message in json_data {
                println!("ID: {}, Message: {}", message.id, message.message);
            }
        }
    }

    fn change_path(&mut self,  input: &str) {
        let new_path = Path::new(input).to_owned();
        if new_path.exists() {
            self.path = new_path;
        } else {
            println!("Path does not exist.");
        }
    }

    fn get_json_data(&self) -> Vec<Message> {
        let json_data = fs::read_to_string(&self.path).expect("Something went wrong reading the file");
        serde_json::from_str::<Vec<Message>>(&json_data)
            .or_else(|_| serde_json::from_str::<Message>(&json_data).map(|m| vec![m]))
            .unwrap_or_default()

    }

    fn new(path: PathBuf) -> Self {
        MessageSaver { path }
    }
}

fn main() {
    let message = Message::new("Hello, world!".to_string());
    let path = Path::new("todo.json").to_owned();
    let mut message_saver = MessageSaver::new(path);
    message_saver.list_messages();

}
