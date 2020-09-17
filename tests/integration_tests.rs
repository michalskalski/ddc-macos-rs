extern crate ddc_macos;
use ddc::Ddc;

#[test]
fn test_get_vcp_feature() {
    let mut monitors = ddc_macos::Monitor::enumerate().unwrap();
    assert_ne!(monitors.len(), 0);

    for monitor in monitors.iter_mut() {
        let input = monitor.get_vcp_feature(0x60).unwrap();
        assert_ne!(input.value(), 0);
    }
}