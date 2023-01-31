use yearly_version::Version;

#[test]
fn save_json() {
    assert_eq!(to_json(Version { year: 0, major: 0, minor: 0, patch: 0 }), "0");
    // assert_eq!(to_json(Version { year: 0, major: 0, minor: 0, patch: 1 }));
    // assert_eq!( to_json(Version { year: 0, major: 0, minor: 0, patch: 65535 }));
    // assert_eq!( to_json(Version { year: 0, major: 0, minor: 1, patch: 0 }));
    // assert_eq!(from_json("16711680"), to_json(Version { year: 0, major: 0, minor: 255, patch: 0 }));
    // assert_eq!(from_json("16777215"), to_json(Version { year: 0, major: 0, minor: 255, patch: 65535 }));
    // assert_eq!(from_json("16777216"), to_json(Version { year: 0, major: 1, minor: 0, patch: 0 }));
    // assert_eq!(from_json("16777217"), to_json(Version { year: 0, major: 1, minor: 0, patch: 1 }));
    //
    // assert_eq!(from_json("4294967295"), to_json(Version { year: 0, major: 255, minor: 255, patch: 65535 }));
    // assert_eq!(from_json("4294967296"), to_json(Version { year: 1, major: 0, minor: 0, patch: 0 }));
    // assert_eq!(from_json("4294967297"), to_json(Version { year: 1, major: 0, minor: 0, patch: 1 }));
    //
    // assert_eq!(from_json("8693049262092"), to_json(Version { year: 2024, major: 2, minor: 29, patch: 12 }));
    // assert_eq!(to_json("18446744073709551615"), to_json(Version { year: 4294967295, major: 255, minor: 255, patch: 65535 }));
}

pub fn to_json(s: Version) -> String {
    match serde_json::to_string(&s) {
        Ok(o) => o,
        Err(_) => unreachable!(),
    }
}
