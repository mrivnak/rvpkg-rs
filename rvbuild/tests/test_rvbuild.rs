use std::path::Path;
use rvbuild::read_package;

#[test]
fn test_read_neofetch() {
    let package = read_package(Path::new("tests/examples/neofetch.rvbuild")).unwrap();
    assert_eq!(package.definition.name, "neofetch");
    assert_eq!(package.definition.version.to_string(), "7.1.0");
    assert_eq!(package.definition.revision, 1);
    assert_eq!(package.definition.description, "A command-line system information tool written in bash 3.2+");
    assert_eq!(package.definition.license, "MIT");
    assert_eq!(package.definition.homepage, "https://github.com/dylanaraps/neofetch");
    assert_eq!(package.definition.dependencies, vec!["bash".to_string()]);
    assert_eq!(package.definition.build_dependencies, Vec::<String>::new());
    assert_eq!(package.definition.provides, vec!["neofetch".to_string()]);
    assert_eq!(package.definition.architectures, vec!["any".to_string()]);
}