use std::{fs::File, io::Write, sync::Mutex, path::PathBuf};


pub struct TestLogger{
    file: Mutex<File>,
}

impl TestLogger {
    pub fn new(name: &str) -> Box<Self> {
        let mut path = PathBuf::new();
        path.push(name);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
            }
        }

        Box::new(Self {
            file: Mutex::new(File::create(path).unwrap()),
        })
    }
}

impl log::Log for TestLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn flush(&self) {
        self.file.lock().unwrap().flush().unwrap();
    }

    fn log(&self, record: &log::Record) {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{:6} - {:25} - {}", record.level(), record.target(), record.args()).unwrap();
    }
}