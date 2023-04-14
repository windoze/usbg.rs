extern crate usbg;

use std::fs;
use std::path::{Path, PathBuf};

use usbg::HIDFunction;
use usbg::UsbGadget;
use usbg::UsbGadgetConfig;
use usbg::UsbGadgetState;

#[allow(dead_code, unused_imports)]
#[test]
fn test_enable() {
    let mut g1 = UsbGadget::new(
        "g1",
        0x1d6b,             // Linux Foundation
        0x0104,             // Multifunction Composite Gadget
        usbg::LANGID_EN_US, // LANGID English
        "USB Armory",
        "Inverse Path",
        "0123456789",
    );
    g1.bcd_device = Some(0x0100); // version 1.0.0
    g1.bcd_usb = Some(0x0200); // USB 2.0
    let hid_function = Box::new(HIDFunction {
        instance_name: "usb0",
        protocol: usbg::HID_PROTOCOL_KEYBOARD,
        subclass: usbg::HID_SUBCLASS_BOOT,
        report_length: 8,
        report_desc: &usbg::HID_KEYBOARD_REPORT_DESC,
    });
    g1.functions.push(hid_function);
    g1.configs.push(UsbGadgetConfig {
        id: 1,
        name: "c",
        description: "USB ECM + HID",
        functions: Vec::new(),
        max_power: Some(100),
    });

    // normally this would be done already via mount but we're just testing here
    fs::create_dir_all(Path::new("/tmp/configfs/usb_gadget")).unwrap();

    let mut usb_state = UsbGadgetState::new().unwrap();
    match usb_state.enable(g1) {
        Ok(_) => println!("Enabled"),
        Err(e) => println!("Failed: {}", e),
    }

    let mut pbuf = PathBuf::from("/tmp/configfs/usb_gadget");
    assert_eq!(usb_state.get_configfs_path(), pbuf);
    pbuf.push("g1");
    pbuf.push("idVendor");
    let p = pbuf.as_path();
    assert!(p.exists());
}
