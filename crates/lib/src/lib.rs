pub use model_derive::Model;
use uuid::Uuid;

pub trait Model {
    fn id(&self) -> Uuid;
}
