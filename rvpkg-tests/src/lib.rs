#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::thread;

    lazy_static! {
        static ref CONTAINER_ENGINE: &'static str = get_container_engine();
    }

    fn get_container_engine() -> &'static str {
        const POSSIBLE_NAMES: [&str; 2] = ["podman", "docker"];

        for name in POSSIBLE_NAMES.iter() {
            let result = Command::new(name).arg("--version").output();
            if result.is_ok() {
                return name;
            }
        }
        panic!("No container engine found")
    }

    fn start_container(name: &str) {
        let result = Command::new(CONTAINER_ENGINE.clone())
            .args(&["run", "--rm", "-t", "-d", "--name", name, "rvpkg-tests"])
            .output()
            .expect(format!("Failed to run {}", CONTAINER_ENGINE.clone()).as_str());
        if !result.status.success() {
            panic!("Failed to start container");
        }

        thread::sleep(std::time::Duration::from_secs(1));
    }

    fn run_in_container(name: &str, command: &mut Vec<&str>) -> Result<String, String> {
        let result = Command::new(CONTAINER_ENGINE.clone())
            .args(vec!["exec", name])
            .args(command)
            .output()
            .map_err(|_| format!("Failed to run {}", CONTAINER_ENGINE.clone().to_owned()))?;
        if !result.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&result.stderr));
            return Err(String::from("Failed to run command in container"));
        }
        Ok(String::from(String::from_utf8_lossy(&result.stdout)))
    }

    fn stop_container(name: &str) {
        let result = Command::new(CONTAINER_ENGINE.clone())
            .args(&["rm", "--force", name])
            .output()
            .expect(format!("Failed to run {}", CONTAINER_ENGINE.clone()).as_str());
        if !result.status.success() {
            panic!("Failed to stop container");
        }
    }

    #[test]
    fn test_container() {
        let name = "rvpkg-tests_test_container";
        start_container(name);
        stop_container(name);
    }

    #[test]
    fn test_version() {
        let name = "rvpkg-tests_test_version";
        start_container(name);

        let result = run_in_container(name, &mut vec!["rvpkg", "--version"]);

        stop_container(name);

        match result {
            Ok(output) => {
                let rvpkg_config =
                    toml::from_str::<toml::Value>(include_str!("../../rvpkg/Cargo.toml")).unwrap();

                let name = rvpkg_config["package"]["name"].as_str().unwrap();
                let version = rvpkg_config["package"]["version"].as_str().unwrap();

                let expected = format!("{} {}\n", name, version);
                assert!(
                    output == expected,
                    "Expected: {}\nGot: {}",
                    expected,
                    output
                );
            }
            Err(e) => panic!("{}", e),
        }
    }
}
