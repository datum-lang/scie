use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IEmbeddedLanguagesMap {
    #[serde(flatten)]
    pub map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenTypesContribution {
    #[serde(flatten)]
    pub map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TMGrammar {
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(alias = "scopeName")]
    scope_name: String,
    path: String,

    #[serde(alias = "injectTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    inject_to: Option<Vec<String>>,

    #[serde(alias = "embeddedLanguages")]
    embedded_languages: IEmbeddedLanguagesMap,

    #[serde(alias = "tokenTypes")]
    token_types: TokenTypesContribution,
}


#[cfg(test)]
mod tests {
    use crate::model::tm_grammar::TMGrammar;

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
        assert_eq!("html", grammar.embedded_languages.map["text.html"]);
    }
}
