use super::*;

impl<'de, T> serde::Deserialize<'de> for Board<T> 
where
    T: serde::Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        deserializer.deserialize_seq(BoardVisitor::<T>::new())
    }
}