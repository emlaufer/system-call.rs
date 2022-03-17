// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for x86-64 MacOS.

use core::arch::asm;

pub mod nr;

pub type SyscallReturn = (usize, bool);

#[inline(always)]
pub unsafe fn syscall0(mut n: usize) -> SyscallReturn {
    /*asm!("syscall"
    : "+{rax}"(n)
    :
    : "rcx", "r11", "memory"
    : "volatile");*/
    let n = n + nr::SYSCALL_MAGIC;
    let mut ret;
    let mut err: i32;
    asm!("syscall",
         "sbb {}, {}", // set err to -1 if carry flag set
         inlateout("rax") n => ret,
         lateout("rcx") _,
         lateout("r11") _,
         lateout(reg) err,
         options(nostack));
    let err_bool = if err == 0 { false } else { true };
    (ret, err_bool)
}

#[inline(always)]
pub unsafe fn syscall1(mut n: usize, a1: usize) -> SyscallReturn {
    /*asm!("syscall"
    : "+{rax}"(n)
    : "{rdi}"(a1)
    : "rcx", "r11", "memory" : "volatile");*/
    let n = n + nr::SYSCALL_MAGIC;
    let mut ret;
    let mut err: i32;
    asm!("syscall",
		 "sbb {}, {}",
         inlateout("rax") n => ret,
         in("rdi") a1,
         lateout("rcx") _,
         lateout("r11") _,
         lateout(reg) err,
         options(nostack));
    let err_bool = if err == 0 { false } else { true };
    (ret, err_bool)
}

#[inline(always)]
pub unsafe fn syscall2(mut n: usize, a1: usize, a2: usize) -> SyscallReturn {
    /*asm!("syscall"
    : "+{rax}"(n)
    : "{rdi}"(a1) "{rsi}"(a2)
    : "rcx", "r11", "memory"
    : "volatile");*/
    let n = n + nr::SYSCALL_MAGIC;
    let mut ret;
    let mut err: i32;
    asm!("syscall",
		 "sbb {}, {}",
         inlateout("rax") n => ret,
         in("rdi") a1,
         in("rsi") a2,
         lateout("rcx") _,
         lateout("r11") _,
         lateout(reg) err,
         options(nostack));
    let err_bool = if err == 0 { false } else { true };
    (ret, err_bool)
}

#[inline(always)]
pub unsafe fn syscall3(mut n: usize, a1: usize, a2: usize, a3: usize) -> SyscallReturn {
    /*asm!("syscall"
    : "+{rax}"(n)
    : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3)
    : "rcx", "r11", "memory"
    : "volatile");*/
    let n = n + nr::SYSCALL_MAGIC;
    let mut ret;
    let mut err: i32;
    asm!("syscall",
		 "sbb {}, {}",
         inlateout("rax") n => ret,
         in("rdi") a1,
         in("rsi") a2,
         in("rdx") a3,
         lateout("rcx") _,
         lateout("r11") _,
         lateout(reg) err,
         options(nostack));
    let err_bool = if err == 0 { false } else { true };
    (ret, err_bool)
}

#[inline(always)]
pub unsafe fn syscall4(mut n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> SyscallReturn {
    /*asm!("syscall"
    : "+{rax}"(n)
    : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4)
    : "rcx", "r11", "memory"
    : "volatile");*/
    let n = n + nr::SYSCALL_MAGIC;
    let mut ret;
    let mut err: i32;
    asm!("syscall",
		 "sbb {}, {}",
         inlateout("rax") n => ret,
         in("rdi") a1,
         in("rsi") a2,
         in("rdx") a3,
         in("r10") a4,
         lateout("rcx") _,
         lateout("r11") _,
         lateout(reg) err,
         options(nostack));
    let err_bool = if err == 0 { false } else { true };
    (ret, err_bool)
}

#[inline(always)]
pub unsafe fn syscall5(
    mut n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> SyscallReturn {
    /*asm!("syscall"
    : "+{rax}"(n)
    : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)
    : "rcx", "r11", "memory"
    : "volatile");*/
    let n = n + nr::SYSCALL_MAGIC;
    let mut ret;
    let mut err: i32;
    asm!("syscall",
		 "sbb {}, {}",
         inlateout("rax") n => ret,
         in("rdi") a1,
         in("rsi") a2,
         in("rdx") a3,
         in("r10") a4,
         in("r8") a5,
         lateout("rcx") _,
         lateout("r11") _,
         lateout(reg) err,
         options(nostack));
    let err_bool = if err == 0 { false } else { true };
    (ret, err_bool)
}

#[inline(always)]
pub unsafe fn syscall6(
    mut n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> SyscallReturn {
    /*asm!("syscall"
    : "+{rax}"(n)
    : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)"{r9}"(a6)
    : "rcx", "r11", "memory"
    : "volatile");*/
    let n = n + nr::SYSCALL_MAGIC;
    let mut ret;
    let mut err: i32;
    asm!("syscall",
		 "sbb {}, {}",
         inlateout("rax") n => ret,
         in("rdi") a1,
         in("rsi") a2,
         in("rdx") a3,
         in("r10") a4,
         in("r8") a5,
         in("r9") a6,
         lateout("rcx") _,
         lateout("r11") _,
         lateout(reg) err,
         options(nostack));
    let err_bool = if err == 0 { false } else { true };
    (ret, err_bool)
}
