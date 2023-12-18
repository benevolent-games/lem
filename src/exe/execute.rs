
use crate::types::*;
use std::{
	rc::Rc,
	cell::RefCell,
	collections::HashMap,
};

struct Environment {
	parent: Option<Rc<RefCell<Environment>>>,
	members: HashMap<String, Expression>,
}

impl Environment {
	fn new(parent: Option<Rc<RefCell<Environment>>>) -> Environment {
		Environment {
			parent,
			members: HashMap::new(),
		}
	}

	fn new_child(&self) -> Environment {
		Environment::new(Some(Rc::clone(&self.parent.as_ref().unwrap())))
	}

	fn lookup(&self, name: &str) -> Result<Expression, String> {
		match self.members.get(name) {
			Some(value) => Ok(value.clone()),
			None => {
				match &self.parent {
					Some(parent_env) => parent_env.borrow().lookup(name),
					None => Err(format!("Variable '{}' not found", name))
				}
			}
		}
	}

	fn eval(&mut self, expression: &Expression) -> Result<Expression, String> {
		match expression {
			Expression::Number(n) => Ok(Expression::Number(*n)),
			Expression::Symbol(s) => self.lookup(s),
			Expression::List(_) => {
				// match list.get(0) {
				// 	Some(_) => Ok(Expression::Number(0)),
				// 	None => Ok(Expression::Number(0)),
				// };
				Ok(Expression::Number(0))
			},
			_ => Ok(Expression::Number(0)),
		}
	}
}

pub fn execute(_: Vec<Expression>) -> Result<(), String> {
	// let root_environment = Rc::new(RefCell::new(Environment::new(None)));

	// for expression in syntax {
	// 	let result = root_environment.borrow_mut().eval(&expression)?;
	// }

	Ok(())
}

