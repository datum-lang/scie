pub enum MatchPriority {
    High,
    Medium,
    Low,
}

pub struct MatcherWithPriority {
    matcher: Matcher,
    priority: MatchPriority,
}

pub struct Matcher {}
