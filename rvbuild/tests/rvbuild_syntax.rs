use nonempty::nonempty;
use rvbuild::package::{PackageArchitecture, PackageSource};
use rvbuild::read_package;
use std::path::Path;

#[test]
fn test_read_neofetch() {
    let package = read_package(Path::new("tests/examples/neofetch.rvbuild")).unwrap();

    assert_eq!(package.definition.name, "neofetch");
    assert_eq!(package.definition.version.to_string(), "7.1.0");
    assert_eq!(package.definition.revision, 1);
    assert_eq!(
        package.definition.description,
        "A command-line system information tool written in bash 3.2+"
    );
    assert_eq!(package.definition.license, "MIT");
    assert_eq!(
        package.definition.homepage,
        "https://github.com/dylanaraps/neofetch"
    );
    assert_eq!(package.definition.dependencies, vec!["bash".to_string()]);
    assert_eq!(package.definition.build_dependencies, Vec::<String>::new());
    assert_eq!(
        package.definition.provides,
        nonempty!["neofetch".to_string()]
    );
    assert_eq!(
        package.definition.architectures,
        nonempty![PackageArchitecture::Any]
    );

    let source: &PackageSource = package.build.source.get("neofetch-7.1.0.tar.gz").unwrap();
    assert_eq!(matches!(*source, PackageSource::Archive { .. }), true);

    assert_eq!(package.build.prepare, None);
    assert_eq!(
        package
            .build
            .build
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<&str>>()[0],
        "tar -xf neofetch-7.1.0.tar.gz"
    );
    let install_lines = package
        .build
        .install
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>();
    assert_eq!(install_lines[0], "cd neofetch-7.1.0");
    assert_eq!(install_lines[1], "make DESTDIR=$OUTDIR install");
    assert_eq!(
        install_lines[2],
        "install -Dm644 LICENSE $OUTDIR/usr/share/licenses/neofetch/LICENSE"
    );
    assert_eq!(package.build.post_install, None);
}
#[test]
fn test_read_invalid_arch() {
    assert!(read_package(Path::new("tests/examples/invalid/invalid_arch.rvbuild")).is_err());
}

#[test]
fn test_read_arch_any_and_other() {
    assert!(read_package(Path::new(
        "tests/examples/invalid/arch_any_and_other.rvbuild"
    ))
    .is_err());
}

#[test]
fn test_insert_variables() {
    let mut package = read_package(Path::new("tests/examples/neofetch.rvbuild")).unwrap();
    rvbuild::insert_variables(&mut package);

    assert_eq!(package.build.prepare, None);
    assert_eq!(
        package
            .build
            .build
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<&str>>()[0],
        "tar -xf neofetch-7.1.0.tar.gz"
    );
    let install_lines = package
        .build
        .install
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>();
    assert_eq!(install_lines[0], "cd neofetch-7.1.0");
    assert_eq!(
        install_lines[1],
        "make DESTDIR=/var/tmp/rvbuild/neofetch install"
    );
    assert_eq!(
        install_lines[2],
        "install -Dm644 LICENSE /var/tmp/rvbuild/neofetch/usr/share/licenses/neofetch/LICENSE"
    );
    assert_eq!(package.build.post_install, None);
}
