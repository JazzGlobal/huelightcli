use std::sync::Mutex;

pub trait ILogger {
    fn log(&self, message: &str);
    fn entries(&self) -> Vec<String>;
}

#[derive(Default)]
pub struct Logger {
    entries: Mutex<Vec<String>>,
}

impl ILogger for Logger {
    fn log(&self, message: &str) {
        /*
         * Logs a message to the logger's internal log storage.
         * Puts a newline after each message.
         */
        self.entries
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(message.to_string() + "\n");
        println!("{}", message);
    }

    fn entries(&self) -> Vec<String> {
        self.entries
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
}
