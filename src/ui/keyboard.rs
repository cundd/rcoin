use std::thread;
use std::sync::mpsc;
use std::io::stdin;
use std::io::Read;
use std::collections::HashMap;

pub struct KeyboardListener<F, R>
    where F: Fn(char) -> R {
    receiver: mpsc::Receiver<char>,
    map: HashMap<char, Vec<F>>,
}

impl<F, R> KeyboardListener<F, R>
    where F: Fn(char) -> R {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            loop {
                let mut buffer = [0; 1];
                let bytes_read = match stdin().read(&mut buffer[..]) {
                    Ok(bytes_read) => bytes_read,
                    Err(e) => panic!(e),
                };

//                let mut buffer = String::new();
//                let bytes_read = match stdin().read_line(&mut buffer) {
//                    Ok(bytes_read) => bytes_read,
//                    Err(e) => panic!(e),
//                };

                if bytes_read > 0 {
                    sender.send(buffer[0] as char).unwrap();
//                    sender.send(buffer.chars().nth(0).unwrap()).unwrap();
                }
            }
        });

        KeyboardListener {
            receiver,
            map: HashMap::new(),
        }
    }

    /// Execute the given callback each time the `key`+`Return` is hit
    pub fn add_listener(&mut self, key: char, callback: F) {
        match self.map.get(&key) {
            Some(_) => { self.map.get_mut(&key).unwrap().push(callback); }
            None => { self.map.insert(key, vec![callback]); }
        }
    }

    /// Check if one of the hotkeys have been hit and execute the registered callbacks
    pub fn listen(&self) -> Vec<R> {
        if let Ok(key) = self.receiver.try_recv() {
            if let Some(listeners) = self.map.get(&key) {
                let mut results = Vec::with_capacity(listeners.len());
                for listener in listeners {
                    results.push(listener(key));
                }

                return results;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn listen_test() {
        KeyboardListener::new().add_listener('q', |_: char| {});
    }
}