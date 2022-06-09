use std::{fmt, marker::PhantomData};
use serde::de::Error;

use super::*;

pub struct BoardVisitor<T> {
    _phantom: PhantomData<T>
}

impl<T> BoardVisitor<T> {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}


impl<'de, T> serde::de::Visitor<'de> for BoardVisitor<T>
where 
    T: serde::Deserialize<'de>
{
    type Value = Board<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("ArrayKeyedMap key value sequence.")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut it = std::iter::from_fn(|| {
            Some(seq.next_element())
        });

        let it: Vec<Option<T>> = it.try_fold(Vec::with_capacity(54), |mut vec, x| {
            vec.push(x?); 
            Ok(vec)
        })?;

        let it: Vec<T> = it.into_iter().map_while(|x| x).collect();

        Ok(Board {
            cells: match it.try_into() {
                Ok(arr) => arr,
                Err(it) => return Err(A::Error::invalid_length(it.len(), &"54")),
            }
        })
    }
}
