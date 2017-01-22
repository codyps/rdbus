#[repr(packed)]
pub struct Bus {
    endian: Endian,
    typ: Type,
    flags: Flags,
    version: u8,

    body_size: u32,
    serial: u32,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            endian: ENDIAN_LITTLE,
            typ: TYPE_INVALID,
            flags: FLAGS_NONE,
            version: 1,

            body_size: 0,
            serial: 0,
        }
    }
}

bitflags! {
    pub flags Type: u8 {
        const TYPE_INVALID = 0,
        const TYPE_METHOD_CALL = 1,
        const TYPE_METHOD_RETURN = 2,
        const TYPE_METHOD_ERROR = 3,
        const TYPE_METHOD_SIGNAL = 4,
    }
}

bitflags! {
    pub flags Endian: u8 {
        const ENDIAN_LITTLE = b'l',
        const ENDIAN_BIG = b'b',
    }
}

bitflags! {
    pub flags Flags: u8 {
        const FLAGS_NONE = 0,
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
