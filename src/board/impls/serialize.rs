use serde::ser::SerializeSeq;

use super::*;

impl<T> serde::Serialize for Board<T> 
where 
    T: serde::Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut seq = serializer.serialize_seq(Some(54))?;
        for i in self.cells.iter() {
            seq.serialize_element(i)?;
        }
        seq.end()
    }
}