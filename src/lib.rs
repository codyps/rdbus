extern crate utf8_cstr;

pub mod type_sig;
pub mod types;
pub mod message;


/**
 * A connection to a bus.
 *
 * A single system will typically have both "system" and "user" (or
 * "session") busses, but may have more. Each bus is an indepentend namespace. D-Bus service &
 * client programs may connect to one or more busses. Having more than one connection to a single
 * bus is not generally needed (instead, it is likely that one should take ownership of more than
 * one name)
 */
pub struct Bus {
    /* FIXME: allow non-unix sockets. Tcp is typically used on windows systems */
    sock: std::os::unix::net::UnixDatagram,
}

impl Bus {
    /**
     * Open the appropriate bus
     */
    pub fn open() -> Result<Bus, String> {
        unimplemented!();
    }

    /**
     * Open the user bus
     */
    pub fn open_user() -> Result<Bus, String> {
        unimplemented!();
    }

    /**
     * Open the system bus
     */
    pub fn open_system() -> Result<Bus, String> {
        unimplemented!();
    }

    /**
     * Create a new bus connection from an already openned & connected unix socket
     */
    #[cfg(unix)]
    pub fn open_unix(s: std::os::unix::net::UnixDatagram) -> Result<Bus, String> {
        Ok(Bus { sock: s } )
    }
}

