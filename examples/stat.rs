extern crate nc;

fn main() {
    let mut statbuf = nc::stat_t::default();

    #[cfg(target_arch = "x86_64")]
    match nc::newfstatat(nc::AT_FDCWD, "/etc/passwd", &mut statbuf, 0) {
        Ok(_) => {
            println!("s: {:?}", statbuf);
        }
        Err(errno) => {
            eprintln!("Failed to get file status, got errno: {}", errno);
        }
    }
    #[cfg(target_arch = "arm")]
    match nc::stat("/etc/passwd", &mut statbuf) {
        Ok(_) => {
            println!("s: {:?}", statbuf);
        }
        Err(errno) => {
            eprintln!("Failed to get file status, got errno: {}", errno);
        }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "arm")))]
    match nc::fstatat(nc::AT_FDCWD, "/etc/passwd", &mut statbuf) {
        Ok(_) => {
            println!("s: {:?}", statbuf);
        }
        Err(errno) => {
            eprintln!("Failed to get file status, got errno: {}", errno);
        }
    }
}
