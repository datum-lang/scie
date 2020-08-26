use crate::inter::{IRawCaptures, IRawRepository, IRawRule};
use crate::grammar::grammar::Grammar;
use crate::rule::{CaptureRule, IRuleRegistry, MatchRule, IncludeOnlyRule, BeginWhileRule, BeginEndRule};

#[derive(Clone, Debug)]
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
        repository: IRawRepository,
    ) -> Vec<CaptureRule> {
        let mut r = vec![];

        if let Some(capts) = captures.clone() {
            let mut maximum_capture_id = 0;
            for (id_str, value) in capts.clone().map.capture_map {
                let id: i32 = id_str.parse().unwrap_or(0);
                if id > maximum_capture_id {
                    maximum_capture_id = id
                }
            }
            for i in 0..maximum_capture_id {
                r.push(CaptureRule::new());
            }

            let cloned_capts = captures.clone().unwrap();
            for (id_str, value) in capts.clone().map.capture_map {
                let numeric_capture_id: i32 = id_str.parse().unwrap_or(0);
                let mut retokenizeCapturedWithRuleId = 0;
                // println!("{:?}", numeric_capture_id.clone().to_string());
                let options_patterns = cloned_capts
                    .map
                    .capture_map
                    .get(&*numeric_capture_id.to_string());

                let desc = captures.clone().unwrap().map.capture_map[&id_str].clone();
                if let Some(rule) = options_patterns {
                    retokenizeCapturedWithRuleId =
                        RuleFactory::get_compiled_rule_id(desc, helper, repository.clone());
                }
                // r[numericCaptureId] = self.create_capture_rule(helper, desc.location, desc.name, desc.content_name, retokenizeCapturedWithRuleId);
            }
        };

        r
    }

    pub fn compile_patterns<'a>(
        origin_patterns: Option<Vec<IRawRule>>,
        helper: &'a mut Grammar,
        repository: IRawRepository,
    ) -> ICompilePatternsResult {
        let mut r: Vec<i32> = vec![];

        if let Some(patterns) = origin_patterns {
            for pattern in patterns {
                let mut pattern_id = -1;
                if let Some(include_s) = pattern.clone().include {
                    let map = repository.clone().map.name_map.clone();

                    if include_s.starts_with("#") {
                        let first = remove_first(include_s.as_str());
                        let local_included_rule = map.get(first);
                        if let Some(rule) = local_included_rule {
                            pattern_id = RuleFactory::get_compiled_rule_id(
                                *rule.clone(),
                                helper,
                                repository.clone(),
                            );
                        } else {
                            println!(
                                "CANNOT find rule for scopeName: {:?}",
                                pattern.clone().include
                            );
                        }
                    } else if include_s == "$base" || include_s == "$self" {
                        let option = pattern.include.unwrap();
                        let local_included_rule = map.get(option.as_str());
                        if let Some(rule) = local_included_rule {
                            pattern_id = RuleFactory::get_compiled_rule_id(
                                *(rule).clone(),
                                helper,
                                repository.clone(),
                            );
                        }
                    } else {
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
                        RuleFactory::get_compiled_rule_id(pattern, helper, repository.clone());
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

    pub fn get_compiled_rule_id<'a>(
        mut desc: IRawRule,
        helper: &'a mut Grammar,
        repository: IRawRepository,
    ) -> i32 {
        if let None = desc.id {
            let id = helper.register_id();
            println!("{:?}", id.clone());
            desc.id = Some(id.clone());

            if let Some(match_s) = desc.match_s {
                let rule_factory = RuleFactory::compile_captures(
                    desc.captures.clone(),
                    helper,
                    repository.clone(),
                );
                let match_rule = MatchRule::new(
                    desc.location.clone(),
                    id.clone(),
                    desc.name.clone(),
                    match_s.clone(),
                    rule_factory,
                );

                helper.register_rule(Box::new(match_rule));
                return desc.id.unwrap();
            };

            if let None = desc.begin {
                if let Some(repo) = desc.repository {
                    //todo: mergeObjects
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
                    RuleFactory::compile_patterns(patterns.clone(), helper, repository.clone());
                let include_only_rule = IncludeOnlyRule::new(
                    desc.location.clone(),
                    id.clone(),
                    desc.name.clone(),
                    desc.content_name.clone(),
                    rule_factory,
                );
                helper.register_rule(Box::new(include_only_rule));
                return desc.id.unwrap();
            }

            if let Some(while_s) = desc.while_s {
                let begin_rule_factory =
                    RuleFactory::compile_captures(desc.begin_captures, helper, repository.clone());
                let end_rule_factory =
                    RuleFactory::compile_captures(desc.end_captures, helper, repository.clone());
                let pattern_factory = RuleFactory::compile_patterns(
                    desc.patterns.clone(),
                    helper,
                    repository.clone(),
                );

                let begin_while_rule = BeginWhileRule::new(
                    desc.location.clone(),
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
                return desc.id.unwrap();
            }

            let begin_rule_factory =
                RuleFactory::compile_captures(desc.begin_captures, helper, repository.clone());
            let end_rule_factory =
                RuleFactory::compile_captures(desc.end_captures, helper, repository.clone());
            // let pattern_factory = RuleFactory::compile_patterns(
            //     desc.patterns.clone(),
            //     helper,
            //     repository.clone(),
            // );

            // todo: register with compile patterns
            let begin_end_rule = BeginEndRule::new(
                desc.location.clone(),
                id.clone(),
                desc.name.clone(),
                desc.content_name.clone(),
                desc.begin.unwrap().clone(),
                begin_rule_factory,
                desc.end,
                end_rule_factory,
                desc.apply_end_pattern_last,
                // pattern_factory,
            );

            helper.register_rule(Box::new(begin_end_rule));
            return desc.id.unwrap();
        }

        desc.id.unwrap()
    }

    pub fn create_capture_rule() {}
}

