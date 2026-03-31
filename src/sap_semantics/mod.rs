pub mod entity_set;
pub mod entity_type;
pub mod property;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub trait SemanticType {
    fn member_name(&self) -> &'static str;
}

pub trait OptionalSemanticType {
    fn opt_sem_type(&self) -> String;
}
