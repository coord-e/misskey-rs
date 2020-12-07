use crate::model::id::Id;

/// Trait for entity types that has an ID.
pub trait Entity {
    /// Gets the ID.
    fn id(&self) -> Id<Self>;
}

/// Trait for types that serves as a reference (i.e. ID) to `E`.
pub trait EntityRef<E: Entity> {
    /// Gets the reference to the entity.
    fn entity_ref(&self) -> Id<E>;
}

impl<E: Entity> EntityRef<E> for &E {
    fn entity_ref(&self) -> Id<E> {
        self.id()
    }
}

impl<E: Entity> EntityRef<E> for Id<E> {
    fn entity_ref(&self) -> Id<E> {
        *self
    }
}
impl<E: Entity> EntityRef<E> for &Id<E> {
    fn entity_ref(&self) -> Id<E> {
        **self
    }
}
