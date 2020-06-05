//! Safe wrappers around functions found in libc "unistd.h" header
use std::prelude::v1::*;

#[cfg(not(target_os = "redox"))]
use crate::errno::{ Errno};
use crate::{ Result };
#[cfg(not(target_os = "redox"))]
use std::os::unix::io::RawFd;



/// Synchronize changes to a file
///
/// See also [fsync(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/fsync.html)
#[inline]
pub fn fsync(fd: RawFd) -> Result<()> {
    let res = unsafe { libc::ocall::fsync(fd) };

    Errno::result(res).map(drop)
}
