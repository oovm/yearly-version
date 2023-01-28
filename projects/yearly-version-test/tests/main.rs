
use std::str::FromStr;
use yearly_version::{SemVer, Version, VersionTag};

#[test]
fn ensure_size() {
    assert_eq!(std::mem::size_of::<Version>(), 8);
    assert_eq!(std::mem::size_of::<SemVer>(), 40);
}


#[test]
fn parse_initial() {
    let sym: SemVer = Version {
        year: 0,
        major: 0,
        minor: 0,
        patch: 0,
    }.into();
    println!("{}", sym);
    let sym: SemVer = VersionTag {
        number: Version {
            year: 0,
            major: 0,
            minor: 0,
            patch: 0,
        },
        tag: "nightly".to_string(),
    }.into();
    println!("{}", sym);

    println!("{:?}", Version::from_str("0.0.0.0"))
}
