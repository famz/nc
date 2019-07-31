
pub mod c;
pub mod consts;
pub mod errno;
pub mod sysno;
pub mod types;

use errno::Errno;
use sysno::Sysno;

const MAX_ERRNO: i32 = 4095;

#[inline(always)]
pub fn check_errno(ret: usize) -> Result<usize, Errno> {
    let reti = ret as isize;
    if reti < 0 && reti >= (-MAX_ERRNO) as isize {
        let reti = (-reti) as Errno;
        return Err(reti);
    } else {
        return Ok(ret);
    }
}

// From kmcallister/syscall.rs
#[inline(always)]
pub unsafe fn syscall0(n: Sysno) -> Result<usize, Errno> {
    let ret: usize;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n)
                      : "memory" "cc"
                      : "volatile");
    return check_errno(ret);
}

#[inline(always)]
pub unsafe fn syscall1(n: Sysno, a1: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1)
                      : "memory" "cc"
                      : "volatile");
    return check_errno(ret);
}

#[inline(always)]
pub unsafe fn syscall2(n: Sysno, a1: usize, a2: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2)
                      : "memory" "cc"
                      : "volatile");
    return check_errno(ret);
}

#[inline(always)]
pub unsafe fn syscall3(n: Sysno, a1: usize, a2: usize, a3: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3)
                      : "memory" "cc"
                      : "volatile");
    return check_errno(ret);
}

#[inline(always)]
pub unsafe fn syscall4(n: Sysno, a1: usize, a2: usize, a3: usize,
                       a4: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3), "{esi}"(a4)
                      : "memory" "cc"
                      : "volatile");
    return check_errno(ret);
}


#[inline(always)]
pub unsafe fn syscall5(n: Sysno, a1: usize, a2: usize, a3: usize,
                       a4: usize, a5: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3), "{esi}"(a4), "{edi}"(a5)
                      : "memory" "cc"
                      : "volatile");
    return check_errno(ret);
}

#[inline(always)]
pub unsafe fn syscall6(n: Sysno, a1: usize, a2: usize, a3: usize,
                       a4: usize, a5: usize, a6: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3), "{esi}"(a4), "{edi}"(a5) "{ebp}"(a6)
                      : "memory" "cc"
                      : "volatile");
    return check_errno(ret);
}

