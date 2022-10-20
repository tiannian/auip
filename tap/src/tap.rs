use crate::{Error, Result};
use std::os::unix::io::AsRawFd;
use tokio::fs::{File, OpenOptions};

const TUNSETIFF: libc::c_ulong = 0x400454CA;
const IFF_TAP: libc::c_int = 0x0002;
const IFF_NO_PI: libc::c_int = 0x1000;

#[repr(C)]
#[derive(Debug)]
struct ifreq {
    ifr_name: [libc::c_char; libc::IF_NAMESIZE],
    ifr_data: libc::c_int, /* ifr_ifindex or ifr_mtu */
}

fn ifreq_new(name: &str) -> ifreq {
    let mut ifreq = ifreq {
        ifr_name: [0; libc::IF_NAMESIZE],
        ifr_data: 0,
    };
    // set name.
    for (i, byte) in name.as_bytes().iter().enumerate() {
        ifreq.ifr_name[i] = *byte as libc::c_char
    }
    ifreq
}

fn ifreq_ioctl(fd: libc::c_int, cmd: libc::c_ulong, ifreq: &mut ifreq) -> libc::c_int {
    unsafe { libc::ioctl(fd, cmd as _, ifreq as *mut ifreq) }
}

pub async fn open_tap_device(name: &str) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .await?;
    let mut ifreq = ifreq_new(name);
    ifreq.ifr_data = IFF_TAP | IFF_NO_PI;
    let fd = file.as_raw_fd();
    let res = ifreq_ioctl(fd, TUNSETIFF, &mut ifreq);
    if res == -1 {
        Err(Error::StdIOError(std::io::Error::last_os_error()))
    } else {
        Ok(file)
    }
}
