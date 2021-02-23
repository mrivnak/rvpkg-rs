pub fn get_lines(path: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(path).expect("Unable to read package log");

    if contents == "Unable to read package db" {
        eprintln!("Error: Unable to read package log");
        std::process::exit(1);
    }

    let data: Vec<String> = contents.lines().map(String::from).collect();

    return data;
}
pub fn get_log() -> Vec<String> {
    return get_lines("fs/var/lib/rvpkg/packages.log");
}
