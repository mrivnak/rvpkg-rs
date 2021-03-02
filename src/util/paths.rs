pub fn get_db_path() -> String {
    return String::from("/usr/share/rvpkg/packages.db");
}

pub fn get_log_path() -> String {
    return String::from("/var/lib/rvpkg/package_log.db");
}

pub fn get_config_path() -> String {
    return String::from("/etc/rvpkg.toml");
}
