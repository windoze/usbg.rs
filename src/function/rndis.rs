use std::fs;
use std::path::Path;

use crate::util::write_data;
use crate::Result;
use crate::UsbGadgetFunction;

#[derive(Clone)]
pub struct RNDISFunction<'a> {
    pub instance_name: &'a str,
    pub dev_addr: &'a str,
    pub host_addr: &'a str,
}

impl<'a> UsbGadgetFunction for RNDISFunction<'a> {
    fn instance_name(&self) -> &str {
        self.instance_name
    }

    fn function_type(&self) -> &str {
        "rndis"
    }

    fn write_to(&self, functions_path: &Path) -> Result<()> {
        let fname = format!(
            "{func_type}.{instance}",
            func_type = self.function_type(),
            instance = self.instance_name()
        );
        let function_path = functions_path.join(fname);
        fs::create_dir(&function_path)?;
        // function attributes
        write_data(
            function_path.join("dev_addr").as_path(),
            self.dev_addr.to_string().as_bytes(),
        )?;
        write_data(
            function_path.join("host_addr").as_path(),
            self.host_addr.to_string().as_bytes(),
        )?;

        Ok(())
    }
}
