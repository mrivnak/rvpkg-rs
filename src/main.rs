use clap::{App, AppSettings, Arg, SubCommand};

mod util;

use util::{data::{Package, Settings}, ImportMode};

fn main() {
    // Get information from Cargo.toml
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

    let config = std::fs::read_to_string(util::paths::get_config_path()).unwrap();
    let settings: Settings = toml::from_str(config.as_str()).unwrap();

    // Argument parsing
    let app = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .author(AUTHORS)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Display verbose output")
                .global(true),
        )
        .arg(
            Arg::with_name("no-confirm")
                .short("y")
                .long("no-confirm")
                .help("Accept package updates without prompting the user")
                .global(true),
        )
        .arg(
            Arg::with_name("show-deps")
                .short("d")
                .long("show-deps")
                .help("Display package dependencies")
                .global(true),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("Display colored output")
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds package(s) to the system package list")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Package(s) to add")
                        .required(true)
                        .min_values(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("Displays information about a package")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Package(s) to check")
                        .required(true)
                        .min_values(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("count").about("Displays the number of installed packages"),
        )
        .subcommand(SubCommand::with_name("list").about("Displays the list of installed packages"))
        .subcommand(
            SubCommand::with_name("search")
                .about("Searches for a package")
                .arg(Arg::with_name("SEARCH").help("Search term").required(true)),
        )
        .subcommand(
            SubCommand::with_name("import")
                .about("Imports a package database from a csv file")
                .arg(
                    Arg::with_name("replace")
                        .short("r")
                        .long("replace")
                        .help("Replace database contents")
                        .conflicts_with("merge")
                        .conflicts_with("raw"),
                )
                .arg(
                    Arg::with_name("merge")
                        .short("m")
                        .long("merge")
                        .help("Merge database contents")
                        .conflicts_with("replace")
                        .conflicts_with("raw"),
                )
                .arg(
                    Arg::with_name("raw")
                        .short("w")
                        .long("raw")
                        .takes_value(true)
                        .value_name("DATA")
                        .help("Imports a single line in csv format")
                        .conflicts_with("PATH"), 
//                      .validator(util::csv_format)  // TODO: write validator
                )
                .arg(
                    Arg::with_name("PATH")
                        .help("Path to CSV file")
                        .required(true)
                        .conflicts_with("raw")
                        .validator(util::file_exists),
                ),
        );

    let matches = app.get_matches();

    let settings = Settings {
        verbose: matches.is_present("verbose") || settings.verbose,
        no_confirm: matches.is_present("no_confirm") || settings.no_confirm,
        show_deps: matches.is_present("show-deps") || settings.show_deps,
        color: matches.is_present("color") || settings.color,
    };

    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let packages: Vec<String> = sub_matches
                .values_of("PACKAGE")
                .unwrap()
                .map(|x| String::from(x))
                .collect(); // Get package arguments
            let packages = util::pkg::parse_packages(packages.as_slice());

            add(settings, packages.as_slice());
        }
        ("check", Some(sub_matches)) => {
            let packages: Vec<String> = sub_matches
                .values_of("PACKAGE")
                .unwrap()
                .map(|x| String::from(x))
                .collect();
            let packages = util::pkg::parse_packages(packages.as_slice());

            check(settings, packages.as_slice());
        }
        ("count", _) => {
            count(settings);
        }
        ("list", _) => {
            list();
        }
        ("search", Some(sub_matches)) => {
            let package: String = String::from(sub_matches.value_of("SEARCH").unwrap());
            search(package);
        }
        ("import", Some(sub_matches)) => {
            let raw = sub_matches.is_present("raw");

            if raw {
                let data = String::from(sub_matches.value_of("raw").unwrap());
                import_raw(data);
            } else {
                let path = String::from(sub_matches.value_of("PATH").unwrap());
                let merge = sub_matches.is_present("merge");
                let replace = sub_matches.is_present("replace");
                let mode;
                if merge {
                    mode = ImportMode::Merge;
                }
                else if replace {
                    mode = ImportMode::Replace;
                }
                else {
                    mode = get_mode();
                }

                import(path, mode);
            }
        }
        _ => {
            unreachable!()
        }
    }
}

// ###### Subcommand Functions ######

fn add(settings: Settings, packages: &[Package]) {
    util::io::print_pkg_table(&packages, &settings);

    print!("Confirm changes? (Y/n): ");
    if !settings.no_confirm {
        use std::io::Write;
        std::io::stdout().flush().unwrap();

        let line: String = text_io::read!("{}\n");
        let line = line.to_ascii_lowercase();

        if line.as_str() == "y" || line.as_str() == "" {
            // continue
        } else if line.as_str() == "n" {
            eprintln!("Exiting...");
            std::process::exit(1);
        } else {
            eprintln!("Unrecognized input, exiting...");
            std::process::exit(1);
        }
    }

    let package_names: Vec<String> = packages.iter().map(|x| x.clone().name).collect();
    util::pkg::install(package_names.as_slice());
}

fn check(settings: Settings, packages: &[Package]) {
    util::io::print_pkg_table(&packages, &settings);
}

fn count(settings: Settings) {
    let log = util::log::Log {
        path: util::paths::get_log_path(),
    };

    println!(
        "{}{}",
        log.get_size(),
        if settings.verbose {
            " package(s) installed"
        } else {
            ""
        }
    )
}

fn list() {
    let log = util::log::Log {
        path: util::paths::get_log_path(),
    };

    for line in log.get_installed() {
        println!("{}", line);
    }
}

fn search(package: String) {
    let db = util::db::DB {
        path: util::paths::get_db_path(),
    };

    let mut results = db.find_key(&package);
    results.sort();
    
    for result in results {
        println!("{}", result);
    }
}

fn import(path: String, mode: ImportMode) {
    let db = util::db::DB {
        path: util::paths::get_db_path(),
    };

    db.import_csv(path, mode);
}

fn import_raw(data: String) {
    let db = util::db::DB {
        path: util::paths::get_db_path(),
    };

    let items: Vec<&str> = data.split_terminator(",").collect();
    if items.len() > 2 {
        eprintln!("Error: invalid csv format");
        eprintln!("Expected : \"<name>,<dep0>;<dep1>;\"");
    }
    if items.len() == 1 {
        let package = items[0];

        db.add_raw(String::from(package), String::from(""));
    } else {
        let package = items[0];
        let deps = items[1];

        db.add_raw(String::from(package), String::from(deps));
    }
}

fn get_mode() -> ImportMode{
    print!("Merge or replace database? (m/r): ");
    use std::io::Write;
    std::io::stdout().flush().unwrap();

    let line: String = text_io::read!("{}\n");
    let line = line.to_ascii_lowercase();

    match line.as_str() {
        "m" => {
            return ImportMode::Merge;
        }
        "r" => {
            return ImportMode::Replace;
        }
        _ => {
            eprintln!("Unrecognized input, exiting...");
            std::process::exit(1);
        }
    }
}