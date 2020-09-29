use crate::grammar::Grammar;
use crate::rule::CompiledRule;

#[derive(Clone, Debug, Serialize)]
pub struct AnchorCache {
    a0_g0: Option<String>,
    a0_g1: Option<String>,
    a1_g0: Option<String>,
    a1_g1: Option<String>,
}

impl Default for AnchorCache {
    fn default() -> Self {
        AnchorCache {
            a0_g0: None,
            a0_g1: None,
            a1_g0: None,
            a1_g1: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSourceList {
    pub _has_anchors: bool,
    pub _cached: Option<CompiledRule>,
    pub _anchor_cache: AnchorCache,
    pub _items: Vec<Box<RegExpSource>>,
}

impl RegExpSourceList {
    pub fn new() -> Self {
        RegExpSourceList {
            _has_anchors: false,
            _cached: None,
            _anchor_cache: Default::default(),
            _items: vec![],
        }
    }

    pub fn push(&mut self, item: Box<RegExpSource>) {
        self._items.push(item.clone());
        if item.clone().has_anchor {
            self._has_anchors = self._has_anchors || item.has_anchor;
        }
    }

    pub fn unshift(&mut self, item: Box<RegExpSource>) {
        self._items.push(item.clone());
        self._items.rotate_right(1);
        if item.clone().has_anchor {
            self._has_anchors = self._has_anchors || item.has_anchor;
        }
    }

    pub fn compile(
        &mut self,
        _grammar: &mut Grammar,
        _allow_a: bool,
        _allow_g: bool,
    ) -> CompiledRule {
        if !self._has_anchors {
            if let None = self._cached {
                let reg_exps = self
                    ._items
                    .clone()
                    .into_iter()
                    .map(|x| x.source)
                    .collect::<Vec<String>>();
                let rules = self
                    ._items
                    .clone()
                    .into_iter()
                    .map(|x| x.rule_id)
                    .collect::<Vec<i32>>();
                let compiled_rule = CompiledRule::new(reg_exps, rules);
                self._cached = Some(compiled_rule.clone());
            };
            return self._cached.clone().unwrap();
        } else {
            println!("// todo: cached {:?}", self._items);
        }

        CompiledRule::new(vec![], vec![])
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSource {
    pub source: String,
    pub rule_id: i32,
    pub has_anchor: bool,
}

impl RegExpSource {
    pub fn new(exp_source: String, rule_id: i32) -> RegExpSource {
        let mut has_anchor = false;

        let mut result: String = "".to_string();
        let length = exp_source.len();
        let mut output: Vec<String> = vec![];
        let mut last_pushed_pos = 0;

        let mut pos = 0;
        while pos < length {
            let ch = exp_source.chars().nth(pos).unwrap();
            if ch == '\\' {
                if pos + 1 < length {
                    let next_char = exp_source.chars().nth(pos + 1).unwrap();
                    if next_char == 'z' {
                        output.push(exp_source[last_pushed_pos..pos].to_string());
                        output.push(String::from("$(?!\n)(?<!\n)"));
                        last_pushed_pos = pos + 2;
                    } else if next_char == 'G' || next_char == 'A' {
                        has_anchor = true
                    }

                    pos = pos + 1;
                }
            }

            pos = pos + 1;
        }

        if last_pushed_pos == 0 {
            result = exp_source.clone()
        } else {
            output.push(exp_source[last_pushed_pos..length].to_string());
            result = output.join("");
        }

        RegExpSource {
            source: result,
            rule_id,
            has_anchor,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::rule::RegExpSource;

    #[test]
    fn should_change_resource_for_g() {
        let source = RegExpSource::new(String::from("\\G"), 1);
        assert!(source.has_anchor);
    }

    #[test]
    fn should_change_resource_for_z() {
        let source = RegExpSource::new(String::from("\\z"), 1);
        assert_eq!("$(?!\n)(?<!\n)", source.source);
    }
}

