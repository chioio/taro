use swc_core::ecma::{
    parser,
    visit::{as_folder, Fold},
};
use crate::{
    PluginConfig,
    transform::*,
};

mod init;

pub fn tr () -> impl Fold {
    let config = serde_json::from_str::<PluginConfig>(
        r#"
        {
        }"#
    )
    .unwrap();
    let visitor = TransformVisitor::new(config);
    as_folder(visitor)
}

pub fn get_syntax_config () -> parser::Syntax {
    parser::Syntax::Es(parser::EsConfig {
        jsx: true,
        ..Default::default()
    })
}
