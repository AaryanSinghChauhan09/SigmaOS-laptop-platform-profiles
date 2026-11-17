//! Integration tests for SigmaOS-laptop-platform-profiles.

use sigmaos_laptop_platform_profiles_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
