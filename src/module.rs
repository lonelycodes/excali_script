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
    pub source: String,
}

impl FileNode {
    fn new(name: String, source: String) -> Self {
        Self {
            name,
            source,
        }
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

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

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

    for item in &module.body {
        if let swc_ecma_ast::ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::Import(import)) = item
        {
            import_statements.push(import.clone());
        }
    }

    for import in import_statements {
        let source = import.src.value.to_string();
        let name = source.split('/').last().unwrap().to_string();
        let node = FileNode::new(name, source);
        dependencies.push(node);
    }

    dependencies
}
