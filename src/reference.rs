use crate::ffi::{get_lineno, get_ref};
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Reference {
    source: String,
    line: i32,
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "source {}, line {}", self.source, self.line)
    }
}

impl Reference {
    pub fn current() -> Self {
        Reference {
            source: get_ref(),
            line: get_lineno(),
        }
    }
}

#[derive(Debug)]
pub struct CodeLine {
    pub line: i32,
    pub score: f32,
}
