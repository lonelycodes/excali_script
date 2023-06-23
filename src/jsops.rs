// #[macro_use]
use std::path::Path;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNode {
    pub name: String,
    pub source: String, // TODO C -- get the full path
}

impl FileNode {
    fn new(name: String, source: String) -> Self {
        Self { name, source }
    }
}

pub fn parse(file_name: &str) -> swc_ecma_ast::Module {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let path = Path::new(file_name);

    let fm = cm.load_file(path).expect("failed to load file");
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    parser.take_errors().into_iter().for_each(|e| {
        e.into_diagnostic(&handler).emit();
    });

    parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parse module")
}

pub fn get_dependencies(module: &swc_ecma_ast::Module) -> Vec<FileNode> {
    module_path!();
    let mut import_statements = Vec::new();
    let mut dependencies = Vec::new();

    module.body.iter().for_each(|item| {
        if let swc_ecma_ast::ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::Import(import)) = item
        {
            import_statements.push(import.clone());
        }
    });


    import_statements.clone().into_iter().for_each(|import| {
        dbg!(import.src);
    });


    import_statements.into_iter().for_each(|import| {
        dependencies.push(FileNode::new(
            import
                .src
                .value
                .to_string()
                .split('/')
                .last()
                .unwrap()
                .to_string(),
            import.src.value.to_string(),
        ));
    });

    dependencies
}
