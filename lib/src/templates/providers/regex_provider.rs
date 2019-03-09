use crate::pipeline::PipelineState;
use crate::state::KatalystState;
use crate::templates::{KatalystTemplatePlaceholder, KatalystTemplateProvider};
use std::str::FromStr;

pub struct RegexTemplateProvider {}

impl KatalystTemplateProvider for RegexTemplateProvider {
    fn identifier(&self) -> &'static str {
        "regex"
    }

    fn build_placeholder(&self, value: String) -> Box<KatalystTemplatePlaceholder> {
        Box::new(RegexTemplatePlaceholder { val: value })
    }
}

#[derive(Debug)]
struct RegexTemplatePlaceholder {
    val: String,
}

impl KatalystTemplatePlaceholder for RegexTemplatePlaceholder {
    fn get_value(&self, state: &PipelineState, _config: &KatalystState) -> String {
        match &state.context.captures {
            Some(caps) => {
                let res = caps.get(&self.val).unwrap_or_else(|| self.none());
                String::from_str(res).unwrap().to_string()
            }
            None => self.none().to_string(),
        }
    }

    fn duplicate(&self) -> Box<KatalystTemplatePlaceholder> {
        RegexTemplatePlaceholder {
            val: self.val.to_owned(),
        }
        .boxed()
    }
}