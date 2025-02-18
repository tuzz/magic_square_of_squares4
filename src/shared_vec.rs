use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct SharedVec(pub Arc<Mutex<Vec<u32>>>);

impl SharedVec {
    pub fn into_inner(self) -> Vec<u32> {
        Mutex::into_inner(Arc::into_inner(self.0).unwrap()).unwrap()
    }

    pub fn push(&self, number: u32) {
        self.0.lock().unwrap().push(number);
    }

    pub fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }
}

impl Serialize for SharedVec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.lock().unwrap().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for SharedVec {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(Arc::new(Mutex::new(Vec::deserialize(deserializer).unwrap()))))
    }
}
