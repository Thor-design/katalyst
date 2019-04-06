use regex::Regex;
use std::collections::HashMap;
use crate::expression::*;

const METHOD: &str = r"\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*";
const TEMPLATE: &str = r"\{\{\s*([^}(=>)\s]+)\s*(?:=>)\s*([^}\s]*)\s*}}";

lazy_static! {
    static ref TEMPLATE_MATCHER: Regex = Regex::new(TEMPLATE).unwrap();
    static ref METHOD_MATCHER: Regex = Regex::new(METHOD).unwrap();
}

pub struct Providers {
    providers: HashMap<&'static str, Box<KatalystTemplateProvider>>,
}

impl Providers {
    pub fn get_from_template(&self, placeholder_text: String) -> Box<KatalystTemplatePlaceholder> {
        match TEMPLATE_MATCHER.captures(&placeholder_text) {
            Some(cap) => {
                let key = &cap[1];
                let val = &cap[2];
                match self.providers.get(key) {
                    Some(p) => p.build_placeholder(val.to_string()),
                    None => Box::new(placeholder_text),
                }
            }
            None => Box::new(placeholder_text),
        }
    }

    pub fn get_from_method(&self, placeholder_text: String) -> Box<KatalystTemplatePlaceholder> {
        match METHOD_MATCHER.captures(&placeholder_text) {
            Some(cap) => {
                let key = &cap[1];
                let val = &cap[2];
                match self.providers.get(key) {
                    Some(p) => p.build_placeholder(val.to_string()),
                    None => Box::new(placeholder_text),
                }
            }
            None => Box::new(placeholder_text),
        }
    }

    pub fn register(&mut self, provider: Box<KatalystTemplateProvider>) {
        self.providers.insert(provider.identifier(), provider);
    }

    pub fn empty() -> Self {
        Providers {
            providers: HashMap::new(),
        }
    }

    pub fn process_template_map(
        &self,
        template: &Option<HashMap<String, String>>,
    ) -> Option<HashMap<String, StringTemplate>> {
        match template {
            Some(m) => Some(
                m.iter()
                    .map(|(k, v)| (k.to_string(), self.process_template(&v)))
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn process_template_option(&self, template: &Option<String>) -> Option<StringTemplate> {
        match template {
            Some(s) => Some(self.process_template(&s)),
            None => None,
        }
    }

    pub fn process_template(&self, template: &str) -> StringTemplate {
        let mut result_placeholders: StringTemplate = vec![];
        if TEMPLATE_MATCHER.is_match(template) {
            let mut last_segment_index = 0;
            for cap in TEMPLATE_MATCHER.find_iter(template) {
                if cap.start() > last_segment_index {
                    let offset = cap.start() - last_segment_index;
                    let segment: String = template
                        .chars()
                        .skip(last_segment_index)
                        .take(offset)
                        .collect();
                    result_placeholders.push(Box::new(segment));
                }
                result_placeholders.push(self.get_from_template(cap.as_str().to_owned()));
                last_segment_index = cap.end();
            }
            if last_segment_index < template.len() {
                let offset = template.len() - last_segment_index;
                let segment: String = template
                    .chars()
                    .skip(last_segment_index)
                    .take(offset)
                    .collect();
                result_placeholders.push(Box::new(segment));
            }
        } else {
            result_placeholders.push(Box::new(template.to_owned()));
        }
        result_placeholders
    }
}

impl Default for Providers {
    fn default() -> Self {
        let mut providers = Providers::empty();
        providers.register(Box::new(EnvTemplateProvider {}));
        providers.register(Box::new(RegexTemplateProvider {}));
        providers.register(Box::new(HeaderTemplateProvider {}));
        providers.register(Box::new(HttpTemplateProvider {}));
        providers.register(Box::new(ClaimTemplateProvider {}));
        providers
    }
}