pub struct GrammarMap {

}

impl GrammarMap {
    pub fn new() -> Self {
        GrammarMap {}
    }
}


#[cfg(test)]
mod tests {
    use crate::language_map::LangExtMap;

    #[test]
    fn should_build_default_maps() {
        let lang_ext_map = LangExtMap::default();
    }
}