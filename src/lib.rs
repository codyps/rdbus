extern crate utf8_cstr;
extern crate unix_socket;

mod types;


/**
 * A connection to a bus.
 *
 * A single system will typically have both "system" and "user" (or
 * "session") busses, but may have more. Each bus is an indepentend namespace. D-Bus service &
 * client programs may connect to one or more busses. Having more than one connection to a single
 * bus is not generally needed (instead, it is likely that one should take ownership of more than
 * one name)
 */
struct Bus {

}

impl for Bus {
    /**
     * Open the appropriate bus
     */
    fn open() -> Result<Bus, String> {
        unimplimented!();
    }

    /**
     * Open the user bus
     */
    fn open_user() -> Result<Bus, String> {
        unimplimented!();
    }

    /**
     * Open the system bus
     */
    fn open_system() -> Result<Bus, String> {
        unimplimented!();
    }
}
