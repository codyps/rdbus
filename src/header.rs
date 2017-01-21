#[repr(packed)]
pub struct Bus {
    endian: u8,
    typ: u8,
    flags: u8,
    version: u8,

    body_size: u32,
    serial: u32,
}

bitflags! {
    pub flags Endian: u8 {
        const ENDIAN_LITTLE = b'l',
        const ENDIAN_BIG = b'b',
    }
}

bitflags! {
    pub flags Flags: u8 {
        const BUS_MESSAGE_NO_REPLY_EXPECETED = 1,
        const BUS_MESSAGE_NO_AUTO_START = 2,
        const BUS_MESSAGE_ALLOW_INTERACTIVE_AUTH = 4,
    }
}

bitflags! {
    pub flags Fields: u8 {
        const INVALID = 0,
        const PATH = 1,
        const INTERFACE = 2,
        const MEMBER = 3,
        const ERROR_NMAME = 4,
        const REPLY_SERIAL = 5,
        const DESTINATION = 6,
        const SENDER = 7,
        const SIGNATURE = 8,
        const UNIX_FDS = 9,
    }
}
