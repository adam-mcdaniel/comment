extern crate comment_strip;

use comment_strip::{strip_comments, CommentStyle};
use std::string::ToString;

pub mod c {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::C, false)
    }
}

pub mod cpp {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::C, false)
    }
}

pub mod rust {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::C, false)
    }
}

pub mod python {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::Shell, false)
    }
}

pub mod atom {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::Atom, false)
    }
}

pub mod shell {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::Shell, false)
    }
}

pub mod xml {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::XML, false)
    }
}

pub mod html {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::XML, false)
    }
}
