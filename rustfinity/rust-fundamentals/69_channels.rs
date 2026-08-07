/*
    # 69. Channels

    Concurrency is an essential concept in modern programming. This challenge focuses on building a message processing system using Rust's channels and threads. You'll create a system where multiple producers send messages with different priority levels, and a single consumer processes and formats these messages.

    ## Your Task

    Implement three key functions that work with a message processing system:

    1. `create_message_channel()` - Creates a channel for sending Message structs
    2. `create_producer_thread()` - Creates a thread that analyzes and forwards messages
    3. `create_consumer_thread()` - Creates a thread that formats and collects messages

    ### Requirements

    #### Producer Thread

    - Receives a vector of messages and a sender channel
    - Updates priority based on content keywords:
      - "ERROR" → Critical
      - "WARNING" → High
      - "DEBUG" → Medium
      - Others become Low
    - Sends each updated message through the channel

    #### Consumer Thread

    - Receives messages until the channel is closed
    - Formats each message as: `[PRIORITY|SENDER_ID] CONTENT`
      where PRIORITY is one of: LOW, MED, HIGH, CRIT
    - Returns a vector of formatted message strings

    ### Notes

    - Ignore error handling, you can simply `unwrap()`
    - Have a look at the `main` function to see how the functions are used.

    ## Hints

    Here are some tips to help you get started:

    <details>
        <summary>Click here for implementation hints</summary>

    - Use `mpsc::channel()` to create the message channel
    - Use `thread::spawn` and move closures
    - Use `while let Ok(msg) = rx.recv()` for receiving
    - Format with `format!("[{}|{}] {}", priority, id, content)`

    </details>
*/

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// #[derive(Copy, Clone)]
pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    // 1. Implement this function to create and return a message channel
    mpsc::channel()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    // TODO: Create a thread that:
    // - Updates the priority based on content
    // - Sends the updated message through the channel
    thread::spawn(move || {
        for mut message in messages {
            message.priority = match message.content.as_str() {
                s if s.contains("ERROR") => Priority::Critical,
                s if s.contains("WARNING") => Priority::High,
                s if s.contains("DEBUG") => Priority::Medium,
                _ => Priority::Low,
            };
            tx.send(message).unwrap();
        }
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    // TODO: Create a thread that:
    // - Receives messages from the channel
    // - Formats them as "[PRIORITY|SENDER_ID] CONTENT"
    // - Returns a vector of formatted messages
    thread::spawn(move || {
        rx.iter().map(|message| {
            let pri = match message.priority {
                Priority::Low => "LOW",
                Priority::Medium => "MED",
                Priority::High => "HIGH",
                Priority::Critical => "CRIT",
            };
            format!("[{}|{}] {}", pri, message.sender_id, message.content)
        }).collect()
    })
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}
