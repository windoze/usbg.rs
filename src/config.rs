use std::fs;
use std::io;
use std::os::unix;
use std::path::Path;

use crate::function::UsbGadgetFunction;
use crate::util::write_data;

pub struct UsbGadgetConfig<'a> {
    pub id: u8,
    pub name: &'a str,
    pub description: &'a str,
    pub functions: Vec<Box<dyn UsbGadgetFunction>>,
    pub max_power: Option<u16>,
}

impl<'a> UsbGadgetConfig<'a> {
    pub fn write_to<P1: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        configs_path: P1,
        functions_path: P2,
        lang: &str,
    ) -> io::Result<()> {
        let config_name = format!("{name}.{id}", name = self.name, id = self.id);
        let config_path = configs_path.as_ref().join(config_name);
        if !config_path.exists() {
            fs::create_dir(&config_path)?;
        }
        let config_strings_path = config_path.join("strings").join(lang);
        if !config_strings_path.exists() {
            fs::create_dir_all(&config_strings_path)?;
        }

        write_data(
            config_strings_path.join("configuration").as_path(),
            self.description.as_bytes(),
        )?;

        if let Some(max_power) = self.max_power {
            write_data(
                config_path.join("MaxPower").as_path(),
                format!("{}", max_power).as_bytes(),
            )?;
        }

        // symlink config functions
        for func in &self.functions {
            let fname = format!(
                "{func_type}.{instance}",
                func_type = func.function_type(),
                instance = func.instance_name()
            );
            let src_path = functions_path.as_ref().join(&fname);
            let dst_path = config_path.join(&fname);
            if !dst_path.exists() {
                unix::fs::symlink(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}
