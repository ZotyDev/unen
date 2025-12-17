use std::any::Any;

pub trait Resource: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}
