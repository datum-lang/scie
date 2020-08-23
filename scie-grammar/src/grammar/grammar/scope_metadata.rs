pub enum TemporaryStandardTokenType {
    Other,
    Comment,
    String,
    RegEx,
    MetaEmbedded
}

pub struct ScopeMetadata {
    pub scope_name: i32,
    pub language_id: i32,
    pub token_type: TemporaryStandardTokenType,
}

pub struct ScopeMetadataProvider {

}

