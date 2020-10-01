use crate::grammar::Grammar;
use crate::rule::CompiledRule;
use regex::{Regex};

lazy_static! {
    static ref HAS_BACK_REFERENCES: Regex = Regex::new(r"\\(\d+)").unwrap();
    static ref BACK_REFERENCING_END: Regex = Regex::new(r"\\(\d+)").unwrap();
}

#[derive(Clone, Debug, Serialize)]
pub struct IRegExpSourceListAnchorCache {
    a0_g0: Option<Box<CompiledRule>>,
    a0_g1: Option<Box<CompiledRule>>,
    a1_g0: Option<Box<CompiledRule>>,
    a1_g1: Option<Box<CompiledRule>>,
}

impl Default for IRegExpSourceListAnchorCache {
    fn default() -> Self {
        IRegExpSourceListAnchorCache {
            a0_g0: None,
            a0_g1: None,
            a1_g0: None,
            a1_g1: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AnchorCache {
    a0_g0: String,
    a0_g1: String,
    a1_g0: String,
    a1_g1: String,
}

impl Default for AnchorCache {
    fn default() -> Self {
        AnchorCache {
            a0_g0: String::from(""),
            a0_g1: String::from(""),
            a1_g0: String::from(""),
            a1_g1: String::from(""),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSourceList {
    pub _has_anchors: bool,
    pub _cached: Option<CompiledRule>,
    pub _anchor_cache: IRegExpSourceListAnchorCache,
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
        allow_a: bool,
        allow_g: bool,
    ) -> Box<CompiledRule> {
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
            return Box::from(self._cached.clone().unwrap());
        } else {
            if allow_a {
                if allow_g {
                    if let None = self._anchor_cache.a1_g1.clone() {
                        self._anchor_cache.a1_g1 = Some(self.resolve_anchors(allow_a, allow_g));
                    }
                    return self._anchor_cache.a1_g1.clone().unwrap();
                } else {
                    if let None = self._anchor_cache.a1_g0.clone() {
                        self._anchor_cache.a1_g0 = Some(self.resolve_anchors(allow_a, allow_g));
                    }
                    return self._anchor_cache.a1_g0.clone().unwrap();
                }
            } else {
                if allow_g {
                    if let None = self._anchor_cache.a0_g1.clone() {
                        self._anchor_cache.a0_g1 = Some(self.resolve_anchors(allow_a, allow_g));
                    }
                    return self._anchor_cache.a0_g1.clone().unwrap();
                } else {
                    if let None = self._anchor_cache.a0_g0.clone() {
                        self._anchor_cache.a0_g0 = Some(self.resolve_anchors(allow_a, allow_g));
                    }
                    return self._anchor_cache.a0_g0.clone().unwrap();
                }
            }
        }
    }

    fn resolve_anchors(&self, allow_a: bool, allow_g: bool) -> Box<CompiledRule> {
        let mut reg_exps = vec![];
        let mut rules = vec![];
        for x in self._items.iter() {
            reg_exps.push(x.resolve_anchors(allow_a, allow_g));
            rules.push(x.rule_id);
        }

        Box::from(CompiledRule::new(reg_exps, rules))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSource {
    pub source: String,
    pub rule_id: i32,
    pub has_anchor: bool,
    _anchor_cache: Option<Box<AnchorCache>>,
    pub has_back_references: bool,
}

impl RegExpSource {
    pub fn new(exp_source: String, rule_id: i32) -> RegExpSource {
        let mut has_anchor = false;

        let result: String;
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

        let anchor_cache: Option<Box<AnchorCache>> = None;

        if last_pushed_pos == 0 {
            result = exp_source.clone()
        } else {
            output.push(exp_source[last_pushed_pos..length].to_string());
            result = output.join("");
        }

        let mut reg_exp_source = RegExpSource {
            source: result,
            rule_id,
            has_anchor,
            _anchor_cache: anchor_cache,
            has_back_references: false
        };

        let cache = reg_exp_source.build_cache();
        reg_exp_source._anchor_cache = Some(Box::from(cache));

        if HAS_BACK_REFERENCES.is_match(reg_exp_source.source.clone().as_str()) {
            println!("HAS_BACK_REFERENCES: {:?}", reg_exp_source.source.clone());
            panic!("HAS_BACK_REFERENCES")
        }

        reg_exp_source
    }

    fn build_cache(&self) -> AnchorCache {
        let length = self.source.len();

        let mut a0_g0_result: Vec<String> = vec![];
        let mut a0_g1_result: Vec<String> = vec![];
        let mut a1_g0_result: Vec<String> = vec![];
        let mut a1_g1_result: Vec<String> = vec![];

        a0_g0_result.resize(length, String::from(""));
        a0_g1_result.resize(length, String::from(""));
        a1_g0_result.resize(length, String::from(""));
        a1_g1_result.resize(length, String::from(""));

        let mut pos = 0;
        let mut ch: char;
        let mut next_char: char;

        while pos < length {
            ch = self.source.clone().chars().nth(pos).unwrap();
            a0_g0_result[pos] = ch.to_string();
            a0_g1_result[pos] = ch.to_string();
            a1_g0_result[pos] = ch.to_string();
            a1_g1_result[pos] = ch.to_string();

            if ch == '\\' {
                if pos + 1 < length {
                    next_char = self.source.clone().chars().nth(pos + 1).unwrap();
                    if next_char == 'A' {
                        a0_g0_result[pos + 1] = String::from("\u{FFFF}");
                        a0_g1_result[pos + 1] = String::from("\u{FFFF}");
                        a1_g0_result[pos + 1] = String::from("A");
                        a1_g1_result[pos + 1] = String::from("A");
                    } else if next_char == 'G' {
                        a0_g0_result[pos + 1] = String::from("\u{FFFF}");
                        a0_g1_result[pos + 1] = String::from("G");
                        a1_g0_result[pos + 1] = String::from("\u{FFFF}");
                        a1_g1_result[pos + 1] = String::from("G");
                    } else {
                        a0_g0_result[pos + 1] = String::from(next_char.clone());
                        a0_g1_result[pos + 1] = String::from(next_char.clone());
                        a1_g0_result[pos + 1] = String::from(next_char.clone());
                        a1_g1_result[pos + 1] = String::from(next_char.clone());
                    }

                    pos = pos + 1;
                }
            }

            pos = pos + 1;
        }

        return AnchorCache {
            a0_g0: a0_g0_result.join(""),
            a0_g1: a0_g1_result.join(""),
            a1_g0: a1_g0_result.join(""),
            a1_g1: a1_g1_result.join(""),
        };
    }

    fn resolve_anchors(&self, allow_a: bool, allow_g: bool) -> String {
        if !self.has_anchor || self._anchor_cache.is_none() {
            return self.source.clone();
        }

        let cached = self._anchor_cache.as_ref().unwrap();
        if allow_a {
            if allow_g {
                return cached.a1_g1.clone();
            } else {
                return cached.a1_g0.clone();
            }
        } else {
            if allow_g {
                return cached.a0_g1.clone();
            } else {
                return cached.a0_g0.clone();
            }
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

    #[test]
    fn should_build_anchor_cache_for_g() {
        let source = RegExpSource::new(String::from("\\G"), 1);
        let cache = source._anchor_cache.unwrap();
        assert_eq!("\\\u{ffff}", cache.a0_g0);
        assert_eq!("\\G", cache.a0_g1);
        assert_eq!("\\\u{ffff}", cache.a1_g0);
        assert_eq!("\\G", cache.a1_g1);
    }

    #[test]
    fn should_build_anchor_cache_for_g_source() {
        let source = RegExpSource::new(String::from("\\G(?!\n)"), 1);
        let cache = source._anchor_cache.unwrap();
        assert_eq!("\\\u{ffff}(?!\n)", cache.a0_g0);
        assert_eq!("\\G(?!\n)", cache.a0_g1);
        assert_eq!("\\\u{ffff}(?!\n)", cache.a1_g0);
        assert_eq!("\\G(?!\n)", cache.a1_g1);
    }

    #[test]
    fn should_build_anchor_cache_for_long() {
        let source = RegExpSource::new(
            String::from("(^[ ]*|\\G\\s*)([^\\s]+)\\s*(=|\\?=|:=|\\+=)"),
            1,
        );
        let cache = source._anchor_cache.unwrap();
        assert_eq!(
            "(^[ ]*|\\\u{ffff}\\s*)([^\\s]+)\\s*(=|\\?=|:=|\\+=)",
            cache.a0_g0
        );
        assert_eq!("(^[ ]*|\\G\\s*)([^\\s]+)\\s*(=|\\?=|:=|\\+=)", cache.a0_g1);
        assert_eq!("(^[ ]*|\\￿\\s*)([^\\s]+)\\s*(=|\\?=|:=|\\+=)", cache.a1_g0);
        assert_eq!("(^[ ]*|\\G\\s*)([^\\s]+)\\s*(=|\\?=|:=|\\+=)", cache.a1_g1);
    }
}
