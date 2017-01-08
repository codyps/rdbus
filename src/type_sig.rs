/*
 * consider types for:
 *  - owned vs reference
 *  - builder vs completed
 *
 */

pub struct Type<'a> {
    v: &'a str
}

#[derive(Debug)]
pub enum TypeError {
    Invalid(char),
    ParenUnclosed(u64),
    ElementRequired,
    ParenClosedBeforeOpen,
}

impl ::std::fmt::Display for TypeError {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            &TypeError::Invalid(c) => write!(fmt, "Type spec contained invalid character '{}'", c),
            &TypeError::ParenUnclosed(c) => write!(fmt, "Type spec left {} parens unclosed", c),
            _ => write!(fmt, "{}", ::std::error::Error::description(self))
        }
    }
}

impl ::std::error::Error for TypeError {
    fn description(&self) -> &str {
        match self {
            &TypeError::Invalid(c) => "Type spec contained invalid character",
            &TypeError::ParenUnclosed(c) => "Type spec left parens unclosed",
            &TypeError::ParenClosedBeforeOpen => "Type spec closed a paren without having any open",
            &TypeError::ElementRequired => "Type spec is missing required element for array",
        }
    }
}

impl<'a> Type<'a> {
    pub fn from_str(v: &'a str) -> Result<Type<'a>, TypeError> {
        let mut depth = 0u64;
        let mut element_required = false;

        /* validate */
        for i in v.chars() {
            match i {
                'y'|'b'|'n'|'q'|'i'|'u'|'x'|'t'|'d'|'h'|
                
                's'|'o'|'g' => {
                    /* valid types */
                    element_required = false;
                }
                'a' => {
                    element_required = true;
                }
                '(' => {
                    depth += 1;
                },
                ')' => {
                    if depth == 0 {
                        return Err(TypeError::ParenClosedBeforeOpen);
                    }
                    depth -= 1;
                    element_required = false;
                },
                a => {
                    /* invalid character */
                    return Err(TypeError::Invalid(a))
                }
            }
        }

        if depth != 0 {
            Err(TypeError::ParenUnclosed(depth))
        } else if element_required {
            Err(TypeError::ElementRequired)
        } else {
            Ok(Type { v: v })
        }
    }

    /*
    pub fn append_type<T: BasicType>(&mut self) {
        unimplemented!();
    }
    */

    pub fn append_type_code(&mut self, code: u8) -> Result<(), String> {
        unimplemented!();
    }
}

impl<'a> ::std::convert::AsRef<str> for Type<'a> {
    fn as_ref(&self) -> &str {
        self.v
    }
}

impl<'a> ::std::borrow::Borrow<str> for Type<'a> {
    fn borrow(&self) -> &str {
        self.v
    }
}

mod test {
    use super::Type;
    #[test]
    fn full_strings() {
        Type::from_str("aa").err().unwrap();
        Type::from_str("(ii").err().unwrap();
        Type::from_str("ii)").err().unwrap();

        Type::from_str("ii").unwrap();
        Type::from_str("aiai").unwrap();
        Type::from_str("(ii)(ii)").unwrap();

        Type::from_str("").unwrap();
    }
}
