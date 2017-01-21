//! DBus "Valid Names" are strings with extra restrictions. These are used to refer to various
//! things on dbus at runtime: Services, Objects, and Interfaces each have a particular name
//! format.

extern crate utf8_cstr;

use std::os::raw::c_char;
use std::{str};
use std::ffi::CStr;
use std::mem::{transmute};
use std::ops::{Deref};
use std::result;

/**
 * A wrapper which promises it always holds a valid dbus object path
 *
 * Requirements (from dbus spec 0.26):
 *
 * - path must begin with ASCII '/' and consist of elements separated by slash characters
 * - each element must only contain the ASCII characters '[A-Z][a-z][0-9]_'
 * - No element may be the empty string
 * - Multiple '/' characters may not occur in sequence
 * - A trailing '/' character is not allowed unless the path is the root path
 * - Further, sd-bus additionally requires nul ('\0') termination of paths.
 */
#[derive(Debug)]
pub struct ObjectPath {
    inner: CStr,
}

impl ObjectPath {
    /**
     * Create a path reference from a u8 slice. Performs all checking needed to ensure requirements
     * are met.
     */
    pub fn from_bytes(b: &[u8]) -> result::Result<&ObjectPath, &'static str> {
        if b.len() < 1 {
            return Err("Path must have at least 1 character ('/')");
        }

        if b.len() > 255 {
            return Err("Path may not have more than 255 characters");
        }

        if b[0] != b'/' as u8 {
            return Err("Path must begin with '/'");
        }

        for w in b.windows(2) {
            let prev = w[0];
            let c = w[1];

            match c {
                b'/' => {
                    if prev == b'/' {
                        return Err("Path must not have 2 '/' next to each other");
                    }
                }
                b'A'...b'Z' | b'a'...b'z' | b'0'...b'9' | b'_' => {
                    // Ok
                }
                b'\0' => {
                    if prev == b'/' && b.len() != 2 {
                        return Err("Path must not end in '/' unless it is the root path");
                    }

                    return Ok(unsafe { ObjectPath::from_bytes_unchecked(b) });
                }
                _ => {
                    return Err("Invalid character in path, only '[A-Z][a-z][0-9]_/' allowed");
                }
            }
        }

        return Err("Path must be terminated in a '\\0' byte (for use by sd-bus)");
    }

    #[inline]
    pub unsafe fn from_bytes_unchecked(b: &[u8]) -> &ObjectPath {
        transmute(b)
    }

    #[inline]
    pub unsafe fn from_ptr_unchecked<'b>(b: *const c_char) -> &'b ObjectPath {
       Self::from_bytes_unchecked(CStr::from_ptr(b).to_bytes())
    }
}

impl Deref for ObjectPath {
    type Target = CStr;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[test]
fn t_path() {
    ObjectPath::from_bytes(b"/\0").unwrap();
    ObjectPath::from_bytes(b"\0").err().unwrap();
    ObjectPath::from_bytes(b"/").err().unwrap();
    ObjectPath::from_bytes(b"/h\0").unwrap();
    ObjectPath::from_bytes(b"/hello\0").unwrap();
    ObjectPath::from_bytes(b"/hello/\0").err().unwrap();
    ObjectPath::from_bytes(b"/hello/goodbye/013/4/HA\0").unwrap();
    ObjectPath::from_bytes(b"/hello/goodbye/013/4?/HA\0").err().unwrap();
}

/**
 * A wrapper which promises it always holds a validated dbus interface name
 */
#[derive(Debug)]
pub struct InterfaceName {
    inner: CStr,
}

impl InterfaceName {
    /**
     * Create a interface name reference from a u8 slice.
     *
     * Users should be careful to ensure all the following
     * requirements are met:
     *
     * dbus spec 0.26 requires:
     *  composed of 1 or more elements seperated by a period ('.') character.
     *  Elements contain at least 1 character
     *  Elements must contain only the ASCII characters '[A-Z][a-z][0-9]_' and must not begin with
     *    a digit
     *  Interface names must contain at least one '.' character (and thus at least 2 elements)
     *  Interface names must not being with a '.' character
     * sd-bus additionally requires nul ('\0') termination of the interface name.
     */
    pub fn from_bytes(b: &[u8]) -> result::Result<&InterfaceName, &'static str> {

        if b.len() < 1 {
            return Err("Name must have more than 0 characters");
        }

        if b.len() > 255 {
            return Err("Name may not have more than 255 characters");
        }

        match b[0] {
            b'.' => return Err("Name must not begin with '.'"),
            b'A'...b'Z' | b'a'...b'z' | b'_' => {
                // Ok
            }
            _ => return Err("Name must only begin with '[A-Z][a-z]_'"),
        }


        let mut periods = 0;
        for w in b.windows(2) {
            let prev = w[0];
            let c = w[1];
            match c {
                b'.' => {
                    if prev == b'.' {
                        return Err("Name must not have 2 '.' next to each other");
                    }

                    periods += 1;
                }
                b'A'...b'Z' | b'a'...b'z' | b'_' => {
                    // Ok
                }
                b'0'...b'9' => {
                    if prev == b'.' {
                        return Err("Name element must not start with '[0-9]'");
                    }
                    // otherwise, Ok
                }
                b'\0' => {
                    if prev == b'.' && b.len() != 1 {
                        return Err("Name must not end in '.'");
                    }

                    if periods < 1 {
                        return Err("Name must have at least 2 elements");
                    }
                    return Ok(unsafe { InterfaceName::from_bytes_unchecked(b) });
                }
                _ => {
                    return Err("Invalid character in interface name, only '[A-Z][a-z][0-9]_\\.' \
                                allowed");
                }
            }
        }

        return Err("Name must be terminated in a '\\0' byte (for use by sd-bus)");
    }

    /// Unsafety:
    ///
    ///  - `b` must be a nul terminated string
    ///  - `b` must contain a valid interface
    #[inline]
    pub unsafe fn from_bytes_unchecked(b: &[u8]) -> &InterfaceName {
        transmute(b)
    }

    /// Unsafety:
    ///
    ///  - lifetime `'a` must be valid
    ///  - `b` must be a nul terminated string
    ///  - `b` must contain a valid interface
    #[inline]
    pub unsafe fn from_ptr_unchecked<'a>(b: *const c_char) -> &'a Self {
        Self::from_bytes_unchecked(CStr::from_ptr(b).to_bytes_with_nul())
    }
}

impl Deref for InterfaceName {
    type Target = CStr;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


#[test]
fn t_interface() {
    InterfaceName::from_bytes(b"12\0").err().unwrap();
    InterfaceName::from_bytes(b"a\0").err().unwrap();
    InterfaceName::from_bytes(b"a.b\0").unwrap();
    InterfaceName::from_bytes(b"a.b.3\0").err().unwrap();
    InterfaceName::from_bytes(b"A.Z.xar.yfds.d3490\0").unwrap();
    InterfaceName::from_bytes(b"a.b.c\0").unwrap();
    InterfaceName::from_bytes(b"a.b.c?\0").err().unwrap();
}

#[derive(Debug)]
pub struct BusName {
    inner: CStr,
}

impl BusName {
    /**
     * Create a bus name reference from a u8 slice.
     *
     * Users should be careful to ensure all the following
     * requirements are met:
     *
     * dbus spec 0.26 requires:
     *  unique names start with a ':'. well-known names do not.
     *  composed of one or more elemenets seperated by a period '.'
     *  all elements must be at least 1 character
     *  elements can contain only the ASCII characters '[A-Z][a-z][0-9]_-'.
     *  elements part of a unique name may begin with a digit. elements in all other bus names must
     *    not begin with a digit.
     *  must contain at least 1 '.', and thus at least 2 elements
     *  must not begin with '.'
     *  must be less than the maximum name length (255)
     *
     * sd-bus additionally requires nul ('\0') termination of the bus name.
     */
    pub fn from_bytes(b: &[u8]) -> result::Result<&Self, &'static str> {

        if b.len() < 1 {
            return Err("Name must have more than 0 characters");
        }

        if b.len() > 255 {
            return Err("Name must not be greater than 255 characters");
        }

        let mut is_unique = false;
        match b[0] {
            b'.' => return Err("Name must not begin with '.'"),
            b'A'...b'Z' | b'a'...b'z' | b'_' | b'-' => {
                // Ok
            }
            b':' => {
                is_unique = true; /* Ok */
            }
            _ => return Err("Name must only begin with '[A-Z][a-z]_'"),
        }

        let mut periods = 0;
        for w in b.windows(2) {
            let prev = w[0];
            let c = w[1];
            match c {
                b'.' => {
                    if prev == b'.' || prev == b':' {
                        return Err("Elements may not be empty");
                    }

                    periods += 1;
                }
                b'A'...b'Z' | b'a'...b'z' | b'_' | b'-' => {
                    // Ok
                }
                b'0'...b'9' => {
                    if prev == b'.' && !is_unique {
                        return Err("Name element must not start with '[0-9]'");
                    }
                    // otherwise, Ok
                }
                b'\0' => {
                    if prev == b'.' && b.len() != 1 {
                        return Err("Name must not end in '.'");
                    }

                    if periods < 1 {
                        return Err("Name must have at least 2 elements");
                    }
                    return Ok(unsafe { BusName::from_bytes_unchecked(b) });
                }
                _ => {
                    return Err("Invalid character in bus name, only '[A-Z][a-z][0-9]_\\.' allowed");
                }
            }
        }

        return Err("Name must be terminated in a '\\0' byte (for use by sd-bus)");
    }

    #[inline]
    pub unsafe fn from_bytes_unchecked(b: &[u8]) -> &Self {
        transmute(b)
    }

    #[inline]
    pub unsafe fn from_ptr_unchecked<'a>(b: *const c_char) -> &'a Self {
        Self::from_bytes_unchecked(CStr::from_ptr(b).to_bytes())
    }
}

impl Deref for BusName {
    type Target = CStr;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[test]
fn t_busname() {
    BusName::from_bytes(b"a.b\0").unwrap();
    BusName::from_bytes(b"a.b").err().unwrap();
    BusName::from_bytes(b"a\0").err().unwrap();
    BusName::from_bytes(b"a.b?\0").err().unwrap();
    BusName::from_bytes(b"a.b-c.a0\0").unwrap();
    BusName::from_bytes(b"a.b-c.0a\0").err().unwrap();
    BusName::from_bytes(b":a.b-c\0").unwrap();
    BusName::from_bytes(b":a.b-c.1\0").unwrap();
}

#[derive(Debug)]
pub struct MemberName {
    inner: CStr,
}

impl MemberName {
    /**
     * Create a member name reference from a u8 slice.
     *
     * Users should be careful to ensure all the following
     * requirements are met:
     *
     * dbus spec 0.26 requires:
     *  must only contain the ASCII characters '[A-Z][a-z][0-9]_' and may not begin with a digit
     *  must not contain the '.' character
     *  must not exceed the maximum name length (255)
     *  must be at least 1 byte in length
     *
     * sd-bus additionally requires nul ('\0') termination of the bus name.
     */
    pub fn from_bytes(b: &[u8]) -> result::Result<&Self, &'static str> {

        if b.len() < 2 {
            return Err("Name must have more than 0 characters");
        }

        if b.len() > 256 {
            return Err("Must be shorter than 255 characters");
        }

        match b[0] {
            b'A'...b'Z' | b'a'...b'z' | b'_' => {
                // Ok
            }
            _ => return Err("Must begin with '[A-Z][a-z]_'"),
        }

        for c in b {
            match *c {
                b'A'...b'Z' | b'a'...b'z' | b'0'...b'9' | b'_' => {
                    // Ok
                }
                b'\0' => return Ok(unsafe { Self::from_bytes_unchecked(b) }),
                _ => {
                    return Err("Invalid character in member name, only '[A-Z][a-z][0-9]_' allowed");
                }
            }
        }

        return Err("Name must be terminated in a '\\0' byte (for use by sd-bus)");
    }

    #[inline]
    pub unsafe fn from_bytes_unchecked(b: &[u8]) -> &Self {
        transmute(b)
    }

    #[inline]
    pub unsafe fn from_ptr_unchecked<'a>(b: *const c_char) -> &'a Self {
        Self::from_bytes_unchecked(CStr::from_ptr(b).to_bytes())
    }
}

impl Deref for MemberName {
    type Target = CStr;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[test]
fn t_member_name() {
    MemberName::from_bytes(b"abc13\0").unwrap();
    MemberName::from_bytes(b"abc.13\0").err().unwrap();
    MemberName::from_bytes(b"1234abc\0").err().unwrap();
    MemberName::from_bytes(b"abc").err().unwrap();
    MemberName::from_bytes(b"\0").err().unwrap();
    MemberName::from_bytes(b"a\0").unwrap();
}
