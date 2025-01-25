from abc import ABC, abstractmethod
from enum import enum

class NodeType(enum):
    Program = "Program"
    ExpressionStatement = "ExpressionStatement"
    InfixStatement = "InfixStatement"
    IntegerLiteral = "IntegerLiteral"
    FloatLiteral = "FloatLiteral"

class Node(ABC):
    @abstractmethod
    def get_type(self) -> NodeType:
        pass 

    @abstractmethod
    def json(self) -> dict:
        pass

class Statement(Node):
    pass

class Expression(Node):
    pass

class Program(Node):
    def __init__(self) -> None:
        self.statements: list[Statement] = []

    def get_type(self) -> NodeType:
        return NodeType.Program

    def json(self) -> dict:
        return {
            "type": self.get_type().value,
            "statements": [{stmt.type().value: stmt.json()} for stmt in self.statements],
        }

class ExpressionStatement(Statement):
    def __init__(self, expr: Expression = None) -> None:
        self.expr: Expression = expr

    def get_type(self) -> NodeType:
        return NodeType.ExpressionStatement

    def json(self) -> dict:
        return {
            "type": self.get_type().value,
            "expression": self.expr.json(),
        }

class InfixExpression(Expression):
    def __init__(self, left_node: Expression, operator: str, right_node: Expression = None) -> None:
        self.left_node: Expression = left_node
        self.operator: str = operator
        self.right_node: Expression = right_node

    def get_type(self):
        return NodeType.InfixStatement

    def json(self):
        return {
            "type": self.get_type().value,
            "left_node": self.left_node.json(),
            "operator": self.operator,
            "right_node": self.right_node.json(),
        }

class IntegerLiteral(Expression):
    def __init__(self, value: int) -> None:
        self.value: int = value 

    def get_type(self) -> NodeType:
        return NodeType.IntegerLiteral

    def json(self) -> dict:
        return {
            "type": self.get_type().value,
            "value": self.value,
        }

class FloatLiteral(Expression):
    def __init__(self, value: float) -> None:
        self.value: float = value 

    def get_type(self) -> NodeType:
        return NodeType.IntegerLiteral

    def json(self) -> dict:
        return {
            "type": self.get_type().value,
            "value": self.value,
        }