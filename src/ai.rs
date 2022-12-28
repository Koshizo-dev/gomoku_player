use crate::Runtime;

pub struct Ai {
    pub runtime: Runtime,
}

impl Ai {
    pub fn new(runtime: Runtime) -> Self {
        Self { runtime }
    }

    pub fn reset(&mut self, board_size: usize) {
        match self.runtime.write(format!("START {}", board_size)) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error whilst reseting board: [{}]", err.to_string());
            }
        }
        self.flush();
        let _ = self.read();
    }

    pub fn read(&mut self) -> String {
        match self.runtime.read() {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error whilst writing to the AI: [{}]", err);
                String::new()
            }
        }
    }

    pub fn write(&mut self, target: String) {
        match self.runtime.write(target) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error whilst writing to the AI: [{}]", err);
            }
        }
    }

    pub fn flush(&mut self) {
        match self.runtime.flush() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error whilst flushing the AI: [{}]", err);
            }
        }
    }

    pub fn stop(&mut self) {
        self.write("END".to_string());
        self.flush();
        let _ = self.runtime.process.wait();
    }
}
