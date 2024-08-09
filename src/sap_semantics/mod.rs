pub mod entity_set;
pub mod entity_type;
pub mod property;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub trait SemanticType {
    fn member_name(&self) -> Vec<u8>;
}

pub trait OptionalSemanticType {
    fn opt_sem_type<T: SemanticType>(&self, opt_self: &Option<T>) -> Vec<u8>;
}
