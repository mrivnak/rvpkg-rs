pub struct Log {
    pub path: String
}


impl Log {
    pub fn get_lines(&self, path: &str) -> Vec<String> {
        let contents = std::fs::read_to_string(path).expect("Unable to read package log");

        if contents == "Unable to read package db" {
            eprintln!("Error: Unable to read package log");
            std::process::exit(1);
        }

        let data: Vec<String> = contents.lines().map(String::from).collect();

        return data;
    }
    pub fn get_log(&self) -> Vec<String> {
        return self.get_lines(self.path.as_str());
    }
}
