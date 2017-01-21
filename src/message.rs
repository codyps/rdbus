/// A DBus message is composed of a header and a body. The header has a fixed type signature, while
/// the body has a variable type signature that is included in the message header
pub struct Message {
    body: ::marshal::Data,
}

impl ::std::default::Default for Message {
    fn default() -> Message {
        Message::new()
    }
}

impl Message {
    pub fn new() -> Message {
        Message { body: ::marshal::Data::new() }
    }

    /*
    fn append<T: Type>(&mut self, item: T) {
        unimplemented!();
    }
    */

}

