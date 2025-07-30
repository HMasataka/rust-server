pub trait LoggingService {
    fn log(&self, message: &str);
}

pub struct ConsoleLoggingService;

impl ConsoleLoggingService {
    pub fn new() -> Self {
        Self
    }
}

impl LoggingService for ConsoleLoggingService {
    fn log(&self, message: &str) {
        println!("[LOG] {}", message);
    }
}