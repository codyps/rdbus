/*
 * consider types for:
 *  - owned vs reference
 *  - builder vs completed
 *
 */

struct Type {
    v: String
}

impl Type {
    pub fn from_string(v: String) -> Result<Type, String> {
        /* validate */
        unimplimented!();
    }

    /*
    pub fn append_type<T: BasicType>(&mut self) {
        unimplimented!();
    }
    */

    pub fn append_type_code(&mut self, code: u8) -> Result<(), String> {
        unimplimented!();
    }
}
