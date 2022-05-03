#![allow(clippy::collapsible_if)]

use std::collections::BTreeMap;

use vidmod_node::Node;

pub mod linecombiner;
pub mod syncextractor;

#[no_mangle]
pub fn plugin_name() -> String {
    "vidmod-plugins-cvbs".to_owned()
}

#[no_mangle]
pub fn register_plugin() -> Vec<(String, fn(params: BTreeMap<String, String>) -> Node)> {
    vec![
        ("LineCombiner".to_owned(), |params| {
            Node(Box::new(linecombiner::LineCombiner::new(params)))
        }),
        ("HSyncExtractor".to_owned(), |params| {
            Node(Box::new(syncextractor::HSyncExtractor::new(params)))
        }),
    ]
}
