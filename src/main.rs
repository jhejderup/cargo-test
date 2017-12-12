extern crate semver;


use semver::Version;
use semver::VersionReq;


fn main() {
    let r = VersionReq::parse(">= 1.0.0").unwrap();
    let v = Version::parse("1.0.0").unwrap();
    
    assert!(r.to_string() == ">= 1.0.0".to_string());
    assert!(r.matches(&v));
}




