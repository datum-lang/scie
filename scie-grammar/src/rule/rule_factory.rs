use crate::grammar::Grammar;
use crate::inter::{ILocation, IRawCaptures, IRawRepository, IRawRule};
use crate::rule::{
    AbstractRule, BeginEndRule, BeginWhileRule, CaptureRule, IRuleRegistry, IncludeOnlyRule,
    MatchRule,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize)]
pub struct ICompilePatternsResult {
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
}

fn remove_first(s: &str) -> &str {
    let (_first, last) = s.split_at(1);
    last
}

pub struct RuleFactory {}

impl RuleFactory {
    // todo: add more rule builder
    fn compile_captures(
        captures: Option<Box<IRawCaptures>>,
        helper: &mut Grammar,
        repository: &mut IRawRepository,
    ) -> Vec<Rc<dyn AbstractRule>> {
        let mut r: Vec<Rc<dyn AbstractRule>> = vec![];
        if captures.is_some() {
            let capts = captures.unwrap();
            let mut maximum_capture_id = 0;
            for (id_str, _value) in capts.clone().map.capture_map {
                let id: i32 = id_str.parse().unwrap_or(0);
                if id > maximum_capture_id {
                    maximum_capture_id = id
                }
            }

            let rc = Rc::new(CaptureRule::empty());
            r.resize((maximum_capture_id + 1) as usize, rc);

            for (id_str, desc) in capts.clone().map.capture_map {
                // todo: figure captureId === '$vscodeTextmateLocation'
                let numeric_capture_id: usize = id_str.parse().unwrap_or(0);
                let mut retokenize_captured_with_rule_id = 0;
                let options_patterns = capts.map.capture_map.get(&*numeric_capture_id.to_string());

                if let Some(rule) = options_patterns {
                    if rule.patterns.is_some() {
                        retokenize_captured_with_rule_id =
                            RuleFactory::get_compiled_rule_id(desc.clone(), helper, repository, "");
                    }
                }

                r[numeric_capture_id] = RuleFactory::create_capture_rule(
                    helper,
                    desc.location.clone(),
                    desc.name.clone(),
                    desc.content_name.clone(),
                    retokenize_captured_with_rule_id,
                );
            }
            // todo: remove first element, because it's filled & empty.
        };

        r
    }

    pub fn create_capture_rule(
        helper: &mut Grammar,
        location: Option<ILocation>,
        name: Option<String>,
        content_name: Option<String>,
        retokenize_captured_with_rule_id: i32,
    ) -> Rc<dyn AbstractRule> {
        let id = helper.register_id();
        let rule = CaptureRule::new(
            location,
            id,
            name,
            content_name,
            retokenize_captured_with_rule_id,
        );

        helper.register_rule(Rc::new(rule));
        return helper.rule_container.get_rule(id);
    }

    pub fn compile_patterns(
        origin_patterns: Option<Vec<IRawRule>>,
        helper: &mut Grammar,
        repository: &mut IRawRepository,
    ) -> ICompilePatternsResult {
        let mut r: Vec<i32> = vec![];

        if origin_patterns.is_some() {
            for pattern in origin_patterns.clone().unwrap().iter() {
                let mut pattern_id = -1;
                if pattern.include.is_some() {
                    let include_s = pattern.include.clone().unwrap();
                    if include_s.starts_with("#") {
                        let first = remove_first(include_s.as_str());
                        if let Some(rule) = repository.map.name_map.get_mut(first).cloned() {
                            pattern_id =
                                RuleFactory::get_compiled_rule_id(*rule, helper, repository, first);
                        } else {
                            // println!(
                            //     "CANNOT find rule for scopeName: {:?}",
                            //     pattern.clone().include
                            // );
                        }
                    } else if include_s == "$base" || include_s == "$self" {
                        pattern_id = 1;
                    // pattern_id = RuleFactory::get_compiled_rule_id(
                    //     *local_included_rule.unwrap(),
                    //     helper,
                    //     repository,
                    //     String::from(include_s.as_str()),
                    // );
                    } else {
                        println!("todo: external grammar {:?}", pattern.include);
                        let mut _external_grammar_name: Option<String> = None;
                        let mut _external_grammar_include: Option<String> = None;
                        let include_string = pattern.include.as_ref().unwrap();
                        let sharp_index = include_string.find("#");

                        if let Some(index) = sharp_index {
                            let (_, last) = include_string.split_at(index);
                            _external_grammar_name = Some(String::from(last));

                            let (_, include_last) = include_string.split_at(index + 1);
                            _external_grammar_include = Some(String::from(include_last));

                            println!(
                                "{:?}, {:?}",
                                _external_grammar_name, _external_grammar_include
                            );
                        }
                    }
                } else {
                    pattern_id =
                        RuleFactory::get_compiled_rule_id(pattern.clone(), helper, repository, "");
                }

                if pattern_id != -1 {
                    let rule = helper.rule_container.get_rule(pattern_id).clone();
                    let mut skip_rule = false;
                    if rule.type_of() == "IncludeOnlyRule"
                        || rule.type_of() == "BeginEndRule"
                        || rule.type_of() == "BeginWhileRule"
                    {
                        if rule.has_missing_pattern() && rule.patterns_length() == 0 {
                            // match rule.get_rule_instance() {
                            //     RuleEnum::BeginEndRule(r) => {
                            //         println!("missing patterns -> {:?}", r);
                            //     }
                            //     _ => {}
                            // }
                            skip_rule = true;
                        }
                    }

                    if skip_rule {
                        continue;
                    }

                    r.push(pattern_id);
                }
            }
        }

        let mut has_missing_patterns = false;
        match origin_patterns {
            None => {
                if 0 != r.len() {
                    has_missing_patterns = true
                }
            }
            Some(patterns) => {
                if patterns.len() != r.len() {
                    has_missing_patterns = true
                }
            }
        }

        let result = ICompilePatternsResult {
            patterns: r,
            has_missing_patterns,
        };

        result
    }

    pub fn get_compiled_rule_id(
        mut desc: IRawRule,
        helper: &mut Grammar,
        repository: &mut IRawRepository,
        desc_name: &str,
    ) -> i32 {
        if let None = desc.id {
            let id = helper.register_id();
            desc.id = Some(id.clone());

            // since we fork logic from vscode-textmate, vscode-textmate will had duplicate some
            // rules. it will cause stackoverflow in our version, so I decide change repository id
            // by name.
            if desc_name != "" {
                if repository.map.name_map.get(desc_name).is_some() {
                    repository.map.name_map.get_mut(desc_name).unwrap().id = Some(id);
                }
            }

            if let Some(match_s) = desc.match_s {
                let rule_factory = RuleFactory::compile_captures(desc.captures, helper, repository);
                let match_rule =
                    MatchRule::new(desc.location, id, desc.name, match_s, rule_factory);

                return helper.register_rule(Rc::new(match_rule));
            };

            if let None = desc.begin {
                if desc.repository.is_some() {
                    desc.repository
                        .unwrap()
                        .map
                        .name_map
                        .extend(repository.clone().map.name_map);
                }

                let mut patterns = desc.patterns;
                if let None = patterns {
                    if let Some(include) = desc.include {
                        let mut raw_rule = IRawRule::new();
                        raw_rule.include = Some(include);

                        patterns = Some(vec![raw_rule])
                    }
                }

                let rule_factory = RuleFactory::compile_patterns(patterns, helper, repository);
                let include_only_rule = IncludeOnlyRule::new(
                    desc.location,
                    id,
                    desc.name,
                    desc.content_name,
                    rule_factory,
                );

                return helper.register_rule(Rc::new(include_only_rule));
            }

            let begin_captures;
            match desc.begin_captures {
                None => begin_captures = desc.captures.clone(),
                Some(..) => begin_captures = desc.begin_captures.clone(),
            }

            if let Some(_) = desc._while {
                let while_captures;
                match desc.while_captures {
                    None => while_captures = desc.captures.clone(),
                    Some(..) => while_captures = desc.while_captures.clone(),
                }

                let compile_begin_captures =
                    RuleFactory::compile_captures(begin_captures, helper, repository);
                let compile_while_captures =
                    RuleFactory::compile_captures(while_captures, helper, repository);
                let pattern_factory =
                    RuleFactory::compile_patterns(desc.patterns, helper, repository);

                let begin_while_rule = BeginWhileRule::new(
                    desc.location,
                    id,
                    desc.name,
                    desc.content_name,
                    desc.begin,
                    compile_begin_captures,
                    desc._while.unwrap(),
                    compile_while_captures,
                    pattern_factory,
                );

                return helper.register_rule(Rc::new(begin_while_rule));
            }

            let end_captures;
            match desc.end_captures {
                None => end_captures = desc.captures.clone(),
                Some(..) => end_captures = desc.end_captures.clone(),
            }

            let begin_rule_factory =
                RuleFactory::compile_captures(begin_captures, helper, repository);
            let end_rule_factory = RuleFactory::compile_captures(end_captures, helper, repository);
            let pattern_factory = RuleFactory::compile_patterns(desc.patterns, helper, repository);

            let begin_end_rule = BeginEndRule::new(
                desc.location,
                id,
                desc.name,
                desc.content_name,
                desc.begin.unwrap(),
                begin_rule_factory,
                desc.end.unwrap(),
                end_rule_factory,
                desc.apply_end_pattern_last,
                pattern_factory,
            );

            return helper.register_rule(Rc::new(begin_end_rule));
        }

        desc.id.unwrap()
    }
}
