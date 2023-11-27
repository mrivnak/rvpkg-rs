use std::process::Command;

fn main() {
    build_container();
}

fn get_container_engine() -> String {
    const POSSIBLE_NAMES: [&str; 2] = ["podman", "docker"];

    for name in POSSIBLE_NAMES.iter() {
        let result = Command::new(name).arg("--version").output();
        if result.is_ok() {
            return name.to_string();
        }
    }
    panic!("No container engine found")
}

fn build_container() {
    let container_engine = get_container_engine();
    let result = Command::new(container_engine.clone())
        .args(&["build", "-f", "rvpkg-tests/Containerfile", "-t", "rvpkg-tests", ".."])
        .output()
        .expect(format!("Failed to run {}", container_engine).as_str());
    if !result.status.success() {
        panic!("Failed to build container");
    }
}

