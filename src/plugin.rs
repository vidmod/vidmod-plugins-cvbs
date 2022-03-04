use vidmod_node::Node;
use vidmod_plugin::Plugin;

pub const SYNC_EXTRACTOR: Plugin = Plugin {
    make_node: |params| {
        Node::Intermediate(Box::new(crate::syncextractor::SyncExtractor::new(params)))
    },
};
