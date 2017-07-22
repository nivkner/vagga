use std::ffi::CString;
use std::io::Error as IoError;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use libc::{c_int};
use libc::{open, close};
use libc::{O_RDONLY, O_CLOEXEC};
use nix::sched::{unshare, setns, CloneFlags};
use unshare::Namespace;

#[cfg(not(feature="containers"))]
pub fn set_namespace<P:AsRef<Path>>(path: P, ns: Namespace)
    -> Result<(), IoError>
{
    unimplemented!();
}

#[cfg(feature="containers")]
pub fn set_namespace<P:AsRef<Path>>(path: P, ns: Namespace)
    -> Result<(), IoError>
{
    let c_path = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let fd = unsafe { open(c_path.as_ptr(), O_RDONLY|O_CLOEXEC, 0) };
    if fd < 0 {
        return Err(IoError::last_os_error());
    }
    let rc = setns(fd, CloneFlags::from_bits_truncate(ns.clone_flag()));
    unsafe { close(fd) };
    rc.map_err( |_| { IoError::last_os_error() })
}

#[cfg(not(feature="containers"))]
pub fn unshare_namespace(ns: Namespace) -> Result<(), IoError> {
    unimplemented!();
}

#[cfg(feature="containers")]
pub fn unshare_namespace(ns: Namespace) -> Result<(), IoError> {
    let rc = unshare(CloneFlags::from_bits_truncate(ns.clone_flag()));
    rc.map_err( |_| { IoError::last_os_error() })
    }
