use yearly_version::Version;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn ensure_size() {
    assert_eq!(std::mem::size_of::<Version>(), 8);
    assert_eq!(std::mem::size_of::<semver::Version>(), 40);
}
