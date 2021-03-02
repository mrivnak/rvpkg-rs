pub fn get_db_path() -> String {
    if cfg!(debug_assertions) {
        return String::from("fs/usr/share/rvpkg/packages.db");
    } else {
        return String::from("/usr/share/rvpkg/packages.db");
    }
}

pub fn get_log_path() -> String {
    if cfg!(debug_assertions) {
        return String::from("fs/var/lib/rvpkg/package_log.db");
    } else {
        return String::from("/var/lib/rvpkg/package_log.db");
    }
}

pub fn get_config_path() -> String {
    if cfg!(debug_assertions) {
        return String::from("fs/etc/rvpkg.toml");
    } else {
        return String::from("/etc/rvpkg.toml");
    }
}
