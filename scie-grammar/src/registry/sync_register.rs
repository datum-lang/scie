use crate::inter::IRawGrammar;
use crate::grammar::Grammar;
use std::collections::BTreeMap as Map;

trait IGrammarRepository {
    fn lookup(&self, scope_name: String) -> Box<IRawGrammar>;
    fn injections(&self, target_scope: String) -> Vec<String>;
}

pub struct SyncRegister {
    grammars: Map<String, Box<Grammar>>,
    raw_grammars: Map<String, Box<IRawGrammar>>,
    injection_grammars: Map<String, Vec<String>>,
}

impl SyncRegister {
    pub fn new() -> Self {
        SyncRegister {
            grammars: Map::new(),
            raw_grammars: Map::new(),
            injection_grammars: Map::new(),
        }
    }

    pub fn dispose(&self) {
        for (_, grammar) in self.grammars.iter() {
            grammar.dispose();
        }
    }

    pub fn add_grammar(&mut self, grammar: Box<IRawGrammar>, injection_scope_names: Option<Vec<String>>) {
        let scope_name = grammar.scope_name.clone().unwrap();
        self.raw_grammars.insert(scope_name.clone(), grammar);
        if injection_scope_names.is_some() {
            self.injection_grammars.insert(scope_name, injection_scope_names.unwrap());
        }
    }
}

impl IGrammarRepository for SyncRegister {
    fn lookup(&self, scope_name: String) -> Box<IRawGrammar> {
        let result = self.raw_grammars.get(scope_name.as_str());
        let x = result.unwrap();
        x.clone()
    }

    fn injections(&self, target_scope: String) -> Vec<String> {
        let result = self.injection_grammars.get(target_scope.as_str());
        let x = result.unwrap();
        x.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::registry::sync_register::{SyncRegister, IGrammarRepository};
    use crate::inter::IRawGrammar;

    #[test]
    fn should_add_grammar() {
        let mut register = SyncRegister::new();
        let mut grammar = IRawGrammar::new();
        grammar.scope_name = Some(String::from("demo"));
        grammar.comment = Some(String::from("comment"));

        register.add_grammar(Box::from(grammar), None);
        let get_grammar = register.lookup(String::from("demo"));

        assert_eq!("comment",get_grammar.comment.unwrap());
    }
}