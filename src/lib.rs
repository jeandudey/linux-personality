#![cfg(target_os = "linux")]
//! # linux-personality
//!
//! This crate is a type safe wrapper around Linux `personality` function.
//!
//! # Examples
//! This sets the personality.
//!
//! ```rust
//! extern crate linux_personality;
//! use linux_personality::personality;
//! use linux_personality::PER_LINUX;
//!
//! let previous_personality = personality(PER_LINUX).unwrap();
//! ```
//!
//! This gets the personality.
//!
//! ```rust
//! extern crate linux_personality;
//! use linux_personality::get_personality;
//!
//! let persona = get_personality().unwrap();
//! ```

extern crate libc;
#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Personality: u32 {
        /// Have [uname(2)][uname2] report a 2.6.40+ version number rather than
        /// a 3.x version number. Added as a stopgap measure to support broken
        /// applications that could not handle the kernel version- numbering
        /// switch from 2.6.x to 3.x.
        ///
        /// [uname2]: http://man7.org/linux/man-pages/man2/uname.2.html
        const UNAME26 = 0x0020000;

        /// With this flag set, disable address-space-layout randomization.
        const ADDR_NO_RANDOMIZE = 0x0040000;

        /// User-space function pointers to signal handlers point (on certain
        /// architectures) to descriptors.
        const FDPIC_FUNCPTRS = 0x0080000;

        /// Map page 0 as read-only (to support binaries that depend on this
        /// SVr4 behavior).
        const MMAP_PAGE_ZERO = 0x0100000;

        /// With this flag set, provide legacy virtual address space layout.
        const ADDR_COMPAT_LAYOUT = 0x0200000;

        /// With this flag set, `PROT_READ` implies `PROT_EXEC` for
        /// [`mmap(2)`](http://man7.org/linux/man-pages/man2/mmap.2.html).
        const READ_IMPLIES_EXEC = 0x0400000;

        /// Limit the address space to 32 bits.
        const ADDR_LIMIT_32BIT = 0x0800000;

        /// No effects(?).
        const SHORT_INODE = 0x1000000;

        /// No effects(?).
        const WHOLE_SECONDS = 0x2000000;

        /// With this flag set, [`select(2)`][select2],
        /// [`pselect(2)`][pselect2], and [`ppoll(2)`][ppoll2] do not modify the
        /// returned timeout argument when interrupted by a signal handler.
        ///
        /// [select2]: http://man7.org/linux/man-pages/man2/select.2.html
        /// [pselect2]: http://man7.org/linux/man-pages/man2/pselect.2.html
        /// [ppoll2]: http://man7.org/linux/man-pages/man2/ppoll.2.html
        const STICKY_TIMEOUTS = 0x4000000;

        /// With this flag set, use `0xC0000000` as the offset at which to
        /// search a virtual memory chunk on [`mmap(2)`][mmap2]; otherwise use
        /// `0xFFFFE000`.
        ///
        /// [mmap2]: http://man7.org/linux/man-pages/man2/mmap.2.html
        const ADDR_LIMIT_3GB = 0x8000000;

        /// Linux
        const PER_LINUX = 0x0000;

        /// Implies `ADDR_LIMIT_32BIT`
        const PER_LINUX_32BIT = 0x0000 | Self::ADDR_LIMIT_32BIT.bits();

        /// Implies `FDPIC_FUNCPTRS`.
        const PER_LINUX_FDPIC = 0x0000 | Self::FDPIC_FUNCPTRS.bits();

        /// Implies `STICKY_TIMEOUTS` and `MMAP_PAGE_ZERO`; otherwise no
        /// effects.
        const PER_SVR4 = 0x0001 | Self::STICKY_TIMEOUTS.bits() | Self::MMAP_PAGE_ZERO.bits();

        /// Implies `STICKY_TIMEOUTS` and `SHORT_INODE`; otherwise no effects.
        const PER_SVR3 = 0x0002 | Self::STICKY_TIMEOUTS.bits() | Self::SHORT_INODE.bits();

        /// Implies `STICKY_TIMEOUTS`, `WHOLE_SECONDS`, and `SHORT_INODE`;
        /// otherwise no effects.
        const PER_SCOSVR3 = 0x0003 | Self::STICKY_TIMEOUTS.bits() | Self::WHOLE_SECONDS.bits() | Self::SHORT_INODE.bits();

        /// Implies `STICKY_TIMEOUTS` and `WHOLE_SECONDS`; otherwise no
        /// effects.
        const PER_OSR5 = 0x0003 | Self::STICKY_TIMEOUTS.bits() | Self::WHOLE_SECONDS.bits();

        /// Implies `STICKY_TIMEOUTS` and `SHORT_INODE`; otherwise no effects.
        const PER_WYSEV386 = 0x0004 | Self::STICKY_TIMEOUTS.bits() | Self::SHORT_INODE.bits();

        /// Implies STICKY_TIMEOUTS; otherwise no effects.
        const PER_ISCR4 = 0x0005 | Self::STICKY_TIMEOUTS.bits();

        /// BSD. (No effects.)
        const PER_BSD = 0x0006;

        /// Implies `STICKY_TIMEOUTS`. Divert library and dynamic linker
        /// searches to `/usr/gnemul`.  Buggy, largely unmaintained, and almost
        /// entirely unused; support was removed in Linux 2.6.26.
        const PER_SUNOS = 0x0006 | Self::STICKY_TIMEOUTS.bits();

        /// Implies `STICKY_TIMEOUTS` and `SHORT_INODE`; otherwise no effects.
        const PER_XENIX = 0x0007 | Self::STICKY_TIMEOUTS.bits() | Self::SHORT_INODE.bits();

        /// [To be documented.]
        const PER_LINUX32 = 0x0008;

        /// Implies ADDR_LIMIT_3GB.
        const PER_LINUX32_3GB = 0x0008 | Self::ADDR_LIMIT_3GB.bits();

        /// IRIX 5 32-bit. Never fully functional; support dropped in Linux
        /// 2.6.27. Implies `STICKY_TIMEOUTS`.
        const PER_IRIX32 = 0x0009 | Self::STICKY_TIMEOUTS.bits();

        /// IRIX 6 new 32-bit. Implies `STICKY_TIMEOUTS`; otherwise no effects.
        const PER_IRIXN32 = 0x000a | Self::STICKY_TIMEOUTS.bits();

        /// IRIX 6 64-bit.  Implies `STICKY_TIMEOUTS`; otherwise no effects.
        const PER_IRIX64 = 0x000b | Self::STICKY_TIMEOUTS.bits();

        /// [To be documented.]
        const PER_RISCOS = 0x000c;

        /// Implies `STICKY_TIMEOUTS`; otherwise no effects.
        const PER_SOLARIS = 0x000d | Self::STICKY_TIMEOUTS.bits();

        /// Implies `STICKY_TIMEOUTS` and `MMAP_PAGE_ZERO`; otherwise no effects.
        const PER_UW7 = 0x000e | Self::STICKY_TIMEOUTS.bits() | Self::MMAP_PAGE_ZERO.bits();

        /// OSF/1 v4.  On alpha, clear top 32 bits of iov_len in the user's
        /// buffer for compatibility with old versions of OSF/1 where iov_len
        /// was defined as. int.
        const PER_OSF4 = 0x000f;

        /// Support for 32-bit HP/UX.  This support was never complete, and was
        /// dropped so that since Linux 4.0, this value has no effect.
        const PER_HPUX = 0x0010;

        /// [to be documented]
        const PER_MASK = 0x00ff;
    }
}

/// Set the process domain execution model.
///
/// # Return value
/// On success returns the Previous `persona` is returned.
/// On failure returns `Err(())` if the kernel was unable to change personality.
pub fn personality(persona: Personality) -> Result<Personality, ()> {
    let previous_persona = unsafe { libc::personality(persona.bits() as libc::c_ulong) };
    if previous_persona == -1 {
        Err(())
    } else {
        Ok(Personality::from_bits(previous_persona as u32).unwrap())
    }
}

/// This function only gets the current `persona`.
///
/// # Return value
/// On success returns the Previous `persona` is returned.
/// On failure returns `Err(())` if the kernel was unable to retrieve personality.
pub fn get_personality() -> Result<Personality, ()> {
    let persona = unsafe { libc::personality(0xffffffff as libc::c_ulong) };
    if persona == -1 {
        Err(())
    } else {
        Ok(Personality::from_bits(persona as u32).unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_linux_per() {
        use super::personality;
        use super::PER_LINUX;

        personality(PER_LINUX).unwrap();
    }

    #[test]
    fn retrieve_per() {
        use super::get_personality;

        get_personality().unwrap();
    }
}
