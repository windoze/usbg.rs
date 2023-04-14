extern crate usbg;

use usbg::UsbGadget;
use usbg::UsbGadgetState;
use usbg::UsbGadgetFunction;
use usbg::UsbGadgetConfig;
use usbg::hid;
use usbg::rndis;

fn main() {
    // general setup
    let mut g1 = UsbGadget::new("g1",
                                0x1d6b, // Linux Foundation
                                0x0104, // Multifunction Composite Gadget
                                usbg::LANGID_EN_US, // LANGID English
                                "USB Armory",
                                "Inverse Path",
                                "d34db33f0123456789");
    g1.bcd_device = Some(0x0100); // version 1.0.0
    g1.bcd_usb = Some(0x0200); // USB 2.0

    // add RNDIS ethernet
    let rndis_fuction = Box::new(rndis::RNDISFunction {
                                     instance_name: "usb0",
                                     dev_addr: "1a:55:89:a2:69:41",
                                     host_addr: "1a:55:89:a2:69:42",
                                 });
    g1.functions.push(rndis_fuction.clone());

    // add HID keyboard
    let hid_function = Box::new(hid::HIDFunction {
                                    instance_name: "usb0",
                                    protocol: hid::HID_PROTOCOL_KEYBOARD,
                                    subclass: hid::HID_SUBCLASS_BOOT,
                                    report_length: 8,
                                    report_desc: &hid::HID_KEYBOARD_REPORT_DESC,
                                });
    g1.functions.push(hid_function.clone());

    // add configuration
    let c1_functions: Vec<Box<dyn UsbGadgetFunction>> = vec![hid_function.clone(), rndis_fuction.clone()];

    let c1 = UsbGadgetConfig {
        id: 1,
        name: "c",
        description: "USB Armory RNDIS + HID",
        max_power: Some(120),
        functions: c1_functions,
    };
    g1.configs.push(c1);

    let mut usb_state = UsbGadgetState::new().unwrap();
    // usb_state.udc_name("someudc.hg0");

    // if you want test against a temp directory you can uncomment this
    // let tmp_configfs = PathBuf::from("/tmp/configfs/usb_gadget");
    // let _ = fs::create_dir_all(tmp_configfs.as_path());
    // usb_state.configfs_path(tmp_configfs);

    match usb_state.enable(g1) {
        Ok(_) => println!("Enabled"),
        Err(e) => println!("Failed: {}", e),
    }
}
