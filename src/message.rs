trait DBusType {
    /*
     * - use little endian if you are supplying bytes directly (this should only be needed for the
     *   basic types)
     * - failures (potentially) occur due to violating message rules, like depth
     */
    fn encode_into(&self, &mut Message) -> Result<(),String>;

    /*
    fn decode_from(&self, &mut MessageIter) -> Result<(),String>
    */
}

impl DBusType for u32 {
    fn encode_into(&self, msg: &mut Message) -> Result<(), String>
    {
        let i = *self;
        let v = [i as u8, (i >> 8) as u8, (i >> 16) as u8, (i >> 24) as u8];
        unsafe {msg.align_to(v.len());}
        msg.data.extend(&v);
        Ok(())
    }
}
impl DBusType for bool {
    fn encode_into(&self, msg: &mut Message) -> Result<(), String>
    {
        let v = if *self { 1u32 } else { 0u32 };
        msg.append(v)
    }
}

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

    fn append<T: DBusType>(&mut self, value: T) -> Result<(), String>
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
}
