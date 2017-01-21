#[derive(Debug)]
enum EncodeError {
    TooLong,
}

trait DBusType {
    /*
     * - use little endian if you are supplying bytes directly (this should only be needed for the
     *   basic types)
     * - failures (potentially) occur due to violating message rules, like depth
     */
    fn encode_into(&self, &mut Message) -> Result<(),EncodeError>;

    /*
    fn decode_from(&self, &mut MessageIter) -> Result<(),DecodeError>
    */
}

fn try_cast(v: usize) -> Result<u32, EncodeError>
{
    if v > (::std::u32::MAX as usize) {
        Err(EncodeError::TooLong)
    } else {
        Ok(v as u32)
    }
}

impl<'a, T: DBusType + ?Sized> DBusType for &'a T {
    fn encode_into(&self, msg: &mut Message) -> Result<(), EncodeError>
    {
        (*self).encode_into(msg)
    }
}

impl DBusType for u32 {
    fn encode_into(&self, msg: &mut Message) -> Result<(), EncodeError>
    {
        let i = *self;
        let v = [i as u8, (i >> 8) as u8, (i >> 16) as u8, (i >> 24) as u8];
        unsafe {msg.align_to(v.len());}
        msg.data.extend(&v);
        Ok(())
    }
}

impl DBusType for u64 {
    fn encode_into(&self, msg: &mut Message) -> Result<(), EncodeError>
    {
        let i = *self;
        let v = [
            i as u8, (i >> 8) as u8, (i >> 16) as u8, (i >> 24) as u8,
            (i >> 32) as u8, (i >> 40) as u8, (i >> 48) as u8, (i >> 56) as u8
        ];
        unsafe {msg.align_to(v.len());}
        msg.data.extend(&v);
        Ok(())
    }
}

impl DBusType for bool {
    fn encode_into(&self, msg: &mut Message) -> Result<(), EncodeError>
    {
        let v = if *self { 1u32 } else { 0u32 };
        v.encode_into(msg)
    }
}

impl DBusType for str {
    fn encode_into(&self, msg: &mut Message) -> Result<(), EncodeError>
    {
        try!(try!(try_cast(self.len())).encode_into(msg));
        msg.data.extend(self.as_bytes());
        msg.data.push(0);
        Ok(())
    }
}

impl<T: DBusType> DBusType for [T]  {
    fn encode_into(&self, msg: &mut Message) -> Result<(), EncodeError>
    {
        try!(try!(try_cast(::std::mem::size_of_val(self))).encode_into(msg));
        for e in self.iter() {
            try!(e.encode_into(msg));
        }
        Ok(())
    }
}

/*
// conflicts with the &'a T impl
impl<T: DBusType, I: Iterator<Item=T> + ExactSizeIterator + Clone> DBusType for I  {
    fn encode_into(&self, msg: &mut Message) -> Result<(), EncodeError>
    {
        try!(msg.append(try!(try_cast(self.len()))));
        let mut i = self.clone();
        for e in i {
            try!(msg.append(e));
        }
        Ok(())
    }
}
*/

pub struct Message {
    data: Vec<u8>
}

impl ::std::default::Default for Message {
    fn default() -> Message {
        Message::new()
    }
}

impl Message {
    pub fn new() -> Message {
        Message { data: vec![] }
    }

    /*
    fn append<T: Type>(&mut self, item: T) {
        unimplemented!();
    }
    */

    /* 
     * Insert padding bytes into the message in preperation for inserting a value that requires a
     * specific alignment.
     *
     * - for basic types, alignment == size.
     * - struct and dict entries are aligned to 8 byte boundaries
     * - alignment is from the start of the message
     * - padding must be minimum size, and must be zero
     *
     * unsafe:
     *
     *  - allows us to break the dbus message format requirements wrt padding
     */
    unsafe fn align_to(&mut self, align: usize)
    {
        let rem = self.data.len() % align;
        for _ in 0..rem {
            self.data.push(0);
        }
    }

    fn append<T: DBusType>(&mut self, value: T) -> Result<(), EncodeError>
    {
        value.encode_into(self)
    }
}


#[cfg(test)]
mod test {
    use super::Message;

    #[test]
    fn append()
    {
        let mut m = Message::default();  
        m.append(24u32);
        assert_eq!(m.data, [24,0,0,0]);
        m.append(true);
        assert_eq!(m.data, [24,0,0,0,1,0,0,0]);
    }

    #[test]
    fn align_to() {
        let mut m = Message::new();
        unsafe {
            m.align_to(1);
            m.align_to(2);
            m.align_to(3);
        }
        assert_eq!(m.data.len(), 0);
        m.data.push(2);
        unsafe { m.align_to(2); }
        assert_eq!(m.data, [2, 0]);
        unsafe { m.align_to(2); }
        assert_eq!(m.data, [2, 0]);
    }

    #[test]
    fn spec_example_1() {
        let mut m = Message::new();
        m.append("foo").unwrap();
        m.append("+").unwrap();
        m.append("bar").unwrap();
        assert_eq!(m.data, [
                   0x03,0,0,0,0x66,0x6f,0x6f,0x00,
                   0x01,0,0,0,0x2b,0x00,0x00,0x00,
                   0x03,0,0,0,0x62,0x61,0x72,0x00,
        ]);
    }

    #[test]
    fn spec_example_marshalling_containers() {
        let mut m = Message::new();
        m.append(&[5u64][..]).unwrap();
        assert_eq!(m.data, [
                   8,0,0,0,
                   0,0,0,0,
                   5,0,0,0,0,0,0,0
        ]);
    }

}
