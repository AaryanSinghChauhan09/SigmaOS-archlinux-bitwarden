//! Integration tests for SigmaOS-archlinux-bitwarden.

use sigmaos_archlinux_bitwarden_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
