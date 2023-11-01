use super::*;
use std::{ffi::CString, fs};

pub struct MountDir {
    target_dir: CString,
}

impl MountDir {
    pub fn new(dev: &str) -> Result<Self> {
        let mut target_dir = dirs::cache_dir().unwrap();
        target_dir.push("sprig-upload");

        let _ = fs::create_dir_all(&target_dir);

        let target_dir = CString::new(target_dir.to_str().unwrap())?;
        let dev_dir = CString::new(dev)?;
        let fs_type = CString::new("vfat")?;

        let err = unsafe {
            libc::mount(
                dev_dir.as_ptr(),
                target_dir.as_ptr(),
                fs_type.as_ptr(),
                0,
                std::ptr::null(),
            )
        };
        if err < 0 {
            unsafe {
                let mount = CString::new("mount")?;
                libc::perror(mount.as_ptr());
            }
        }

        Ok(Self { target_dir })
    }

    pub fn get_target_dir(&self) -> &str {
        self.target_dir.to_str().unwrap()
    }
}

impl Drop for MountDir {
    fn drop(&mut self) {
        unsafe {
            libc::umount(self.target_dir.as_ptr());
            let _ = fs::remove_dir(self.target_dir.to_str().unwrap());
        }
    }
}
