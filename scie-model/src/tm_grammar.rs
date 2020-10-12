use std::collections::{BTreeMap};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IEmbeddedLanguagesMap {
    #[serde(flatten)]
    pub map: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TokenTypesContribution {
    #[serde(flatten)]
    pub map: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TMGrammar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(alias = "scopeName")]
    pub scope_name: String,
    pub path: String,

    #[serde(alias = "injectTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_to: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "embeddedLanguages")]
    pub embedded_languages: Option<BTreeMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "tokenTypes")]
    pub token_types: Option<BTreeMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use crate::tm_grammar::TMGrammar;

    #[test]
    fn should_serialise_language_map() {
        let code = "{
        \"scopeName\": \"text.html.basic\",
        \"path\": \"./syntaxes/html.tmLanguage.json\",
        \"embeddedLanguages\": {
          \"text.html\": \"html\",
          \"source.css\": \"css\",
          \"source.js\": \"javascript\",
          \"source.python\": \"python\",
          \"source.smarty\": \"smarty\"
        },
        \"tokenTypes\": {
          \"meta.tag string.quoted\": \"other\"
        }
      }";

        let grammar: TMGrammar = serde_json::from_str(&code).unwrap();
        assert_eq!("html", grammar.embedded_languages.unwrap().map["text.html"]);
    }
}
