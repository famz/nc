// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub type c_char = u8;

// First import architecture specific types.
#[cfg(target_arch = "aarch64")]
#[path = "aarch64/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(target_arch = "arm")]
#[path = "arm/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(target_arch = "mips")]
#[path = "mips/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(target_arch = "mips64")]
#[path = "mips64/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(target_arch = "powerpc64")]
#[path = "ppc64/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(target_arch = "s390x")]
#[path = "s390x/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(target_arch = "x86")]
#[path = "x86/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod arch;
pub use arch::*;

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
mod page;

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
pub use page::*;

#[cfg(target_arch = "aarch64")]
mod signal;
#[cfg(target_arch = "aarch64")]
pub use signal::*;

mod aio;
mod aio_abi;
mod bitsperlong;
mod bpf;
mod capability;
mod compat;
mod dqblk_xfs;
mod eventpoll;
mod fcntl;
mod fs;
mod fs_readdir;
mod futex;
mod getcpu;
mod hugetlb_encode;
mod io_uring;
mod ioctl;
mod ioctls;
mod ioprio;
mod ipc;
mod ipcbuf;
mod kcmp;
mod key;
mod limits;
mod linux_dirent;
mod linux_fs;
mod linux_fs_types;
mod linux_net;
mod linux_quota;
mod linux_signal;
mod linux_socket;
mod linux_time64;
mod linux_timex;
mod membarrier;
mod memfd;
mod mempolicy;
mod mman;
mod mount;
mod mqueue;
mod msg;
mod msgbuf;
mod perf_event;
mod personality;
mod poll;
mod posix_types;
mod prctl;
mod ptrace;
mod quota;
mod resource;
mod rseq;
mod sched;
mod sched_types;
mod seccomp;
mod sem;
mod shm;
mod shmbuf;
mod siginfo;
mod signal_defs;
mod socket;
mod sockios;
mod splice;
mod statfs;
mod swap;
mod sysctl;
mod sysinfo;
mod termbits;
mod termios;
mod time;
mod time_types;
mod times;
mod timex;
mod types;
mod uapi_fadvise;
mod uapi_fcntl;
mod uapi_in;
mod uapi_in6;
mod uapi_inotify;
mod uapi_kexec;
mod uapi_linux_tcp;
mod uapi_mman;
mod uapi_mman_common;
mod uapi_net;
mod uapi_reboot;
mod uapi_resource;
mod uapi_serial;
mod uapi_socket;
mod uapi_stat;
mod uapi_timerfd;
mod uapi_wait;
mod uapi_xattr;
mod uio;
mod utime;
mod utsname;

pub use aio::*;
pub use aio_abi::*;
pub use bitsperlong::*;
pub use bpf::*;
pub use capability::*;
pub use compat::*;
pub use dqblk_xfs::*;
pub use eventpoll::*;
pub use fcntl::*;
pub use fs::*;
pub use fs_readdir::*;
pub use futex::*;
pub use getcpu::*;
pub use hugetlb_encode::*;
pub use io_uring::*;
pub use ioctl::*;
pub use ioctls::*;
pub use ioprio::*;
pub use ipc::*;
pub use ipcbuf::*;
pub use kcmp::*;
pub use key::*;
pub use limits::*;
pub use linux_dirent::*;
pub use linux_fs::*;
pub use linux_fs_types::*;
pub use linux_net::*;
pub use linux_quota::*;
pub use linux_signal::*;
pub use linux_socket::*;
pub use linux_time64::*;
pub use linux_timex::*;
pub use membarrier::*;
pub use memfd::*;
pub use mempolicy::*;
pub use mman::*;
pub use mman::*;
pub use mount::*;
pub use mqueue::*;
pub use msg::*;
pub use msgbuf::*;
pub use perf_event::*;
pub use personality::*;
pub use poll::*;
pub use posix_types::*;
pub use prctl::*;
pub use ptrace::*;
pub use quota::*;
pub use resource::*;
pub use rseq::*;
pub use sched::*;
pub use sched_types::*;
pub use seccomp::*;
pub use sem::*;
pub use shm::*;
pub use shmbuf::*;
pub use siginfo::*;
pub use signal_defs::*;
pub use socket::*;
pub use sockios::*;
pub use splice::*;
pub use statfs::*;
pub use swap::*;
pub use sysctl::*;
pub use sysinfo::*;
pub use termbits::*;
pub use termios::*;
pub use time::*;
pub use time_types::*;
pub use times::*;
pub use timex::*;
pub use types::*;
pub use uapi_fadvise::*;
pub use uapi_fcntl::*;
pub use uapi_in::*;
pub use uapi_in6::*;
pub use uapi_inotify::*;
pub use uapi_kexec::*;
pub use uapi_linux_tcp::*;
pub use uapi_mman::*;
pub use uapi_mman_common::*;
pub use uapi_net::*;
pub use uapi_reboot::*;
pub use uapi_resource::*;
pub use uapi_serial::*;
pub use uapi_socket::*;
pub use uapi_stat::*;
pub use uapi_timerfd::*;
pub use uapi_wait::*;
pub use uapi_xattr::*;
pub use uio::*;
pub use utime::*;
pub use utsname::*;
