
nc
===

![Build status](https://github.com/xushaohua/nc/actions/workflows/rust.yml/badge.svg)
[![Latest version](https://img.shields.io/crates/v/nc.svg)](https://crates.io/crates/nc)
[![Documentation](https://docs.rs/nc/badge.svg)](https://docs.rs/nc)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.36+-yellow.svg)
![License](https://img.shields.io/crates/l/nc.svg)

Execute system call directly without `std` or `libc`.

- [Documentation](https://docs.rs/nc)
- [Release notes](https://github.com/xushaohua/nc/releases)

Features:
- No libc required
- Access syscalls directly, via assembly
- No global errno variable, every function returns an errno instead
- Support latest kernel APIs, like io-uring and pidfd introduced in linux 5.0+

## Usage
Add this to `Cargo.toml`:
```toml
[dependencies]
nc = "0.7"
```

## Examples
Get file stat:
```rust
let mut statbuf = nc::stat_t::default();
match nc::stat("/etc/passwd", &mut statbuf) {
    Ok(_) => println!("s: {:?}", statbuf),
    Err(errno) => eprintln!("Failed to get file status, got errno: {}", errno),
}
```

Get human-readable error string:
```rust
let errno = nc::EPERM;
println!("err: {:?}", nc::strerror(errno);
```

Fork process:
```rust
let pid = nc::fork();
match pid {
    Ok(pid) => {
        if pid == 0 {
            println!("child process: {}", pid);
            let args = [""];
            let env = [""];
            match nc::execve("/bin/ls", &args, &env) {
                Ok(_) => {},
                Err(errno) => eprintln!("`ls` got err: {}", errno),
            }
        } else if pid < 0 {
            eprintln!("fork() error!");
        } else {
            println!("parent process!");
        }
    },
    Err(errno) => eprintln!("errno: {}", errno),
}
```

Kill self:
```rust
let pid = nc::getpid();
let ret = nc::kill(pid, nc::SIGTERM);
// Never reach here.
println!("ret: {:?}", ret);
```

Or handle signals:
```rust
fn handle_alarm(signum: i32) {
    assert_eq!(signum, nc::SIGALRM);
}

fn main() {
    let ret = nc::signal(nc::SIGALRM, handle_alarm as nc::sighandler_t);
    assert!(ret.is_ok());
    let remaining = nc::alarm(1);
    let ret = nc::pause();
    assert!(ret.is_err());
    assert_eq!(ret, Err(nc::EINTR));
    assert_eq!(remaining, 0);
}
```

## Stable version
For stable version of rustc, please install a C compiler (`gcc` or `clang`) first.
As `llvm_asm!` feature is unavailable in stable version.


## Platforms and Architectures
- linux
  - x86
  - x86-64
  - arm
  - aarch64
  - mips
  - mipsel
  - mips64
  - mips64el
  - powerpc64
  - s390x
- freebsd
  - x86-64

## Related projects
- [nix][nix]
- [syscall][syscall]
- [relibc][relibc]
- [Linux Syscall Support][lss]
- [syscall pkg in golang][go-syscall]

[syscall]: https://github.com/kmcallister/syscall.rs
[relibc]: https://gitlab.redox-os.org/redox-os/relibc.git
[nix]: https://github.com/nix-rust/nix
[lss]: https://chromium.googlesource.com/linux-syscall-support
[go-syscall]: https://github.com/golang/go/tree/master/src/syscall

## License
This library is release in [Apache License](LICENSE).
