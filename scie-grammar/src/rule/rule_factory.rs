use crate::grammar::Grammar;
use crate::inter::{ILocation, IRawCaptures, IRawRepository, IRawRule};
use crate::rule::{
    AbstractRule, BeginEndRule, BeginWhileRule, CaptureRule, IRuleRegistry, IncludeOnlyRule,
    MatchRule, Rule,
};

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

            r.resize(
                (maximum_capture_id + 1) as usize,
                Box::new(CaptureRule::empty()),
            );

            for (id_str, desc) in capts.clone().map.capture_map {
                // todo: figure captureId === '$vscodeTextmateLocation'
                let numeric_capture_id: usize = id_str.parse().unwrap_or(0);
                let mut retokenize_captured_with_rule_id = 0;
                let options_patterns = capts.map.capture_map.get(&*numeric_capture_id.to_string());

                if let Some(rule) = options_patterns {
                    if let Some(patterns) = rule.clone().patterns {
                        retokenize_captured_with_rule_id = RuleFactory::get_compiled_rule_id(
                            desc.clone(),
                            helper,
                            repository,
                            String::from(""),
                        );
                    }
                }
                r[numeric_capture_id] = RuleFactory::create_capture_rule(
                    helper,
                    desc.clone().location,
                    desc.clone().name,
                    desc.clone().content_name,
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
        retokenizeCapturedWithRuleId: i32,
    ) -> Box<dyn AbstractRule> {
        let id = helper.register_id();
        let rule = CaptureRule::new(
            location,
            id,
            name,
            content_name,
            retokenizeCapturedWithRuleId,
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

        if let Some(patterns) = origin_patterns.clone() {
            for pattern in patterns {
                let mut pattern_id = -1;
                if let Some(include_s) = pattern.clone().include {
                    if include_s.starts_with("#") {
                        let first = remove_first(include_s.as_str());
                        let mut local_included_rule = repository.map.name_map.get_mut(first);
                        if let Some(mut rule) = local_included_rule.cloned() {
                            pattern_id = RuleFactory::get_compiled_rule_id(
                                *rule,
                                helper,
                                repository,
                                String::from(first),
                            );
                        } else {
                            println!(
                                "CANNOT find rule for scopeName: {:?}",
                                pattern.clone().include
                            );
                        }
                    } else if include_s == "$base" || include_s == "$self" {
                        let local_included_rule =
                            repository.map.name_map.get_mut(include_s.as_str());
                        if let Some(mut rule) = local_included_rule.cloned() {
                            pattern_id = RuleFactory::get_compiled_rule_id(
                                *rule,
                                helper,
                                repository,
                                String::from(include_s.as_str()),
                            );
                        }
                    } else {
                        // todo: find the cases
                        println!("todo: external grammar {:?}", pattern.include);
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
                    pattern_id = RuleFactory::get_compiled_rule_id(
                        pattern,
                        helper,
                        repository,
                        String::from(""),
                    );
                }

                if pattern_id != -1 {
                    let rule = helper.get_rule(pattern_id);
                    let mut skip_rule = false;
                    if rule.type_of() == "IncludeOnlyRule"
                        || rule.type_of() == "BeginEndRule"
                        || rule.type_of() == "BeginWhileRule"
                    {
                        if rule.has_missing_pattern() {
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
        match origin_patterns.clone() {
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
        desc_name: String,
    ) -> i32 {
        if let None = desc.id {
            let id = helper.register_id();
            desc.id = Some(id.clone());

            // since we fork logic from vscode-textmate, vscode-textmate will had duplicate some
            // rules. it will cause stackoverflow in our version, so I decide change repository id
            // by name.
            if desc_name != "" {
                if let Some(a) = repository.map.name_map.get(desc_name.as_str()).clone() {
                    repository
                        .map
                        .name_map
                        .get_mut(desc_name.as_str())
                        .unwrap()
                        .id = Some(id);
                }
            }

            if let Some(match_s) = desc.match_s {
                let rule_factory = RuleFactory::compile_captures(desc.captures, helper, repository);
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
                        patterns = Some(vec![raw_rule.clone()])
                    }
                }

                let rule_factory = RuleFactory::compile_patterns(patterns, helper, repository);
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
                let pattern_factory =
                    RuleFactory::compile_patterns(desc.patterns, helper, repository);

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
            let end_rule_factory = RuleFactory::compile_captures(end_captures, helper, repository);
            let pattern_factory = RuleFactory::compile_patterns(desc.patterns, helper, repository);

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
                pattern_factory,
            );

            helper.register_rule(Box::new(begin_end_rule));
            return id.clone();
        }

        desc.id.unwrap()
    }
}
