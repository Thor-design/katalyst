pub mod builder;
pub mod parsers;

use regex::Regex;

#[derive(Clone, Debug)]
pub struct Gateway {
    pub routes: Vec<Route>,
    pub listener: Listener,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub pattern: Regex,
    pub children: Option<Vec<Route>>,
    pub message: Option<String>,
    pub downstream: Downstream,
}

#[derive(Clone, Debug)]
pub struct Listener {
    pub interface: String,
}

#[derive(Clone, Debug)]
pub struct Downstream {
    pub base_url: String,
    pub path: String,
}