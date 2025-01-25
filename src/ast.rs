use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum NodeType {
    Program,
    Statement,
    Expression,
    ExpressionStatement,
    InfixStatement,
    IntegerLiteral,
    FloatLiteral,
}

impl NodeType {
    pub fn value(&self) -> &'static str {
        match self {
            NodeType::Program => "Program",
            NodeType::Statement => "Statement",
            NodeType::Expression => "Expression",
            NodeType::ExpressionStatement => "ExpressionStatement",
            NodeType::InfixStatement => "InfixStatement",
            NodeType::IntegerLiteral => "IntegerLiteral",
            NodeType::FloatLiteral => "FloatLiteral",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
    ExpressionStatement(ExpressionStatement),
    InfixExpression(InfixExpression),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
}

impl Node {
    pub fn get_type(&self) -> NodeType {
        match self {
            Node::Program(_) => NodeType::Program,
            Node::Statement(_) => NodeType::Statement,
            Node::Expression(_) => NodeType::Expression,
            Node::ExpressionStatement(_) => NodeType::ExpressionStatement,
            Node::InfixExpression(_) => NodeType::InfixStatement,
            Node::IntegerLiteral(_) => NodeType::IntegerLiteral,
            Node::FloatLiteral(_) => NodeType::FloatLiteral,
        }
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        match self {
            Node::Program(program) => program.json(),
            Node::Statement(stmt) => stmt.json(),
            Node::Expression(expr) => expr.json(),
            Node::ExpressionStatement(expr_stmt) => expr_stmt.json(),
            Node::InfixExpression(infix_expr) => infix_expr.json(),
            Node::IntegerLiteral(int_lit) => int_lit.json(),
            Node::FloatLiteral(float_lit) => float_lit.json(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Program {
    pub statements: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Node) {
        self.statements.push(statement);
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "type".to_string(),
            serde_json::Value::String(NodeType::Program.value().to_string()),
        );

        let statements_json: Vec<serde_json::Value> = self
            .statements
            .iter()
            .map(|stmt| {
                let mut stmt_map = HashMap::new();
                stmt_map.insert(
                    stmt.get_type().value().to_string(),
                    serde_json::to_value(stmt.json()).unwrap(),
                );
                serde_json::Value::Object(stmt_map.into_iter().collect())
            })
            .collect();

        map.insert(
            "statements".to_string(),
            serde_json::Value::Array(statements_json),
        );
        map
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ExpressionStatement {
    pub expr: Box<Node>,
}

impl ExpressionStatement {
    pub fn new(expr: Node) -> Self {
        ExpressionStatement {
            expr: Box::new(expr),
        }
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "type".to_string(),
            serde_json::Value::String(NodeType::ExpressionStatement.value().to_string()),
        );
        map.insert(
            "expression".to_string(),
            serde_json::to_value(self.expr.json()).unwrap(),
        );
        map
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InfixExpression {
    pub left_node: Box<Node>,
    pub operator: String,
    pub right_node: Box<Node>,
}

impl InfixExpression {
    pub fn new(left_node: Node, operator: String, right_node: Node) -> Self {
        InfixExpression {
            left_node: Box::new(left_node),
            operator,
            right_node: Box::new(right_node),
        }
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "type".to_string(),
            serde_json::Value::String(NodeType::InfixStatement.value().to_string()),
        );
        map.insert(
            "left_node".to_string(),
            serde_json::to_value(self.left_node.json()).unwrap(),
        );
        map.insert(
            "operator".to_string(),
            serde_json::Value::String(self.operator.clone()),
        );
        map.insert(
            "right_node".to_string(),
            serde_json::to_value(self.right_node.json()).unwrap(),
        );
        map
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IntegerLiteral {
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(value: i64) -> Self {
        IntegerLiteral { value }
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "type".to_string(),
            serde_json::Value::String(NodeType::IntegerLiteral.value().to_string()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.value)),
        );
        map
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expression {
    InfixExpression(InfixExpression),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
}

impl Expression {
    pub fn get_type(&self) -> NodeType {
        match self {
            Expression::InfixExpression(_) => NodeType::InfixStatement,
            Expression::IntegerLiteral(_) => NodeType::IntegerLiteral,
            Expression::FloatLiteral(_) => NodeType::FloatLiteral,
        }
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        match self {
            Expression::InfixExpression(infix_expr) => infix_expr.json(),
            Expression::IntegerLiteral(int_lit) => int_lit.json(),
            Expression::FloatLiteral(float_lit) => float_lit.json(),
        }
    }
}

impl Statement {
    pub fn get_type(&self) -> NodeType {
        match self {
            Statement::ExpressionStatement(_) => NodeType::ExpressionStatement,
        }
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        match self {
            Statement::ExpressionStatement(expr_stmt) => expr_stmt.json(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FloatLiteral {
    pub value: f64,
}

impl FloatLiteral {
    pub fn new(value: f64) -> Self {
        FloatLiteral { value }
    }

    pub fn json(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "type".to_string(),
            serde_json::Value::String(NodeType::FloatLiteral.value().to_string()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(self.value).unwrap()),
        );
        map
    }
}
