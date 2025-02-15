// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

fn get_file_limit() {
    let pid = nc::getpid();
    let mut res_limit = nc::rlimit64_t::default();
    let ret = nc::prlimit64(pid, nc::RLIMIT_NOFILE, None, Some(&mut res_limit));
    if ret.is_err() {
        eprintln!("Failed to get file resource limitation!");
    } else {
        println!(
            "Limit of open files, current: {}, max: {}",
            res_limit.rlim_cur, res_limit.rlim_max
        );
    }
}

fn set_file_limit() {
    let pid = nc::getpid();
    let res_limit = nc::rlimit64_t {
        rlim_cur: 512,
        rlim_max: 1024,
    };
    let ret = nc::prlimit64(pid, nc::RLIMIT_NOFILE, Some(&res_limit), None);
    if ret.is_err() {
        eprintln!("Failed to update file resource limitation!");
    } else {
        println!(
            "Update limit of open files, current: {}, max: {}",
            res_limit.rlim_cur, res_limit.rlim_max
        );
    }
}

fn main() {
    println!(
        "Open /proc/{}/limits to see limits of current process!",
        nc::getpid()
    );
    get_file_limit();
    set_file_limit();
    get_file_limit();
    let _ = nc::util::pause();
}
