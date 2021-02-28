pub struct Log {
    pub path: String
}

impl Log {
    pub fn get_log(&self) -> Vec<String> {
        return get_lines(self.path.as_str()).unwrap();
    }
}

pub fn get_lines(path: &str) -> Result<Vec<String>, String> {
    let contents = std::fs::read_to_string(path);

    match contents {
        Ok(c) => {
            let data: Vec<String> = c.lines().map(String::from).collect();
            return Ok(data)
        }
        Err(e) => {
            Err(String::from("Unable to read file"))
        }
    }

    

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_lines() {
        let lines: Vec<String> = super::get_lines("tests/files/test_get_lines.txt").unwrap();

        assert_eq!(3, lines.len());

        assert_eq!(String::from("line one"), lines[0]);
    }
}