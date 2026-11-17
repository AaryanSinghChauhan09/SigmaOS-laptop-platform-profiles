// Integration test for SigmaOS-laptop-platform-profiles
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use laptop_platform_profiles as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
