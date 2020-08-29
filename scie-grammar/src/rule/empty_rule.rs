use crate::rule::AbstractRule;

#[derive(Clone, Debug, Serialize)]
pub struct EmptyRule {}

impl AbstractRule for EmptyRule {
    fn id(&self) -> i32 { 0 }
    fn type_of(&self) -> String {
        String::from("EmptyRule")
    }
}
