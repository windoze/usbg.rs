use std::path::Path;

use crate::Result;
use crate::config::UsbGadgetConfig;
use crate::function::UsbGadgetFunction;
use crate::util::{create_dir_if_not_exists, write_data};

// USB Language Identifiers (LANGIDs)
// http://www.usb.org/developers/docs/USB_LANGIDs.pdf
pub const LANGID_EN_US: u16 = 0x0409;

// USB Vendor and Device Identifiers
// http://www.linux-usb.org/usb.ids

pub struct UsbGadget<'a> {
    pub name: &'a str,
    pub vendor_id: u16,
    pub product_id: u16,
    pub lang: u16,
    pub product: &'a str,
    pub manufacturer: &'a str,
    pub serial_number: &'a str,
    // attribute names
    pub bcd_usb: Option<u16>,
    pub bcd_device: Option<u16>,
    pub device_class: Option<u8>,
    pub device_subclass: Option<u8>,
    pub device_protocol: Option<u8>,
    // functions
    pub functions: Vec<Box<dyn UsbGadgetFunction>>,
    // configurations
    pub configs: Vec<UsbGadgetConfig<'a>>,
}

impl<'a> UsbGadget<'a> {
    pub fn new(
        name: &'a str,
        vendor_id: u16,
        product_id: u16,
        lang: u16,
        product: &'a str,
        manufacturer: &'a str,
        serial_number: &'a str,
    ) -> UsbGadget<'a> {
        UsbGadget {
            name,
            vendor_id,
            product_id,
            lang,
            product,
            manufacturer,
            serial_number,
            bcd_usb: None,
            bcd_device: None,
            device_class: None,
            device_subclass: None,
            device_protocol: None,
            functions: Vec::new(),
            configs: Vec::new(),
        }
    }

    pub fn write_to<P: AsRef<Path>>(&self, gadget_path: P) -> Result<()> {
        create_dir_if_not_exists(&gadget_path)?;

        // vendor and product id
        write_data(
            gadget_path.as_ref().join("idVendor").as_path(),
            format!("0x{:04x}", self.vendor_id).as_bytes(),
        )?;
        write_data(
            gadget_path.as_ref().join("idProduct").as_path(),
            format!("0x{:04x}", self.product_id).as_bytes(),
        )?;

        // bcdDevice and bcdUSB
        if let Some(bcd_device) = self.bcd_device {
            write_data(
                gadget_path.as_ref().join("bcdDevice").as_path(),
                format!("0x{:04x}", bcd_device).as_bytes(),
            )?;
        }
        if let Some(bcd_usb) = self.bcd_usb {
            write_data(
                gadget_path.as_ref().join("bcdUSB").as_path(),
                format!("0x{:04x}", bcd_usb).as_bytes(),
            )?;
        }

        // string attributes
        let lang = format!("0x{:04x}", &self.lang);
        let strings_path = gadget_path.as_ref().join("strings").join(&lang);
        create_dir_if_not_exists(&strings_path)?;
        write_data(
            strings_path.join("serialnumber").as_path(),
            self.serial_number.as_bytes(),
        )?;
        write_data(
            strings_path.join("manufacturer").as_path(),
            self.manufacturer.as_bytes(),
        )?;
        write_data(
            strings_path.join("product").as_path(),
            self.product.as_bytes(),
        )?;

        // functions
        let functions_path = gadget_path.as_ref().join("functions");
        create_dir_if_not_exists(&functions_path)?;
        for func in &self.functions {
            func.write_to(functions_path.as_path())?;
        }

        // configs
        let configs_path = gadget_path.as_ref().join("configs");
        create_dir_if_not_exists(&configs_path)?;
        for config in &self.configs {
            config.write_to(
                configs_path.as_path(),
                functions_path.as_path(),
                lang.as_str(),
            )?;
        }

        Ok(())
    }
}
