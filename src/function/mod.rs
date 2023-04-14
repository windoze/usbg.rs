pub mod hid;
pub mod ecm;
pub mod rndis;

use std::path::Path;

use crate::Result;

pub use hid::*;

pub trait UsbGadgetFunction {
    fn instance_name(&self) -> &str;
    fn function_type(&self) -> &str;
    fn write_to(&self, functions_path: &Path) -> Result<()>;
}