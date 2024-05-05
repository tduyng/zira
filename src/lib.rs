pub mod ast;
pub mod hir;
pub mod indexer;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod sema;

fn error(loc: lexer::Loc, msg: String) -> ! {
	panic!("{loc}: error: {msg}\n");
}

#[cfg(test)]
mod testing;
