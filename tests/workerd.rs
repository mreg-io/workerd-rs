use workerd::Workerd;

#[test]
fn workerd_version() {
    let version = Workerd::version().unwrap();
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
}
