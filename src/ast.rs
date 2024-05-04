use crate::lexer::Loc;

#[derive(Clone, PartialEq, Eq)]
pub struct Ast {
	pub definitions: Vec<Definition>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Definition {
	Procedure(Procedure),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Procedure {
	pub name: String,
	pub parameters: Vec<Parameter>,
	pub return_ty: Option<Ty>,
	pub body: Statement,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Parameter {
	pub name: String,
	pub ty: Ty,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Statement {
	pub kind: StatementKind,
	pub loc: Loc,
}

#[derive(Clone, PartialEq, Eq)]
pub enum StatementKind {
	Expression(Expression),
	Block(Vec<Statement>),
	LocalDeclaration { name: String, ty: Ty },
	LocalDefinition { name: String, value: Expression },
	Assignment { lhs: Expression, rhs: Expression },
	Return { value: Option<Expression> },
}

#[derive(Clone, PartialEq, Eq)]
pub struct Expression {
	pub kind: ExpressionKind,
	pub loc: Loc,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ExpressionKind {
	Integer(u64),
	Variable(String),
	True,
	False,
	Binary { lhs: Box<Expression>, operator: BinaryOperator, rhs: Box<Expression> },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulo,
	ShiftLeft,
	ShiftRight,
	BitAnd,
	BitOr,
	BitXor,
	And,
	Or,
	Equal,
	NotEqual,
	Less,
	Greater,
	LessEqual,
	GreaterEqual,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Ty {
	Int,
}

impl Ast {
	pub fn pretty_print(&self) -> String {
		let mut ctx = PrettyPrintCtx { buf: String::new(), indentation: 0 };
		ctx.print_ast(self);
		ctx.buf
	}
}

struct PrettyPrintCtx {
	buf: String,
	indentation: usize,
}

impl PrettyPrintCtx {
	fn print_ast(&mut self, ast: &Ast) {
		for definition in &ast.definitions {
			match definition {
				Definition::Procedure(proc) => self.print_procedure(proc),
			}
		}
	}

	fn print_procedure(&mut self, proc: &Procedure) {
		self.s("proc ");
		self.s(&proc.name);
		self.s("(");

		for (i, parameter) in proc.parameters.iter().enumerate() {
			if i != 0 {
				self.s(", ");
			}

			self.s(&parameter.name);
			self.s(" ");
			self.print_ty(&parameter.ty);
		}

		self.s(")");

		if let Some(return_ty) = &proc.return_ty {
			self.s(" ");
			self.print_ty(return_ty);
			self.s(" ");
		}

		if proc.body.kind == StatementKind::Block(Vec::new()) {
			if proc.return_ty.is_none() {
				self.s(" ");
			}
			self.s("{}");
		} else {
			self.newline();
			self.print_statement(&proc.body);
		}

		self.newline()
	}

	fn print_statement(&mut self, statement: &Statement) {
		match &statement.kind {
			StatementKind::LocalDeclaration { name, ty } => {
				self.s("var ");
				self.s(name);
				self.s(" ");
				self.print_ty(ty);
			}
			StatementKind::LocalDefinition { name, value } => {
				self.s(name);
				self.s(" := ");
				self.print_expression(value);
			}
			StatementKind::Expression(e) => self.print_expression(e),
			StatementKind::Return { value } => {
				self.s("return");

				if let Some(value) = value {
					self.s(" ");
					self.print_expression(value);
				}
			}

			StatementKind::Block(statements) => {
				self.s("{");
				self.indentation += 1;

				for statement in statements {
					self.newline();
					self.print_statement(statement);
				}

				self.indentation -= 1;
				self.newline();
				self.s("}");
			}
			StatementKind::Assignment { lhs, rhs } => {
				self.print_expression(lhs);
				self.s(" = ");
				self.print_expression(rhs);
			}
		}
	}

	fn print_expression(&mut self, expression: &Expression) {
		match &expression.kind {
			ExpressionKind::Integer(i) => self.s(&format!("{i}")),
			ExpressionKind::Variable(name) => self.s(name),
			ExpressionKind::True => self.s("true"),
			ExpressionKind::False => self.s("false"),
			ExpressionKind::Binary { lhs, operator, rhs } => {
				self.s("(");
				self.print_expression(lhs);
				self.s(" ");

				let op = match operator {
					BinaryOperator::Add => "+",
					BinaryOperator::Subtract => "-",
					BinaryOperator::Multiply => "*",
					BinaryOperator::Divide => "/",
					BinaryOperator::Modulo => "%",
					BinaryOperator::ShiftLeft => "<<",
					BinaryOperator::ShiftRight => ">>",
					BinaryOperator::BitAnd => "&",
					BinaryOperator::BitOr => "|",
					BinaryOperator::BitXor => "^",
					BinaryOperator::And => "&&",
					BinaryOperator::Or => "||",
					BinaryOperator::Equal => "==",
					BinaryOperator::NotEqual => "!=",
					BinaryOperator::Less => "<",
					BinaryOperator::Greater => ">",
					BinaryOperator::LessEqual => "<=",
					BinaryOperator::GreaterEqual => ">=",
				};
				self.s(op);

				self.s(" ");
				self.print_expression(rhs);
				self.s(")");
			}
		}
	}

	fn print_ty(&mut self, ty: &Ty) {
		match ty {
			Ty::Int => self.s("int"),
		}
	}

	fn s(&mut self, s: &str) {
		self.buf.push_str(s);
	}

	fn newline(&mut self) {
		self.buf.push('\n');
		for _ in 0..self.indentation {
			self.buf.push('\t');
		}
	}
}