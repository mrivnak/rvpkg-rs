use clap::{Arg, App, SubCommand};

struct Settings {
    verbose:    bool,
    no_confirm: bool,
    runtime:    bool,
    show_deps:  bool
}

struct Package {
    name: String,
    installed: bool,
    req_deps: Vec<String>,
    rec_deps: Vec<String>,
    opt_deps: Vec<String>,
    req_run_deps: Vec<String>,
    rec_run_deps: Vec<String>,
    opt_run_deps: Vec<String>
}

fn main() {
    // Get information from Cargo.toml
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

    // Default settings
    let settings = Settings {
        verbose: false,
        no_confirm: false,
        runtime: false,
        show_deps: false
    };

    // Setup file paths
    // TODO: paths

    // Argument parsing
    let app = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .author(AUTHORS)
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Display verbose output"))
        .arg(Arg::with_name("no-confirm")
            .short("y")
            .long("no-confirm")
            .help("Accept package updates without prompting the user"))
        .arg(Arg::with_name("runtime")
            .short("r")
            .long("runtime")
            .help("Display runtime dependencies"))
        .arg(Arg::with_name("show-deps")
            .short("d")
            .long("show-deps")
            .help("Display package dependencies"))
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
        );

    let matches = app.get_matches();
}
