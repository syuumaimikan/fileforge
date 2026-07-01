use fileforge_core::Result;

use crate::Record;

pub trait Reader {
    fn next_record(&mut self) -> Result<Option<Record<'_>>>;
}
