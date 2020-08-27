use crate::grammar::grammar::Grammar;
use crate::inter::{IRawCaptures, IRawRepository, IRawRule, ILocation};
use crate::rule::{BeginEndRule, BeginWhileRule, CaptureRule, IRuleRegistry, IncludeOnlyRule, MatchRule, AbstractRule, Rule};

#[derive(Clone, Debug, Serialize)]
pub struct ICompilePatternsResult {
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
}

fn remove_first(s: &str) -> &str {
    let (first, last) = s.split_at(1);
    last
}

pub struct RuleFactory {}

impl RuleFactory {
    // todo: add more rule builder
    fn compile_captures(
        captures: Option<Box<IRawCaptures>>,
        helper: &mut Grammar,
        repository: &mut IRawRepository,
    ) -> Vec<Box<dyn AbstractRule>> {
        let mut r: Vec<Box<dyn AbstractRule>> = vec![];

        if let Some(capts) = captures.clone() {
            let mut maximum_capture_id = 0;
            for (id_str, value) in capts.clone().map.capture_map {
                let id: i32 = id_str.parse().unwrap_or(0);
                if id > maximum_capture_id {
                    maximum_capture_id = id
                }
            }
            for i in 0..maximum_capture_id + 1 {
                r.push(Box::new(CaptureRule::empty()));
            }

            let cloned_capts = captures.clone().unwrap();
            for (id_str, value) in capts.clone().map.capture_map {
                let numeric_capture_id: usize = id_str.parse().unwrap_or(0);
                let mut retokenize_captured_with_rule_id = 0;
                let options_patterns = cloned_capts
                    .map
                    .capture_map
                    .get(&*numeric_capture_id.to_string());

                let desc = captures.clone().unwrap().map.capture_map[&id_str].clone();
                if let Some(rule) = options_patterns {
                    retokenize_captured_with_rule_id =
                        RuleFactory::get_compiled_rule_id(desc.clone(), helper, repository, String::from(""));
                }
                r[numeric_capture_id] = RuleFactory::create_capture_rule(helper, desc.clone().location, desc.clone().name, desc.clone().content_name, retokenize_captured_with_rule_id);
            }
        };

        r
    }

    pub fn create_capture_rule(helper: &mut Grammar, location: Option<ILocation>, name: Option<String>, content_name: Option<String>, retokenizeCapturedWithRuleId: i32) -> Box<dyn AbstractRule> {
        let id = helper.register_id();
        let rule = CaptureRule::new(
            location,
            id,
            name,
            content_name,
        );
        helper.register_rule(Box::from(rule));
        return helper.get_rule(id);
    }


    pub fn compile_patterns(
        origin_patterns: Option<Vec<IRawRule>>,
        helper: &mut Grammar,
        repository: &mut IRawRepository,
    ) -> ICompilePatternsResult {
        let mut r: Vec<i32> = vec![];

        if let Some(patterns) = origin_patterns {
            for pattern in patterns {
                let mut pattern_id = -1;
                if let Some(include_s) = pattern.clone().include {
                    let mut repository_map = repository.map.name_map.clone();

                    if include_s.starts_with("#") {
                        let first = remove_first(include_s.as_str());
                        let mut local_included_rule = repository.map.name_map.get_mut(first);
                        if let Some(mut rule) = local_included_rule.cloned() {
                            pattern_id = RuleFactory::get_compiled_rule_id(
                                // todo: replace cloned, id cannot not update
                                *rule,
                                helper,
                                repository,
                                String::from(first)
                            );
                        } else {
                            println!(
                                "CANNOT find rule for scopeName: {:?}",
                                pattern.clone().include
                            );
                        }
                    } else if include_s == "$base" || include_s == "$self" {
                        let local_included_rule = repository.map.name_map.get_mut(include_s.as_str());
                        if let Some(mut rule) = local_included_rule.cloned() {
                            pattern_id = RuleFactory::get_compiled_rule_id(
                                *rule,
                                helper,
                                repository,
                                String::from(include_s.as_str())
                            );
                        }
                    } else {
                        // todo: find the cases
                        println!("todo: {:?}", pattern.include);
                        let mut external_grammar_name: Option<String> = None;
                        let mut external_grammar_include: Option<String> = None;
                        let include_string = pattern.include.unwrap();
                        let sharp_index = include_string.find("#");

                        if let Some(index) = sharp_index {
                            let (_, last) = include_string.split_at(index);
                            external_grammar_name = Some(String::from(last));

                            let (_, include_last) = include_string.split_at(index + 1);
                            external_grammar_include = Some(String::from(include_last));

                            println!(
                                "{:?}, {:?}",
                                external_grammar_name, external_grammar_include
                            );
                        }
                    }
                } else {
                    pattern_id =
                        RuleFactory::get_compiled_rule_id(pattern, helper, repository, String::from(""));
                }

                if pattern_id != -1 {
                    let rule = helper.get_rule(pattern_id);
                    // let  mut skipRule = false;
                    // if rule.type_of() == "IncludeOnlyRule" || rule.type_of() == "BeginEndRule" || rule.type_of() == "BeginWhileRule" {
                    // skipRule = true;
                    // }

                    r.push(pattern_id);
                }
            }
        }

        let result = ICompilePatternsResult {
            patterns: r,
            has_missing_patterns: false,
        };

        result
    }

    pub fn get_compiled_rule_id(
        mut desc: IRawRule,
        helper: &mut Grammar,
        repository: &mut IRawRepository,
        desc_name: String,
    ) -> i32 {
        if let None = desc.id {
            let id = helper.register_id();
            desc.id = Some(id.clone());

            if desc_name != "" {
                if let Some(a) = repository.map.name_map.get(desc_name.as_str()).clone() {
                    repository.map.name_map.get_mut(desc_name.as_str()).unwrap().id = Some(id);
                }
            }

            if let Some(match_s) = desc.match_s {
                let rule_factory = RuleFactory::compile_captures(
                    desc.captures.clone(),
                    helper,
                    repository,
                );
                let match_rule = MatchRule::new(
                    desc.location.clone(),
                    id.clone(),
                    desc.name.clone(),
                    match_s.clone(),
                    rule_factory,
                );

                helper.register_rule(Box::new(match_rule));
                return id.clone();
            };

            if let None = desc.begin {
                if let Some(repo) = desc.repository.clone() {
                    //todo: mergeObjects
                    desc.repository.unwrap().map.name_map.extend(repository.clone().map.name_map);
                }
                let mut patterns = desc.patterns;
                if let None = patterns {
                    if let Some(include) = desc.include {
                        let mut raw_rule = IRawRule::new();
                        raw_rule.include = Some(include);
                        patterns = Some(vec![raw_rule.clone()])
                    }
                }

                let rule_factory =
                    RuleFactory::compile_patterns(patterns.clone(), helper, repository);
                let include_only_rule = IncludeOnlyRule::new(
                    desc.location.clone(),
                    id.clone(),
                    desc.name.clone(),
                    desc.content_name.clone(),
                    rule_factory,
                );

                helper.register_rule(Box::new(include_only_rule));
                return id.clone();
            }

            let mut begin_captures = desc.begin_captures.clone();
            if let None = begin_captures {
                desc.begin_captures = desc.clone().captures
            }

            let mut end_captures = desc.end_captures.clone();
            if let None = end_captures {
                desc.end_captures = desc.clone().captures
            }

            if let Some(while_s) = desc.while_s {
                let begin_rule_factory =
                    RuleFactory::compile_captures(begin_captures, helper, repository);
                let end_rule_factory =
                    RuleFactory::compile_captures(end_captures, helper, repository);
                let pattern_factory = RuleFactory::compile_patterns(
                    desc.patterns.clone(),
                    helper,
                    repository,
                );

                let begin_while_rule = BeginWhileRule::new(
                    desc.location,
                    id.clone(),
                    desc.name.clone(),
                    desc.content_name.clone(),
                    desc.begin,
                    begin_rule_factory,
                    desc.end,
                    end_rule_factory,
                    pattern_factory,
                );

                helper.register_rule(Box::new(begin_while_rule));
                return id.clone();
            }

            let begin_rule_factory =
                RuleFactory::compile_captures(begin_captures, helper, repository);
            let end_rule_factory =
                RuleFactory::compile_captures(end_captures, helper, repository);
            let pattern_factory = RuleFactory::compile_patterns(
                desc.patterns,
                helper,
                repository,
            );

            // todo: register with compile patterns
            let begin_end_rule = BeginEndRule::new(
                desc.location.clone(),
                id.clone(),
                desc.name.clone(),
                desc.content_name.clone(),
                desc.begin.unwrap().clone(),
                begin_rule_factory,
                desc.end.unwrap().clone(),
                end_rule_factory,
                desc.apply_end_pattern_last,
                pattern_factory.patterns,
            );

            helper.register_rule(Box::new(begin_end_rule));
            return id.clone();
        }

        desc.id.unwrap()
    }
}
