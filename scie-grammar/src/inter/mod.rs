use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct ILocation {
    pub filename: String,
    pub line: String,
    pub chart: String,
}

pub struct ILocatable {
    pub textmate_location: Option<ILocation>,
}

pub struct IRawCapturesMap {
    capture_map: HashMap<String, IRawRule>
}

pub struct IRawRepositoryMap {
    name_map: HashMap<String, IRawRule>,
    self_s: IRawRule,
    base_s: IRawRule,
}

pub struct IRawRepository {
    pub map: Box<IRawRepositoryMap>,
    pub location: ILocatable,
}

pub struct IRawCaptures {
    pub map: IRawCapturesMap,
    pub location: ILocatable,
}

pub struct IRawRule {
    pub id: Option<i32>,

    pub location: ILocation,
    pub include: Option<String>,
    pub content_name: Option<String>,
    pub match_s: Option<String>,
    pub captures: Option<Box<IRawCaptures>>,

    pub begin: Option<String>,
    pub beginCaptures: Option<Box<IRawCaptures>>,
    pub end: Option<String>,
    pub endCaptures: Option<Box<IRawCaptures>>,

    pub while_s: Option<String>,
    pub whileCaptures: Option<Box<IRawCaptures>>,

    pub pattern: Option<Vec<IRawRule>>,
    pub repository: Option<IRawRepository>,
    pub applyEndPatternLast: Option<bool>,
}

pub struct InjectionMap {
    // todo: readonly injections?: { [expression: string]: IRawRule };
    map: HashMap<String, IRawRule>
}

pub struct IRawGrammar {
    pub location: ILocatable,

    pub scope_name: String,
    pub patterns: Vec<IRawRule>,
    pub injections: Option<InjectionMap>,
    pub injectionSelector: Option<String>,
    pub fileTypes: Option<Vec<String>>,
    pub name: Option<String>,
    pub firstLineMatch: Option<String>,
}
