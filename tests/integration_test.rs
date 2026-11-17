//! Integration tests for SigmaOS-archlinux32-keyring.

use sigmaos_archlinux32_keyring_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
