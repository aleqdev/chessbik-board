use serde::{ser::SerializeSeq, Serializer, Serialize};

use super::*;

impl<T> Serialize for Board<T> 
where 
    T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer 
    {
        let mut seq = serializer.serialize_seq(Some(54))?;
        for i in self.cells.iter() {
            seq.serialize_element(i)?;
        }
        seq.end()
    }
}