use std::ffi::OsStr;
use std::process::Command;

fn create_commands(commands: &str) -> Vec<Command> {
    commands.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts = shlex::split(l).unwrap();
            let (cmd, args) = parts.split_first().unwrap();
            let mut command = Command::new(cmd);
            command.args(args);
            command
        }).collect()
}

#[test]
fn test_create_commands() {
    let commands = create_commands("echo hello\n\nls -l\n");
    assert_eq!(commands.len(), 2);
    assert_eq!(commands[0].get_program(), "echo");
    assert_eq!(commands[0].get_args().collect::<Vec<&OsStr>>(), &["hello"]);
    assert_eq!(commands[1].get_program(), "ls");
    assert_eq!(commands[1].get_args().collect::<Vec<&OsStr>>(), &["-l"]);
}