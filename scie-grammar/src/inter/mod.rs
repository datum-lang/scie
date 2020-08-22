use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct ILocation {
    pub filename: String,
    pub line: String,
    pub chart: String,
}

impl ILocation {
    pub fn new() -> Self {
        ILocation {
            filename: "".to_string(),
            line: "".to_string(),
            chart: "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ILocatable {
    pub textmate_location: Option<ILocation>,
}

impl ILocatable {
    pub fn new() -> Self {
        ILocatable {
            textmate_location: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRawCapturesMap {
    capture_map: HashMap<String, IRawRule>
}

#[derive(Debug, Clone)]
pub struct IRawRepositoryMap {
    name_map: HashMap<String, IRawRule>,
    self_s: IRawRule,
    base_s: IRawRule,
}

impl IRawRepositoryMap {
    pub fn new() -> Self {
        IRawRepositoryMap {
            name_map: Default::default(),
            self_s: IRawRule::new(),
            base_s: IRawRule::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRawRepository {
    pub map: Box<IRawRepositoryMap>,
    pub location: ILocatable,
}

impl IRawRepository {
    pub fn new() -> Self {
        IRawRepository {
            map: Box::new(IRawRepositoryMap::new()),
            location: ILocatable::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRawCaptures {
    pub map: IRawCapturesMap,
    pub location: ILocatable,
}

#[derive(Debug, Clone)]
pub struct IRawRule {
    pub id: Option<i32>,

    pub location: ILocation,
    pub include: Option<String>,
    pub content_name: Option<String>,
    pub match_s: Option<String>,
    pub captures: Option<Box<IRawCaptures>>,

    pub begin: Option<String>,
    pub begin_captures: Option<Box<IRawCaptures>>,
    pub end: Option<String>,
    pub end_captures: Option<Box<IRawCaptures>>,

    pub while_s: Option<String>,
    pub while_captures: Option<Box<IRawCaptures>>,

    pub pattern: Option<Vec<IRawRule>>,
    pub repository: Option<IRawRepository>,
    pub apply_end_pattern_last: Option<bool>,
}

impl IRawRule {
    pub fn new() -> Self {
        IRawRule {
            id: None,
            location: ILocation::new(),
            include: None,
            content_name: None,
            match_s: None,
            captures: None,
            begin: None,
            begin_captures: None,
            end: None,
            end_captures: None,
            while_s: None,
            while_captures: None,
            pattern: None,
            repository: None,
            apply_end_pattern_last: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InjectionMap {
    // todo: readonly injections?: { [expression: string]: IRawRule };
    map: HashMap<String, IRawRule>
}

pub struct IRawGrammar {
    pub location: ILocatable,
    pub repository: IRawRepository,
    pub scope_name: String,
    pub patterns: Vec<IRawRule>,
    pub injections: Option<InjectionMap>,
    pub injection_selector: Option<String>,
    pub file_types: Option<Vec<String>>,
    pub name: Option<String>,
    pub first_line_match: Option<String>,
}

impl IRawGrammar {
    pub fn new() -> IRawGrammar {
        IRawGrammar {
            location: ILocatable { textmate_location: None },
            repository: IRawRepository::new(),
            scope_name: "".to_string(),
            patterns: vec![],
            injections: None,
            injection_selector: None,
            file_types: None,
            name: None,
            first_line_match: None,
        }
    }
}