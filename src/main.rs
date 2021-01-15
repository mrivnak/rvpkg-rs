use clap::{Arg, App, AppSettings, SubCommand};

mod util;

struct Settings {
    verbose:    bool,
    no_confirm: bool,
    runtime:    bool,
    show_deps:  bool
}

fn main() {
    // Get information from Cargo.toml
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

    // Setup file paths
    // TODO: paths

    // Argument parsing
    let app = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .author(AUTHORS)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Display verbose output")
            .global(true))
        .arg(Arg::with_name("no-confirm")
            .short("y")
            .long("no-confirm")
            .help("Accept package updates without prompting the user")
            .global(true))
        .arg(Arg::with_name("runtime")
            .short("r")
            .long("runtime")
            .help("Display runtime dependencies")
            .global(true))
        .arg(Arg::with_name("show-deps")
            .short("d")
            .long("show-deps")
            .help("Display package dependencies")
            .global(true))
        .subcommand(SubCommand::with_name("add")
            .about("Adds package(s) to the system package list")
            .arg(Arg::with_name("PACKAGE")
                .help("Package(s) to add")
                .required(true)
                .min_values(1)
            )
        )
        .subcommand(SubCommand::with_name("check")
            .about("Displays information about a package")
            .arg(Arg::with_name("PACKAGE")
                .help("Package(s) to check")
                .required(true)
                .min_values(1)
            )
        )
        .subcommand(SubCommand::with_name("count")
            .about("Displays the number of installed packages")
        )
        .subcommand(SubCommand::with_name("list")
            .about("Displays the list of installed packages")
        )
        .subcommand(SubCommand::with_name("new")
            .about("Interactively adds a new package to the database")
        )
        .subcommand(SubCommand::with_name("search")
            .about("Searches for a package")
            .arg(Arg::with_name("SEARCH")
                .help("Search term")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("built-with")
            .about("Checks if one package is build with another")
            .arg(Arg::with_name("PACKAGE")
                .help("Package to check")
                .required(true)
            )
            .arg(Arg::with_name("DEPENDENCIES")
                .help("Dependencies to check against")
                .required(true)
                .min_values(1)
            )
        )
        .subcommand(SubCommand::with_name("tail")
            .about("Displays the last N lines of the log file")
            .arg(Arg::with_name("LINES")
                .help("Number of lines to display")
                .default_value("5")  // Will convert to int later
                .validator(is_pos_int)
            )
        );

    let matches = app.get_matches();

    let settings = Settings {
        verbose: matches.is_present("verbose"),
        no_confirm: matches.is_present("no_confirm"),
        runtime: matches.is_present("runtime"),
        show_deps: matches.is_present("show-deps")
    };

    let mut packages: Vec<util::data::Package> = Vec::new();



    match matches.subcommand_name() {
        Some("add") => {
            add(settings, packages.as_slice());
        },
        Some("built-with") => {},
        Some("check") => {},
        Some("count") => {},
        Some("list") => {},
        Some("new") => {},
        Some("search") => {},
        Some("tail") => {},
        _ => {}
    }
}

// ###### Subcommand Functions ######

fn add(settings: Settings, packages: &[util::data::Package]) {
    // TODO: implement add
}

fn built_with(settings: Settings) {
    // TODO: implement built with
}

fn check(settings: Settings) {
    // TODO: implement check
}

fn count(settings: Settings) {
    // TODO: implement count
}

fn list(settings: Settings) {
    // TODO: implement list
}

fn new(settings: Settings) {
    // TODO: implement new
}

fn search(settings: Settings) {
    // TODO: implement search
}

fn tail(settings: Settings) {
    // TODO: implement tail
}

// Miscellaneous Functions

fn is_pos_int(s: String) -> Result<(), String> {
    let test = s.parse::<u64>().is_ok();

    return if test { return Ok(()); } else { Err(String::from("Value must be a positive integer")) };
}
