
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

fn get_user_input() -> String {

    println!("1: Add a message");
    println!("2: List all messages");
    println!("3: Delete a message");
    println!("4: Change path for save");
    println!("5: Exit the program");
    println!("Please enter a number");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.len() > 1 || input.trim() == "1" || input.trim() == "2" || input.trim() == "3" || input.trim() == "4" || input.trim() == "5" {
        input = input[0..1].to_string();
    }
    else {
        println!("Please enter a valid number");
        return get_user_input();
    }
    input.trim().to_string()
}

fn main() {
    let mut message_saver: MessageSaver = MessageSaver::new(PathBuf::from("todo.json"));
    let mut user_input: String;
    loop {
        user_input = get_user_input();
        match user_input.as_str() {
            "1" => {
                println!("Please enter a message:");
                let mut message = String::new();
                std::io::stdin().read_line(&mut message).expect("Failed to read line");
                let message = message.trim().to_string();
                if !message.is_empty() {
                    message_saver.push_to_json(message);
                } else {
                    println!("Message cannot be empty.");
                }
            }
            "2" => {
                message_saver.list_messages();
            }
            "3" => {
                println!("Please enter the message to delete:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();
                if !name.is_empty() {
                    message_saver.delete_message(name);
                } else {
                    println!("Message cannot be empty.");
                }
            }
            "4" => {
                println!("Please enter the new path for save:");
                let mut path = String::new();
                std::io::stdin().read_line(&mut path).expect("Failed to read line");
                let path = PathBuf::from(path.trim());
                message_saver.change_path(path.to_str().unwrap());
            }
            "5" => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}
