use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::util::write_data;
use crate::Result;
use crate::UsbGadget;

#[derive(Debug, Default)]
pub struct UsbGadgetState {
    configfs_path: PathBuf,
    udc_name: String,
}

impl UsbGadgetState {
    pub fn new() -> Result<UsbGadgetState> {
        let udc_dir = PathBuf::from("/sys/class/udc");
        let mut udc_name = String::new();
        if let Ok(entries) = fs::read_dir(&udc_dir) {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        println!("Using UDC: {:?}", entry.file_name());
                        if let Ok(fname) = entry.file_name().into_string() {
                            udc_name.push_str(fname.as_str());
                            break;
                        }
                    }
                    Err(e) => Err(e)?,
                }
            }
        }

        Ok(UsbGadgetState {
            configfs_path: PathBuf::from("/sys/kernel/config/usb_gadget"),
            udc_name,
        })
    }
    
    pub fn get_configfs_path(&self) -> &Path {
        self.configfs_path.as_path()
    }

    pub fn configfs_path<P: AsRef<Path>>(&mut self, configfs_path: P) -> &mut UsbGadgetState {
        self.configfs_path = configfs_path.as_ref().to_path_buf();
        self
    }

    pub fn udc_name(&mut self, udc_name: &str) -> &mut UsbGadgetState {
        self.udc_name = String::from(udc_name);
        self
    }

    pub fn enable(&mut self, gadget: UsbGadget) -> Result<()> {
        if !self.configfs_path.exists() {
            return Err(
                io::Error::new(io::ErrorKind::Other, "ConfigFs path does not exist").into(),
            );
        }

        // write gadget to ConfigFs
        let gadget_path = self.configfs_path.join(gadget.name);
        gadget.write_to(&gadget_path)?;

        // write UDC to enable
        write_data(gadget_path.join("UDC").as_path(), self.udc_name.as_bytes())?;

        Ok(())
    }

    pub fn disable(&mut self, gadget: UsbGadget) -> Result<()> {
        if !self.configfs_path.exists() {
            return Err(
                io::Error::new(io::ErrorKind::Other, "ConfigFs path does not exist").into(),
            );
        }

        // write gadget to ConfigFs
        let gadget_path = self.configfs_path.join(gadget.name);
        gadget.write_to(&gadget_path)?;

        // NOTE: Shoule be same as `echo "" > UDC`, right?
        write_data(gadget_path.join("UDC").as_path(), "\n".as_bytes())?;

        // TODO: Tear down steps
        // 1. Unlink `$DEV_DIR`/configs/*/*.*`
        // 2. `rmdir $DEV_DIR`/configs/*/strings/*`
        // 3. `rmdir $DEV_DIR`/configs/*`
        // 4. `rmdir $DEV_DIR`/functions/*`
        // 5. `rmdir $DEV_DIR`strings/*`
        // 6. `rmdir $DEV_DIR`

        Ok(())
    }
}
