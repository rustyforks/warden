use std::ops::Deref;
use std::sync::Arc;

use serde::Serialize;

#[derive(Debug)]
pub struct State<T: ?Sized>(Arc<T>);

impl<T> State<T> {
    pub fn new(value: T) -> State<T> {
        State(Arc::new(value))
    }
}

impl<T: ?Sized> State<T> {
    /// Get a reference to the inner, contained state
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Convert to the internal `Arc<T>` type
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for State<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> Clone for State<T> {
    fn clone(&self) -> Self {
        State(self.0.clone())
    }
}

impl<T: ?Sized> From<Arc<T>> for State<T> {
    fn from(arc: Arc<T>) -> Self {
        State(arc)
    }
}

impl<T> Serialize for State<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
