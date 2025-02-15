// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod arch;
pub use arch::*;

mod _iovec;
mod _sigset;
mod _timespec;
mod _timeval;
mod _types;
mod fcntl;
mod limits;
mod mman;
mod mount;
mod poll;
mod reboot;
mod resource;
mod sched;
mod select;
mod signal;
mod socket;
mod stat;
mod syslimits;
mod time;
mod types;
mod unistd;

pub use _iovec::*;
pub use _sigset::*;
pub use _timespec::*;
pub use _timeval::*;
pub use _types::*;
pub use fcntl::*;
pub use limits::*;
pub use mman::*;
pub use mount::*;
pub use poll::*;
pub use reboot::*;
pub use resource::*;
pub use sched::*;
pub use select::*;
pub use signal::*;
pub use socket::*;
pub use stat::*;
pub use syslimits::*;
pub use time::*;
pub use types::*;
pub use unistd::*;
