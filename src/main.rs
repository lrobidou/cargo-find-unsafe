use clap::Parser;
use proc_macro2::LineColumn;
use quote::ToTokens;
use std::fs;
use syn::{Expr, ItemFn, spanned::Spanned, visit::Visit};
use walkdir::WalkDir;

mod cli;
mod pretty_print;

use cli::Cli;

struct UnsafeFinder<'a> {
    current_fn: Option<&'a syn::ItemFn>,
    path: &'a std::path::Path,
}

impl<'a, 'ast: 'a> Visit<'ast> for UnsafeFinder<'a> {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let prev = self.current_fn;
        self.current_fn = Some(i);
        syn::visit::visit_item_fn(self, i);
        self.current_fn = prev;
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let Expr::Unsafe(uns) = expr {
            if let Some(func) = self.current_fn {
                // let name = func.sig.ident.to_string();
                let sig = func.sig.to_token_stream().to_string();
                println!("\x1b[34m{}\x1b[0m", sig);
                // let sig_str = func.sig.to_token_stream().to_string();
                let start: LineColumn = func.span().start();

                println!(
                    "--> {}:{}:{}\n",
                    self.path.display(),
                    start.line,
                    start.column
                );
            }

            let code = uns.to_token_stream().to_string();
            let pretty = pretty_print::pretty_print(&code);
            println!("{}\n---\n", pretty);
        }
    }
}

fn main() {
    let Cli::FindUnsafe(args) = Cli::parse();

    // gte the passed dir or use the current one as the default
    let path = match args.path {
        Some(root) => root,
        None => env::current_dir().expect("cannot access current directory"),
    };

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            let path = entry.path();
            let content = fs::read_to_string(path).unwrap();
            let syntax = syn::parse_file(&content).unwrap();
            let mut finder = UnsafeFinder {
                current_fn: None,
                path,
            };
            finder.visit_file(&syntax);
        }
    }
}
