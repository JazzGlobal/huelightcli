pub trait ILogger {
    fn log(&mut self, message: &str);
    fn entries(&self) -> &Vec<String>;
}

#[derive(Default)]
pub struct Logger {
    entries: Vec<String>,
}

impl ILogger for Logger {
    fn log(&mut self, message: &str) {
        /*
         * Logs a message to the logger's internal log storage.
         * Puts a newline after each message.
         */
        self.entries.push(message.to_string() + "\n");
        println!("{}", message);
    }

    fn entries(&self) -> &Vec<String> {
        &self.entries
    }
}