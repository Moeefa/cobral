use super::{expressions::Expression, Location};

#[derive(Debug, Clone)]
pub enum Statement {
  Variable {
    name: String,
    initializer: Box<Expression>,
    location: Location,
  },
  Constant {
    name: String,
    initializer: Box<Expression>,
    location: Location,
  },
  Assignment {
    target: Box<Expression>,
    index: Option<Box<Expression>>,
    value: Box<Expression>,
    location: Location,
  },
  If {
    condition: Box<Option<Expression>>,
    true_block: Vec<Statement>,
    else_if_blocks: Vec<(Box<Option<Expression>>, Vec<Statement>)>,
    else_block: Option<Vec<Statement>>,
    location: Location,
  },
  While {
    condition: Box<Expression>,
    body: Vec<Statement>,
    location: Location,
  },
  For {
    initializer: Box<Statement>,
    condition: Box<Expression>,
    update: Box<Statement>,
    body: Vec<Statement>,
    location: Location,
  },
  Switch {
    expression: Box<Expression>,
    cases: Vec<(Box<Expression>, Vec<Statement>, bool)>,
    default: Option<(Vec<Statement>, bool)>,
    location: Location,
  },
  Return {
    value: Option<Box<Expression>>,
    location: Location,
  },
  Function {
    name: String,
    params: Vec<String>,
    body: Vec<Statement>,
    location: Location,
  },
  Import(String, Location),
  Expression(Expression, Location),
}

impl Statement {
  pub fn location(&self) -> Location {
    match self {
      Statement::Expression(_, location) => location.clone(),
      Statement::Assignment { location, .. } => location.clone(),
      Statement::If { location, .. } => location.clone(),
      Statement::Return { location, .. } => location.clone(),
      Statement::For { location, .. } => location.clone(),
      Statement::Switch { location, .. } => location.clone(),
      Statement::Function { location, .. } => location.clone(),
      Statement::Import(_, location) => location.clone(),
      Statement::Variable { location, .. } => location.clone(),
      Statement::Constant { location, .. } => location.clone(),
      Statement::While { location, .. } => location.clone(),
    }
  }
}
