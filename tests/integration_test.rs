// Integration test for SigmaOS-archlinux32-keyring
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_archlinux32_keyring as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
