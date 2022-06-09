use serde::{Deserialize, Deserializer};

use super::*;

impl<'de, T> Deserialize<'de> for Board<T> 
where
    T: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        deserializer.deserialize_seq(BoardVisitor::<T>::new())
    }
}