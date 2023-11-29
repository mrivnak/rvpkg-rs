use std::process::Command;

fn create_commands(commands: &str) -> Vec<Command> {
    split_lines(commands)
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts = shlex::split(l).unwrap();
            let (cmd, args) = parts.split_first().unwrap();
            let mut command = Command::new(cmd);
            command.args(args);
            command
        })
        .collect()
}

fn split_lines(lines: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = Vec::new();
    for line in lines.lines() {
        if line.ends_with('\\') {
            current_line.push(line.trim_end_matches('\\'));
        } else if !current_line.is_empty() {
            current_line.push(line);
            result.push(current_line.join(" "));
            current_line.clear();
        } else {
            result.push(line.to_owned());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_create_commands() {
        let commands = create_commands(
            "echo hello\n\nls -l\necho \"string with spaces\"\necho \\\nmultiline\n",
        );
        assert_eq!(commands.len(), 4);
        assert_eq!(commands[0].get_program(), "echo");
        assert_eq!(commands[0].get_args().collect::<Vec<&OsStr>>(), &["hello"]);
        assert_eq!(commands[1].get_program(), "ls");
        assert_eq!(commands[1].get_args().collect::<Vec<&OsStr>>(), &["-l"]);
        assert_eq!(commands[2].get_program(), "echo");
        assert_eq!(
            commands[2].get_args().collect::<Vec<&OsStr>>(),
            &["string with spaces"]
        );
        assert_eq!(commands[3].get_program(), "echo");
        assert_eq!(
            commands[3].get_args().collect::<Vec<&OsStr>>(),
            &["multiline"]
        );
    }

    #[test]
    fn test_split_lines() {
        let test_string = "line 1\nline 2\nline 3\n";
        let lines = split_lines(test_string);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 1");
        assert_eq!(lines[1], "line 2");
        assert_eq!(lines[2], "line 3");
    }

    #[test]
    fn test_split_lines_with_escape() {
        let test_string = "line 1\nline 2\\\nline 3\n";
        let lines = split_lines(test_string);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "line 1");
        assert_eq!(lines[1], "line 2 line 3");
    }
}
