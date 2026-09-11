use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::process::{self, Command};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

type Scope = HashMap<String, Value>;

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Number(f64),
    Text(String),
    Bool(bool),
    Null,
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Function(Box<FunctionValue>),
    Builtin(String),
    Class(Box<ClassValue>),
    Object(Rc<RefCell<ObjectValue>>),
}

#[derive(Clone, Debug, PartialEq)]
struct ClassValue {
    name: String,
    constructor: Option<FunctionValue>,
    methods: HashMap<String, FunctionValue>,
}

#[derive(Clone, Debug, PartialEq)]
struct ObjectValue {
    class_name: String,
    properties: HashMap<String, Value>,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{}", format_number(*value)),
            Value::Text(text) => write!(f, "{text}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::Null => write!(f, "null"),
            Value::List(items) => write!(
                f,
                "[{}]",
                items
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Map(entries) => write!(
                f,
                "{{{}}}",
                entries
                    .iter()
                    .map(|(key, value)| format!("\"{key}\": {value}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Function(_) => write!(f, "<function>"),
            Value::Builtin(name) => write!(f, "<builtin {name}>"),
            Value::Class(class) => write!(f, "<class {}>", class.name),
            Value::Object(object) => write!(f, "<object {}>", object.borrow().class_name),
        }
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        value.to_string()
    }
}

impl Value {
    fn as_bool(&self) -> bool {
        match self {
            Value::Number(value) => *value != 0.0,
            Value::Text(text) => !text.is_empty(),
            Value::Bool(value) => *value,
            Value::Null => false,
            Value::List(items) => !items.is_empty(),
            Value::Map(entries) => !entries.is_empty(),
            Value::Function(_) => true,
            Value::Builtin(_) => true,
            Value::Class(_) => true,
            Value::Object(_) => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Number(f64),
    Text(String),
    Bool(bool),
    Null,
    Var(String),
    ListLiteral(Vec<Expr>),
    MapLiteral(Vec<(String, Expr)>),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Member {
        target: Box<Expr>,
        name: String,
    },
    New {
        class_name: String,
        args: Vec<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
enum Stmt {
    Say(Expr),
    Expression(Expr),
    Assign(String, Expr),
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Repeat {
        count: Expr,
        body: Vec<Stmt>,
    },
    Each {
        variable: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    ClassDef {
        name: String,
        methods: Vec<(String, FunctionValue)>,
    },
    PropertyAssign {
        object: String,
        property: String,
        expression: Expr,
    },
    Return(Expr),
}

#[derive(Clone, Debug)]
struct Program {
    statements: Vec<Stmt>,
}

#[derive(Clone, Debug)]
struct Line {
    indent: usize,
    text: String,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(f64),
    String(String),
    Identifier(String),
    Bool(bool),
    Null,
    Symbol(String),
    Eof,
}

struct Parser {
    lines: Vec<Line>,
    index: usize,
}

struct ExprParser {
    tokens: Vec<Token>,
    index: usize,
}

impl Program {
    fn parse(source: &str) -> Result<Self, String> {
        let mut lines = Vec::new();
        for raw in source.lines() {
            let trimmed = raw.trim_end_matches('\r');
            let indent = trimmed.chars().take_while(|ch| ch == &' ').count();
            let text = strip_comment(&trimmed[indent..]).trim().to_string();
            if !text.is_empty() {
                lines.push(Line { indent, text });
            }
        }

        let mut parser = Parser { lines, index: 0 };
        let statements = parser.parse_block(0)?;
        Ok(Self { statements })
    }

    fn execute(&self, scope: &mut Scope) -> Result<(), String> {
        install_builtins(scope);
        for statement in &self.statements {
            execute_statement(statement, scope)?;
        }
        Ok(())
    }
}

fn strip_comment(source: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    for (index, character) in source.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' && quote.is_some() {
            escaped = true;
            continue;
        }
        if character == '\'' || character == '"' {
            quote = if quote == Some(character) {
                None
            } else if quote.is_none() {
                Some(character)
            } else {
                quote
            };
            continue;
        }
        if character == '#' && quote.is_none() {
            return &source[..index];
        }
    }
    source
}

impl Parser {
    fn parse_block(&mut self, base_indent: usize) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while self.index < self.lines.len() {
            let current = &self.lines[self.index];
            if current.text.is_empty() {
                self.index += 1;
                continue;
            }
            if current.text == "else" || current.text.starts_with("else ") {
                break;
            }
            if current.text == "end" {
                self.index += 1;
                break;
            }
            if current.indent < base_indent {
                break;
            }
            if current.indent > base_indent {
                return Err(format!("Unexpected indentation at line {}", self.index + 1));
            }
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        let line = self.lines[self.index].text.clone();
        self.index += 1;

        if line == "say" {
            return Err("say expects an expression".to_string());
        }

        if let Some(rest) = line.strip_prefix("say ") {
            return Ok(Stmt::Say(parse_expression(rest)?));
        }

        if let Some(rest) = line.strip_prefix("say(") {
            if !rest.ends_with(')') {
                return Err("Expected ')' after say expression".to_string());
            }
            return Ok(Stmt::Say(parse_expression(&rest[..rest.len() - 1])?));
        }

        if let Some(rest) = line.strip_prefix("if ") {
            let condition = parse_expression(rest)?;
            if self.index >= self.lines.len() {
                return Err("If statement requires a block".to_string());
            }
            let body_indent = self.lines[self.index].indent;
            if body_indent <= 0 && self.lines[self.index].text != "else" {
                return Err("If statement requires a block".to_string());
            }
            let then_body = self.parse_block(body_indent)?;
            let else_body =
                if self.index < self.lines.len() && self.lines[self.index].text == "else" {
                    self.index += 1;
                    let else_indent = if self.index < self.lines.len() {
                        self.lines[self.index].indent
                    } else {
                        0
                    };
                    if else_indent > 0 {
                        self.parse_block(else_indent)?
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                };
            return Ok(Stmt::If {
                condition,
                then_body,
                else_body,
            });
        }

        if let Some(rest) = line.strip_prefix("while ") {
            let condition = parse_expression(rest)?;
            let body_indent = if self.index < self.lines.len() {
                self.lines[self.index].indent
            } else {
                0
            };
            if body_indent == 0 {
                return Err("While statement requires a block".to_string());
            }
            let body = self.parse_block(body_indent)?;
            return Ok(Stmt::While { condition, body });
        }

        if let Some(rest) = line.strip_prefix("repeat ") {
            let count = parse_expression(rest)?;
            let body_indent = if self.index < self.lines.len() {
                self.lines[self.index].indent
            } else {
                0
            };
            if body_indent == 0 {
                return Err("Repeat statement requires a block".to_string());
            }
            let body = self.parse_block(body_indent)?;
            return Ok(Stmt::Repeat { count, body });
        }

        if let Some(rest) = line.strip_prefix("each ") {
            let in_index = rest
                .find(" in ")
                .ok_or_else(|| format!("Invalid each loop: '{line}'"))?;
            let variable = rest[..in_index].trim().to_string();
            let iterable = parse_expression(rest[in_index + 4..].trim())?;
            let body_indent = if self.index < self.lines.len() {
                self.lines[self.index].indent
            } else {
                0
            };
            if body_indent == 0 {
                return Err("Each statement requires a block".to_string());
            }
            let body = self.parse_block(body_indent)?;
            return Ok(Stmt::Each {
                variable,
                iterable,
                body,
            });
        }

        if let Some(name) = line.strip_prefix("class ") {
            let name = name.trim().to_string();
            if name.is_empty() || name.contains(char::is_whitespace) {
                return Err("Class name must be a single identifier".to_string());
            }
            let body_indent = if self.index < self.lines.len() {
                self.lines[self.index].indent
            } else {
                0
            };
            if body_indent == 0 {
                return Err("Class definition requires a body".to_string());
            }
            let body = self.parse_block(body_indent)?;
            let mut methods = Vec::new();
            for statement in body {
                match statement {
                    Stmt::FunctionDef { name, params, body } => {
                        methods.push((name, FunctionValue::new(params, body)));
                    }
                    _ => return Err("Class bodies currently support methods only".to_string()),
                }
            }
            return Ok(Stmt::ClassDef { name, methods });
        }

        if let Some(rest) = line.strip_prefix("fn ") {
            let open = rest.find('(').unwrap_or(rest.len());
            let name = rest[..open].trim().to_string();
            let params_part = &rest[open + 1..];
            let close = params_part.find(')').unwrap_or(params_part.len());
            let params = if close == 0 {
                Vec::new()
            } else {
                params_part[..close]
                    .split(',')
                    .map(|part| part.trim().to_string())
                    .filter(|part| !part.is_empty())
                    .collect()
            };
            let body_indent = if self.index < self.lines.len() {
                self.lines[self.index].indent
            } else {
                0
            };
            if body_indent == 0 {
                return Err("Function definition requires a body".to_string());
            }
            let body = self.parse_block(body_indent)?;
            return Ok(Stmt::FunctionDef { name, params, body });
        }

        if let Some(rest) = line.strip_prefix("return ") {
            return Ok(Stmt::Return(parse_expression(rest)?));
        }

        if let Some((name, value)) = split_assignment(&line) {
            if let Some((object, property)) = name.split_once('.') {
                if object.is_empty() || property.is_empty() || property.contains('.') {
                    return Err("Invalid property assignment".to_string());
                }
                return Ok(Stmt::PropertyAssign {
                    object: object.to_string(),
                    property: property.to_string(),
                    expression: parse_expression(value)?,
                });
            }
            return Ok(Stmt::Assign(name, parse_expression(value)?));
        }

        Ok(Stmt::Expression(parse_expression(&line)?))
    }
}

fn split_assignment(line: &str) -> Option<(String, &str)> {
    let eq_index = line.find('=')?;
    let name = line[..eq_index].trim();
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    Some((name.to_string(), line[eq_index + 1..].trim()))
}

#[derive(Clone, Debug, PartialEq)]
struct FunctionValue {
    params: Vec<String>,
    body: Vec<Stmt>,
}

impl FunctionValue {
    fn new(params: Vec<String>, body: Vec<Stmt>) -> Self {
        Self { params, body }
    }
}

fn parse_expression(source: &str) -> Result<Expr, String> {
    let tokens = tokenize_expression(source)?;
    let mut parser = ExprParser { tokens, index: 0 };
    let expression = parser.parse_or()?;
    if !matches!(parser.peek(), Some(Token::Eof)) {
        return Err("Unexpected trailing token in expression".to_string());
    }
    Ok(expression)
}

impl ExprParser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Some(Token::Symbol(op)) if op == "or") {
            self.index += 1;
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: "or".to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), Some(Token::Symbol(op)) if op == "and") {
            self.index += 1;
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: "and".to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_addition()?;
        while let Some(Token::Symbol(op)) = self.peek() {
            if matches!(op.as_str(), "<" | ">" | "<=" | ">=" | "==" | "!=") {
                let op = op.clone();
                self.index += 1;
                let right = self.parse_addition()?;
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication()?;
        while let Some(Token::Symbol(op)) = self.peek() {
            if matches!(op.as_str(), "+" | "-") {
                let op = op.clone();
                self.index += 1;
                let right = self.parse_multiplication()?;
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while let Some(Token::Symbol(op)) = self.peek() {
            if matches!(op.as_str(), "*" | "/") {
                let op = op.clone();
                self.index += 1;
                let right = self.parse_unary()?;
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Some(Token::Symbol(op)) if op == "not") {
            self.index += 1;
            let right = self.parse_unary()?;
            return Ok(Expr::Binary {
                left: Box::new(Expr::Bool(true)),
                op: "not".to_string(),
                right: Box::new(right),
            });
        }
        if matches!(self.peek(), Some(Token::Symbol(op)) if op == "-") {
            self.index += 1;
            let right = self.parse_unary()?;
            return Ok(Expr::Binary {
                left: Box::new(Expr::Number(0.0)),
                op: "-".to_string(),
                right: Box::new(right),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().cloned() {
            Some(Token::Number(value)) => {
                self.index += 1;
                Ok(Expr::Number(value))
            }
            Some(Token::String(value)) => {
                self.index += 1;
                Ok(Expr::Text(value))
            }
            Some(Token::Bool(value)) => {
                self.index += 1;
                Ok(Expr::Bool(value))
            }
            Some(Token::Null) => {
                self.index += 1;
                Ok(Expr::Null)
            }
            Some(Token::Identifier(name)) => {
                self.index += 1;
                if name == "new" {
                    let class_name = match self.peek().cloned() {
                        Some(Token::Identifier(class_name)) => {
                            self.index += 1;
                            class_name
                        }
                        _ => return Err("Expected class name after new".to_string()),
                    };
                    match self.peek() {
                        Some(Token::Symbol(symbol)) if symbol == "(" => self.index += 1,
                        _ => return Err("Expected '(' after class name".to_string()),
                    }
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::Symbol(symbol)) if symbol == ")") {
                        loop {
                            args.push(self.parse_or()?);
                            match self.peek() {
                                Some(Token::Symbol(symbol)) if symbol == "," => self.index += 1,
                                Some(Token::Symbol(symbol)) if symbol == ")" => break,
                                _ => {
                                    return Err(
                                        "Expected ',' or ')' in constructor call".to_string()
                                    );
                                }
                            }
                        }
                    }
                    match self.peek() {
                        Some(Token::Symbol(symbol)) if symbol == ")" => self.index += 1,
                        _ => return Err("Expected ')' after constructor arguments".to_string()),
                    }
                    return Ok(Expr::New { class_name, args });
                }
                let mut expr = Expr::Var(name);
                loop {
                    match self.peek() {
                        Some(Token::Symbol(symbol)) if symbol == "(" => {
                            self.index += 1;
                            let mut args = Vec::new();
                            if !matches!(self.peek(), Some(Token::Symbol(symbol)) if symbol == ")")
                            {
                                loop {
                                    args.push(self.parse_or()?);
                                    match self.peek() {
                                        Some(Token::Symbol(symbol)) if symbol == "," => {
                                            self.index += 1
                                        }
                                        Some(Token::Symbol(symbol)) if symbol == ")" => break,
                                        _ => {
                                            return Err(
                                                "Expected ',' or ')' in function call".to_string()
                                            );
                                        }
                                    }
                                }
                            }
                            match self.peek() {
                                Some(Token::Symbol(symbol)) if symbol == ")" => {
                                    self.index += 1;
                                    expr = Expr::Call {
                                        callee: Box::new(expr),
                                        args,
                                    };
                                }
                                _ => return Err("Expected ')'".to_string()),
                            }
                        }
                        Some(Token::Symbol(symbol)) if symbol == "[" => {
                            self.index += 1;
                            let index_expr = self.parse_or()?;
                            match self.peek() {
                                Some(Token::Symbol(symbol)) if symbol == "]" => {
                                    self.index += 1;
                                    expr = Expr::Index {
                                        target: Box::new(expr),
                                        index: Box::new(index_expr),
                                    };
                                }
                                _ => return Err("Expected ']' in index expression".to_string()),
                            }
                        }
                        Some(Token::Symbol(symbol)) if symbol == "." => {
                            self.index += 1;
                            let member = match self.peek().cloned() {
                                Some(Token::Identifier(member)) => {
                                    self.index += 1;
                                    member
                                }
                                _ => return Err("Expected member name after '.'".to_string()),
                            };
                            expr = Expr::Member {
                                target: Box::new(expr),
                                name: member,
                            };
                        }
                        _ => break,
                    }
                }
                Ok(expr)
            }
            Some(Token::Symbol(symbol)) if symbol == "[" => {
                self.index += 1;
                let mut items = Vec::new();
                if !matches!(self.peek(), Some(Token::Symbol(symbol)) if symbol == "]") {
                    loop {
                        items.push(self.parse_or()?);
                        match self.peek() {
                            Some(Token::Symbol(symbol)) if symbol == "," => self.index += 1,
                            Some(Token::Symbol(symbol)) if symbol == "]" => break,
                            _ => return Err("Expected ',' or ']' in list literal".to_string()),
                        }
                    }
                }
                match self.peek() {
                    Some(Token::Symbol(symbol)) if symbol == "]" => {
                        self.index += 1;
                        Ok(Expr::ListLiteral(items))
                    }
                    _ => Err("Expected ']'".to_string()),
                }
            }
            Some(Token::Symbol(symbol)) if symbol == "{" => {
                self.index += 1;
                let mut entries = Vec::new();
                if !matches!(self.peek(), Some(Token::Symbol(symbol)) if symbol == "}") {
                    loop {
                        let key = match self.peek().cloned() {
                            Some(Token::String(value)) => {
                                self.index += 1;
                                value
                            }
                            _ => return Err("Map keys must be strings".to_string()),
                        };
                        match self.peek() {
                            Some(Token::Symbol(symbol)) if symbol == ":" => self.index += 1,
                            _ => return Err("Expected ':' in map literal".to_string()),
                        }
                        entries.push((key, self.parse_or()?));
                        match self.peek() {
                            Some(Token::Symbol(symbol)) if symbol == "," => self.index += 1,
                            Some(Token::Symbol(symbol)) if symbol == "}" => break,
                            _ => return Err("Expected ',' or '}' in map literal".to_string()),
                        }
                    }
                }
                match self.peek() {
                    Some(Token::Symbol(symbol)) if symbol == "}" => {
                        self.index += 1;
                        Ok(Expr::MapLiteral(entries))
                    }
                    _ => Err("Expected '}'".to_string()),
                }
            }
            Some(Token::Symbol(symbol)) if symbol == "(" => {
                self.index += 1;
                let expression = self.parse_or()?;
                match self.peek() {
                    Some(Token::Symbol(symbol)) if symbol == ")" => {
                        self.index += 1;
                        Ok(expression)
                    }
                    _ => Err("Expected ')'".to_string()),
                }
            }
            Some(Token::Eof) => Err("Unexpected end of expression".to_string()),
            _ => Err("Unexpected token in expression".to_string()),
        }
    }
}

fn tokenize_expression(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let ch = chars[index];
        if ch.is_whitespace() {
            index += 1;
            continue;
        }
        if ch == '.'
            && !chars
                .get(index + 1)
                .is_some_and(|next| next.is_ascii_digit())
        {
            tokens.push(Token::Symbol(".".to_string()));
            index += 1;
            continue;
        }

        match ch {
            '"' | '\'' => {
                let quote = ch;
                let mut value = String::new();
                let mut escaped = false;
                index += 1;
                while index < chars.len() {
                    let current = chars[index];
                    if escaped {
                        match current {
                            'n' => value.push('\n'),
                            't' => value.push('\t'),
                            '"' => value.push('"'),
                            '\'' => value.push('\''),
                            '\\' => value.push('\\'),
                            _ => value.push(current),
                        }
                        escaped = false;
                    } else if current == '\\' {
                        escaped = true;
                    } else if current == quote {
                        index += 1;
                        break;
                    } else {
                        value.push(current);
                    }
                    index += 1;
                }
                if index == 0 || chars.get(index.saturating_sub(1)) != Some(&quote) {
                    return Err("Unterminated string literal".to_string());
                }
                tokens.push(Token::String(value));
            }
            '0'..='9' | '.' => {
                let start = index;
                let mut has_digit = ch.is_ascii_digit();
                index += 1;
                while index < chars.len() {
                    let current = chars[index];
                    if current.is_ascii_digit() {
                        has_digit = true;
                        index += 1;
                    } else if current == '.' && !chars[start..index].iter().any(|c| *c == '.') {
                        index += 1;
                    } else {
                        break;
                    }
                }
                let number = chars[start..index].iter().collect::<String>();
                if !has_digit || number == "." {
                    return Err(format!("Invalid number: '{number}'"));
                }
                tokens.push(Token::Number(number.parse::<f64>().unwrap_or(0.0)));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = index;
                index += 1;
                while index < chars.len() {
                    let current = chars[index];
                    if current.is_ascii_alphanumeric() || current == '_' {
                        index += 1;
                    } else {
                        break;
                    }
                }
                let identifier = chars[start..index].iter().collect::<String>();
                tokens.push(match identifier.as_str() {
                    "true" => Token::Bool(true),
                    "false" => Token::Bool(false),
                    "null" => Token::Null,
                    "and" => Token::Symbol("and".to_string()),
                    "or" => Token::Symbol("or".to_string()),
                    "not" => Token::Symbol("not".to_string()),
                    _ => Token::Identifier(identifier),
                });
            }
            _ => {
                let text = &input[index..];
                if text.starts_with("<=")
                    || text.starts_with(">=")
                    || text.starts_with("==")
                    || text.starts_with("!=")
                {
                    tokens.push(Token::Symbol(text[..2].to_string()));
                    index += 2;
                } else if matches!(
                    ch,
                    '+' | '-'
                        | '*'
                        | '/'
                        | '('
                        | ')'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '<'
                        | '>'
                        | '='
                        | ','
                        | ':'
                        | '.'
                ) {
                    tokens.push(Token::Symbol(ch.to_string()));
                    index += 1;
                } else {
                    return Err(format!("Unexpected character '{ch}' in expression"));
                }
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

fn execute_statement(statement: &Stmt, scope: &mut Scope) -> Result<(), String> {
    match statement {
        Stmt::Say(expression) => {
            let value = evaluate_expression(expression, scope)?;
            println!("{value}");
            Ok(())
        }
        Stmt::Expression(expression) => {
            evaluate_expression(expression, scope)?;
            Ok(())
        }
        Stmt::Assign(name, expression) => {
            let value = evaluate_expression(expression, scope)?;
            scope.insert(name.clone(), value);
            Ok(())
        }
        Stmt::PropertyAssign {
            object,
            property,
            expression,
        } => {
            let value = evaluate_expression(expression, scope)?;
            match scope.get_mut(object) {
                Some(Value::Object(instance)) => {
                    instance
                        .borrow_mut()
                        .properties
                        .insert(property.clone(), value);
                    Ok(())
                }
                Some(_) => Err(format!("{object} is not an object")),
                None => Err(format!("Unknown variable: {object}")),
            }
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let value = evaluate_expression(condition, scope)?;
            if value.as_bool() {
                for statement in then_body {
                    execute_statement(statement, scope)?;
                }
            } else {
                for statement in else_body {
                    execute_statement(statement, scope)?;
                }
            }
            Ok(())
        }
        Stmt::While { condition, body } => {
            while evaluate_expression(condition, scope)?.as_bool() {
                for statement in body {
                    execute_statement(statement, scope)?;
                }
            }
            Ok(())
        }
        Stmt::Repeat { count, body } => {
            let count_value = evaluate_expression(count, scope)?;
            let repeat_count = match count_value {
                Value::Number(value) => value as i64,
                _ => return Err("Repeat count must be a number".to_string()),
            };
            for _ in 0..repeat_count.max(0) {
                for statement in body {
                    execute_statement(statement, scope)?;
                }
            }
            Ok(())
        }
        Stmt::Each {
            variable,
            iterable,
            body,
        } => {
            let iterable_value = evaluate_expression(iterable, scope)?;
            let values = match iterable_value {
                Value::List(items) => items,
                Value::Text(text) => text.chars().map(|ch| Value::Text(ch.to_string())).collect(),
                Value::Map(entries) => entries
                    .into_iter()
                    .map(|(key, value)| Value::Text(format!("{key}: {value}")))
                    .collect(),
                value => {
                    return Err(format!(
                        "Each loop requires a list or iterable, got {value}"
                    ));
                }
            };
            for item in values {
                let mut loop_scope = scope.clone();
                loop_scope.insert(variable.clone(), item);
                for statement in body {
                    execute_statement(statement, &mut loop_scope)?;
                }
            }
            Ok(())
        }
        Stmt::FunctionDef { name, params, body } => {
            let function = FunctionValue::new(params.clone(), body.clone());
            scope.insert(name.clone(), Value::Function(Box::new(function)));
            Ok(())
        }
        Stmt::ClassDef { name, methods } => {
            let mut constructor = None;
            let mut class_methods = HashMap::new();
            for (method_name, method) in methods {
                if method_name == "new" {
                    constructor = Some(method.clone());
                } else {
                    class_methods.insert(method_name.clone(), method.clone());
                }
            }
            scope.insert(
                name.clone(),
                Value::Class(Box::new(ClassValue {
                    name: name.clone(),
                    constructor,
                    methods: class_methods,
                })),
            );
            Ok(())
        }
        Stmt::Return(expression) => {
            let value = evaluate_expression(expression, scope)?;
            Err(format!("RETURN:{value}"))
        }
    }
}

fn evaluate_expression(expression: &Expr, scope: &Scope) -> Result<Value, String> {
    match expression {
        Expr::Number(value) => Ok(Value::Number(*value)),
        Expr::Text(value) => interpolate_text(value, scope),
        Expr::Bool(value) => Ok(Value::Bool(*value)),
        Expr::Null => Ok(Value::Null),
        Expr::ListLiteral(items) => {
            let mut values = Vec::new();
            for item in items {
                values.push(evaluate_expression(item, scope)?);
            }
            Ok(Value::List(values))
        }
        Expr::MapLiteral(entries) => {
            let mut map = HashMap::new();
            for (key, value) in entries {
                map.insert(key.clone(), evaluate_expression(value, scope)?);
            }
            Ok(Value::Map(map))
        }
        Expr::Var(name) => scope
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Unknown variable: {name}")),
        Expr::New { class_name, args } => {
            let class = match scope.get(class_name) {
                Some(Value::Class(class)) => class.clone(),
                Some(_) => return Err(format!("{class_name} is not a class")),
                None => return Err(format!("Unknown class: {class_name}")),
            };
            let mut evaluated_args = Vec::new();
            for arg in args {
                evaluated_args.push(evaluate_expression(arg, scope)?);
            }
            let mut instance = Value::Object(Rc::new(RefCell::new(ObjectValue {
                class_name: class.name.clone(),
                properties: HashMap::new(),
            })));
            if let Some(constructor) = &class.constructor {
                let mut constructor_scope = scope.clone();
                constructor_scope.insert("this".to_string(), instance.clone());
                if evaluated_args.len() != constructor.params.len() {
                    return Err(format!(
                        "Constructor expected {} arguments, got {}",
                        constructor.params.len(),
                        evaluated_args.len()
                    ));
                }
                for (param, value) in constructor.params.iter().zip(evaluated_args) {
                    constructor_scope.insert(param.clone(), value);
                }
                for statement in &constructor.body {
                    if let Err(error) = execute_statement(statement, &mut constructor_scope) {
                        if error.starts_with("RETURN:") {
                            break;
                        }
                        return Err(error);
                    }
                }
                instance = constructor_scope
                    .remove("this")
                    .ok_or_else(|| "Constructor lost this object".to_string())?;
            }
            Ok(instance)
        }
        Expr::Index { target, index } => {
            let target_value = evaluate_expression(target, scope)?;
            let index_value = evaluate_expression(index, scope)?;
            match (&target_value, &index_value) {
                (Value::List(values), Value::Number(num)) => {
                    let idx = *num as usize;
                    values
                        .get(idx)
                        .cloned()
                        .ok_or_else(|| format!("Index out of range: {idx}"))
                }
                (Value::Text(text), Value::Number(num)) => {
                    let chars: Vec<char> = text.chars().collect();
                    let idx = *num as usize;
                    chars
                        .get(idx)
                        .map(|ch| Value::Text(ch.to_string()))
                        .ok_or_else(|| format!("Index out of range: {idx}"))
                }
                (Value::Map(map), Value::Text(key)) => map
                    .get(key)
                    .cloned()
                    .ok_or_else(|| format!("Missing key: {key}")),
                _ => Err(format!("Cannot index {target_value} with {index_value}")),
            }
        }
        Expr::Member { target, name } => {
            let target_value = evaluate_expression(target, scope)?;
            match target_value {
                Value::Object(instance) => instance
                    .borrow()
                    .properties
                    .get(name)
                    .cloned()
                    .or_else(|| {
                        scope
                            .get(&instance.borrow().class_name)
                            .and_then(|value| match value {
                                Value::Class(class) => class
                                    .methods
                                    .get(name)
                                    .cloned()
                                    .map(|method| Value::Function(Box::new(method))),
                                _ => None,
                            })
                    })
                    .ok_or_else(|| format!("Unknown member: {name}")),
                _ => Err(format!("Cannot access member {name}")),
            }
        }
        Expr::Call { callee, args } => {
            if let Expr::Member { target, name } = callee.as_ref() {
                let receiver = evaluate_expression(target, scope)?;
                let instance = match &receiver {
                    Value::Object(instance) => instance,
                    _ => return Err(format!("Cannot call member {name} on non-object")),
                };
                let method = match scope.get(&instance.borrow().class_name) {
                    Some(Value::Class(class)) => class.methods.get(name).cloned(),
                    _ => None,
                }
                .ok_or_else(|| format!("Unknown method: {name}"))?;
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(evaluate_expression(arg, scope)?);
                }
                return invoke_method(scope, &method, receiver, evaluated_args);
            }
            let callee_value = evaluate_expression(callee, scope)?;
            let mut evaluated_args = Vec::new();
            for arg in args {
                evaluated_args.push(evaluate_expression(arg, scope)?);
            }
            match callee_value {
                Value::Function(function) => {
                    let mut callable_scope = scope.clone();
                    invoke_function(&mut callable_scope, &function, evaluated_args)
                }
                Value::Builtin(name) => invoke_builtin(&name, evaluated_args),
                _ => Err(format!("Value is not callable: {callee_value}")),
            }
        }
        Expr::Binary { left, op, right } => {
            let left_value = evaluate_expression(left, scope)?;
            let right_value = evaluate_expression(right, scope)?;
            match op.as_str() {
                "+" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Text(a), Value::Text(b)) => Ok(Value::Text(format!("{a}{b}"))),
                    (Value::Text(a), Value::Number(b)) => Ok(Value::Text(format!("{a}{b}"))),
                    (Value::Number(a), Value::Text(b)) => Ok(Value::Text(format!("{a}{b}"))),
                    _ => Err(format!("Cannot add {left_value} and {right_value}")),
                },
                "-" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    _ => Err(format!("Cannot subtract {left_value} and {right_value}")),
                },
                "*" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    _ => Err(format!("Cannot multiply {left_value} and {right_value}")),
                },
                "/" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) if *b != 0.0 => Ok(Value::Number(a / b)),
                    _ => Err("Division by zero".to_string()),
                },
                "<" => Ok(Value::Bool(compare_values(
                    &left_value,
                    &right_value,
                    |a, b| a < b,
                )?)),
                ">" => Ok(Value::Bool(compare_values(
                    &left_value,
                    &right_value,
                    |a, b| a > b,
                )?)),
                "<=" => Ok(Value::Bool(compare_values(
                    &left_value,
                    &right_value,
                    |a, b| a <= b,
                )?)),
                ">=" => Ok(Value::Bool(compare_values(
                    &left_value,
                    &right_value,
                    |a, b| a >= b,
                )?)),
                "==" => Ok(Value::Bool(value_equals(&left_value, &right_value))),
                "!=" => Ok(Value::Bool(!value_equals(&left_value, &right_value))),
                "and" => Ok(Value::Bool(left_value.as_bool() && right_value.as_bool())),
                "or" => Ok(Value::Bool(left_value.as_bool() || right_value.as_bool())),
                "not" => Ok(Value::Bool(!right_value.as_bool())),
                _ => Err(format!("Unsupported operator: '{op}'")),
            }
        }
    }
}

fn invoke_function(
    scope: &mut Scope,
    function: &FunctionValue,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != function.params.len() {
        return Err(format!(
            "Function expected {} arguments, got {}",
            function.params.len(),
            args.len()
        ));
    }
    let mut local_scope = scope.clone();
    for (param, value) in function.params.iter().zip(args) {
        local_scope.insert(param.clone(), value);
    }
    for statement in &function.body {
        match statement {
            Stmt::Return(expression) => return evaluate_expression(expression, &local_scope),
            _ => {
                if let Err(err) = execute_statement(statement, &mut local_scope) {
                    if let Some(rest) = err.strip_prefix("RETURN:") {
                        return Ok(Value::Text(rest.to_string()));
                    }
                    return Err(err);
                }
            }
        }
    }
    Ok(Value::Null)
}

fn invoke_method(
    scope: &Scope,
    function: &FunctionValue,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != function.params.len() {
        return Err(format!(
            "Method expected {} arguments, got {}",
            function.params.len(),
            args.len()
        ));
    }
    let mut local_scope = scope.clone();
    local_scope.insert("this".to_string(), receiver);
    for (param, value) in function.params.iter().zip(args) {
        local_scope.insert(param.clone(), value);
    }
    for statement in &function.body {
        match statement {
            Stmt::Return(expression) => return evaluate_expression(expression, &local_scope),
            _ => {
                if let Err(error) = execute_statement(statement, &mut local_scope) {
                    if let Some(value) = error.strip_prefix("RETURN:") {
                        return Ok(Value::Text(value.to_string()));
                    }
                    return Err(error);
                }
            }
        }
    }
    Ok(Value::Null)
}

fn install_builtins(scope: &mut Scope) {
    for name in [
        "size",
        "typeOf",
        "toText",
        "toNumber",
        "isEmpty",
        "isNumber",
        "startsWith",
        "endsWith",
        "replace",
        "count",
        "indexOf",
        "any",
        "all",
        "sort",
        "unique",
        "product",
        "pow",
        "sqrt",
        "lower",
        "upper",
        "trim",
        "contains",
        "split",
        "join",
        "first",
        "last",
        "sum",
        "range",
        "push",
        "pop",
        "reverse",
        "slice",
        "clamp",
        "animate",
        "aro",
        "print",
        "echo",
        "log",
        "info",
        "abs",
        "floor",
        "ceil",
        "round",
        "min",
        "max",
        "is_empty",
        "is_function",
        "has_key",
        "has_item",
        "char_at",
        "to_chars",
        "from_chars",
        "pad_start",
        "pad_end",
        "shift",
        "unshift",
        "remove",
        "flatten",
        "sin",
        "cos",
        "tan",
        "exp",
        "random",
        "random_int",
        "keys",
        "values",
        "merge",
        "get_or_default",
        "length",
        "get",
    ] {
        scope
            .entry(name.to_string())
            .or_insert_with(|| Value::Builtin(name.to_string()));
    }
}

fn invoke_builtin(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "size" => {
            require_args(name, &args, 1)?;
            let size = match &args[0] {
                Value::Text(value) => value.chars().count(),
                Value::List(value) => value.len(),
                Value::Map(value) => value.len(),
                _ => return Err("size expects text, list, or map".to_string()),
            };
            Ok(Value::Number(size as f64))
        }
        "typeOf" => {
            require_args(name, &args, 1)?;
            let value_type = match args[0] {
                Value::Number(_) => "number",
                Value::Text(_) => "text",
                Value::Bool(_) => "bool",
                Value::Null => "null",
                Value::List(_) => "list",
                Value::Map(_) => "map",
                Value::Function(_) | Value::Builtin(_) => "function",
                Value::Class(_) => "class",
                Value::Object(_) => "object",
            };
            Ok(Value::Text(value_type.to_string()))
        }
        "toText" => {
            require_args(name, &args, 1)?;
            Ok(Value::Text(args[0].to_string()))
        }
        "toNumber" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Number(value) => Ok(Value::Number(*value)),
                Value::Text(value) => value
                    .parse::<f64>()
                    .map(Value::Number)
                    .map_err(|_| "toNumber could not parse text".to_string()),
                _ => Err("toNumber expects a number or text".to_string()),
            }
        }
        "isEmpty" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Text(value) => Ok(Value::Bool(value.is_empty())),
                Value::List(values) => Ok(Value::Bool(values.is_empty())),
                Value::Map(entries) => Ok(Value::Bool(entries.is_empty())),
                Value::Null => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            }
        }
        "isNumber" => {
            require_args(name, &args, 1)?;
            Ok(Value::Bool(matches!(args[0], Value::Number(_))))
        }
        "startsWith" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(prefix)) => {
                    Ok(Value::Bool(value.starts_with(prefix)))
                }
                _ => Err("startsWith expects text and a prefix".to_string()),
            }
        }
        "endsWith" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(suffix)) => {
                    Ok(Value::Bool(value.ends_with(suffix)))
                }
                _ => Err("endsWith expects text and a suffix".to_string()),
            }
        }
        "replace" => {
            require_args(name, &args, 3)?;
            match (&args[0], &args[1], &args[2]) {
                (Value::Text(value), Value::Text(target), Value::Text(replacement)) => {
                    Ok(Value::Text(value.replace(target, replacement)))
                }
                _ => Err("replace expects text, target, and replacement".to_string()),
            }
        }
        "count" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(target)) => {
                    let mut count = 0;
                    let mut search_start = 0;
                    while let Some(index) = value[search_start..].find(target) {
                        count += 1;
                        search_start += index + target.len();
                    }
                    Ok(Value::Number(count as f64))
                }
                _ => Err("count expects text and a substring".to_string()),
            }
        }
        "indexOf" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(target)) => Ok(Value::Number(
                    value.find(target).unwrap_or(usize::MAX) as f64,
                )),
                (Value::List(values), value) => {
                    let index = values.iter().position(|item| item == value);
                    Ok(Value::Number(index.map_or(-1.0, |idx| idx as f64)))
                }
                _ => Err("indexOf expects text or list and a value".to_string()),
            }
        }
        "any" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => Ok(Value::Bool(values.iter().any(Value::as_bool))),
                _ => Err("any expects a list".to_string()),
            }
        }
        "all" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => Ok(Value::Bool(values.iter().all(Value::as_bool))),
                _ => Err("all expects a list".to_string()),
            }
        }
        "sort" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => {
                    let mut result = values.clone();
                    result.sort_by(|left, right| match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            left.partial_cmp(right).unwrap_or(Ordering::Equal)
                        }
                        (Value::Text(left), Value::Text(right)) => left.cmp(right),
                        (Value::Bool(left), Value::Bool(right)) => left.cmp(right),
                        _ => left.to_string().cmp(&right.to_string()),
                    });
                    Ok(Value::List(result))
                }
                _ => Err("sort expects a list".to_string()),
            }
        }
        "unique" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => {
                    let mut result = Vec::new();
                    for value in values {
                        if !result.iter().any(|existing| existing == value) {
                            result.push(value.clone());
                        }
                    }
                    Ok(Value::List(result))
                }
                _ => Err("unique expects a list".to_string()),
            }
        }
        "product" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => {
                    let product = values.iter().try_fold(1.0, |acc, value| {
                        let number = number_arg(name, value)?;
                        Ok::<f64, String>(acc * number)
                    })?;
                    Ok(Value::Number(product))
                }
                _ => Err("product expects a list of numbers".to_string()),
            }
        }
        "pow" => {
            require_args(name, &args, 2)?;
            let base = number_arg(name, &args[0])?;
            let exponent = number_arg(name, &args[1])?;
            Ok(Value::Number(base.powf(exponent)))
        }
        "sqrt" => {
            require_args(name, &args, 1)?;
            let value = number_arg(name, &args[0])?;
            if value < 0.0 {
                return Err("sqrt expects a non-negative number".to_string());
            }
            Ok(Value::Number(value.sqrt()))
        }
        "lower" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Text(value) => Ok(Value::Text(value.to_lowercase())),
                _ => Err("lower expects text".to_string()),
            }
        }
        "upper" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Text(value) => Ok(Value::Text(value.to_uppercase())),
                _ => Err("upper expects text".to_string()),
            }
        }
        "trim" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Text(value) => Ok(Value::Text(value.trim().to_string())),
                _ => Err("trim expects text".to_string()),
            }
        }
        "contains" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(sub)) => Ok(Value::Bool(value.contains(sub))),
                _ => Err("contains expects text and a substring".to_string()),
            }
        }
        "split" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(separator)) => {
                    let parts = value
                        .split(separator)
                        .map(|part| Value::Text(part.to_string()))
                        .collect();
                    Ok(Value::List(parts))
                }
                _ => Err("split expects text and a separator".to_string()),
            }
        }
        "join" => {
            require_args(name, &args, 2)?;
            let separator = match &args[1] {
                Value::Text(value) => value.clone(),
                _ => return Err("join expects a list and a separator text".to_string()),
            };
            let items = match &args[0] {
                Value::List(values) => values,
                _ => return Err("join expects a list and a separator text".to_string()),
            };
            let text = items
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(&separator);
            Ok(Value::Text(text))
        }
        "first" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => values
                    .first()
                    .cloned()
                    .ok_or_else(|| "first expects a non-empty list".to_string()),
                Value::Text(value) => value
                    .chars()
                    .next()
                    .map(|ch| Value::Text(ch.to_string()))
                    .ok_or_else(|| "first expects a non-empty text value".to_string()),
                _ => Err("first expects a list or text".to_string()),
            }
        }
        "last" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => values
                    .last()
                    .cloned()
                    .ok_or_else(|| "last expects a non-empty list".to_string()),
                Value::Text(value) => value
                    .chars()
                    .next_back()
                    .map(|ch| Value::Text(ch.to_string()))
                    .ok_or_else(|| "last expects a non-empty text value".to_string()),
                _ => Err("last expects a list or text".to_string()),
            }
        }
        "sum" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => {
                    let total =
                        values
                            .iter()
                            .try_fold(0.0_f64, |acc, value| -> Result<f64, String> {
                                let number = number_arg(name, value)?;
                                Ok::<f64, String>(acc + number)
                            })?;
                    Ok(Value::Number(total))
                }
                _ => Err("sum expects a list of numbers".to_string()),
            }
        }
        "push" => {
            require_args(name, &args, 2)?;
            match &args[0] {
                Value::List(values) => {
                    let mut result = values.clone();
                    result.push(args[1].clone());
                    Ok(Value::List(result))
                }
                _ => Err("push expects a list and a value".to_string()),
            }
        }
        "pop" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(values) => values
                    .last()
                    .cloned()
                    .ok_or_else(|| "pop expects a non-empty list".to_string()),
                _ => Err("pop expects a list".to_string()),
            }
        }
        "reverse" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Text(value) => Ok(Value::Text(value.chars().rev().collect())),
                Value::List(values) => {
                    let mut result = values.clone();
                    result.reverse();
                    Ok(Value::List(result))
                }
                _ => Err("reverse expects text or a list".to_string()),
            }
        }
        "slice" => {
            require_args(name, &args, 3)?;
            let start = index_arg(name, &args[1])?;
            let end = index_arg(name, &args[2])?;
            if start > end {
                return Err("slice start cannot exceed end".to_string());
            }
            match &args[0] {
                Value::Text(value) => {
                    let chars: Vec<char> = value.chars().collect();
                    if end > chars.len() {
                        return Err("slice end is out of range".to_string());
                    }
                    Ok(Value::Text(chars[start..end].iter().collect()))
                }
                Value::List(values) => {
                    if end > values.len() {
                        return Err("slice end is out of range".to_string());
                    }
                    Ok(Value::List(values[start..end].to_vec()))
                }
                _ => Err("slice expects text or a list".to_string()),
            }
        }
        "clamp" => {
            require_args(name, &args, 3)?;
            let value = number_arg(name, &args[0])?;
            let minimum = number_arg(name, &args[1])?;
            let maximum = number_arg(name, &args[2])?;
            if minimum > maximum {
                return Err("clamp minimum cannot exceed maximum".to_string());
            }
            Ok(Value::Number(value.clamp(minimum, maximum)))
        }
        "animate" => {
            require_args(name, &args, 3)?;
            let message = match &args[0] {
                Value::Text(value) => value,
                _ => {
                    return Err(
                        "animate expects text, frame count, and delay in milliseconds".to_string(),
                    );
                }
            };
            let frames = index_arg(name, &args[1])?;
            let delay = index_arg(name, &args[2])?;
            if frames == 0 {
                return Err("animate frame count must be greater than zero".to_string());
            }
            let mut stdout = std::io::stdout();
            for frame in 1..=frames {
                write!(stdout, "\r{message} {frame}/{frames}")
                    .map_err(|error| error.to_string())?;
                stdout.flush().map_err(|error| error.to_string())?;
                if delay > 0 {
                    thread::sleep(Duration::from_millis(delay as u64));
                }
            }
            writeln!(stdout).map_err(|error| error.to_string())?;
            Ok(Value::Text(message.clone()))
        }
        "aro" | "print" | "echo" | "info" => {
            require_args(name, &args, 1)?;
            let value = args[0].clone();
            println!("{value}");
            Ok(value)
        }
        "range" => {
            require_args(name, &args, 2)?;
            let start = number_arg(name, &args[0])?;
            let end = number_arg(name, &args[1])?;
            let mut values = Vec::new();
            let step = if start <= end { 1.0 } else { -1.0 };
            let mut current = start;
            if step > 0.0 {
                while current < end {
                    values.push(Value::Number(current));
                    current += step;
                }
            } else {
                while current > end {
                    values.push(Value::Number(current));
                    current += step;
                }
            }
            Ok(Value::List(values))
        }
        "abs" | "floor" | "ceil" | "round" => {
            require_args(name, &args, 1)?;
            let value = number_arg(name, &args[0])?;
            let result = match name {
                "abs" => value.abs(),
                "floor" => value.floor(),
                "ceil" => value.ceil(),
                _ => value.round(),
            };
            Ok(Value::Number(result))
        }
        "min" | "max" => {
            if args.is_empty() {
                return Err(format!("{name} expects at least one number"));
            }
            let mut values = args.iter().map(|value| number_arg(name, value));
            let first = values.next().unwrap()?;
            let result = values.try_fold(first, |current, value| -> Result<f64, String> {
                let next = value?;
                Ok(if name == "min" {
                    current.min(next)
                } else {
                    current.max(next)
                })
            })?;
            Ok(Value::Number(result))
        }
        // PHASE 2: New standard library functions
        "is_empty" => {
            require_args(name, &args, 1)?;
            let is_empty = match &args[0] {
                Value::Text(text) => text.is_empty(),
                Value::List(list) => list.is_empty(),
                Value::Map(map) => map.is_empty(),
                Value::Null => true,
                _ => false,
            };
            Ok(Value::Bool(is_empty))
        }
        "is_function" => {
            require_args(name, &args, 1)?;
            let is_func = matches!(args[0], Value::Function(_) | Value::Builtin(_));
            Ok(Value::Bool(is_func))
        }
        "has_key" => {
            require_args(name, &args, 2)?;
            match &args[0] {
                Value::Map(map) => {
                    let key = match &args[1] {
                        Value::Text(k) => k.clone(),
                        _ => return Err("has_key expects text key".to_string()),
                    };
                    Ok(Value::Bool(map.contains_key(&key)))
                }
                _ => Err("has_key expects a map".to_string()),
            }
        }
        "has_item" => {
            require_args(name, &args, 2)?;
            match &args[0] {
                Value::List(list) => Ok(Value::Bool(list.contains(&args[1]))),
                _ => Err("has_item expects a list".to_string()),
            }
        }
        "char_at" => {
            require_args(name, &args, 2)?;
            match &args[0] {
                Value::Text(text) => {
                    let index = index_arg(name, &args[1])?;
                    let chars: Vec<char> = text.chars().collect();
                    if index >= chars.len() {
                        return Err("char_at index out of bounds".to_string());
                    }
                    Ok(Value::Text(chars[index].to_string()))
                }
                _ => Err("char_at expects text".to_string()),
            }
        }
        "to_chars" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Text(text) => {
                    let chars = text.chars().map(|c| Value::Text(c.to_string())).collect();
                    Ok(Value::List(chars))
                }
                _ => Err("to_chars expects text".to_string()),
            }
        }
        "from_chars" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(chars) => {
                    let text = chars
                        .iter()
                        .map(|v| match v {
                            Value::Text(s) => Ok(s.clone()),
                            _ => Err("from_chars expects list of text".to_string()),
                        })
                        .collect::<Result<String, String>>()?;
                    Ok(Value::Text(text))
                }
                _ => Err("from_chars expects a list".to_string()),
            }
        }
        "pad_start" => {
            require_args(name, &args, 3)?;
            match (&args[0], &args[1], &args[2]) {
                (Value::Text(text), Value::Number(length), Value::Text(pad_char)) => {
                    let len = *length as usize;
                    let pad = if pad_char.is_empty() {
                        " "
                    } else {
                        pad_char.as_str()
                    };
                    let missing = len.saturating_sub(text.chars().count());
                    let padding: String = pad.chars().cycle().take(missing).collect();
                    let result = format!("{padding}{text}");
                    Ok(Value::Text(result))
                }
                _ => Err("pad_start expects text, number, and char".to_string()),
            }
        }
        "pad_end" => {
            require_args(name, &args, 3)?;
            match (&args[0], &args[1], &args[2]) {
                (Value::Text(text), Value::Number(length), Value::Text(pad_char)) => {
                    let len = *length as usize;
                    let pad = if pad_char.is_empty() {
                        " "
                    } else {
                        pad_char.as_str()
                    };
                    let missing = len.saturating_sub(text.chars().count());
                    let padding: String = pad.chars().cycle().take(missing).collect();
                    let result = format!("{text}{padding}");
                    Ok(Value::Text(result))
                }
                _ => Err("pad_end expects text, number, and char".to_string()),
            }
        }
        "shift" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(list) => {
                    if list.is_empty() {
                        return Err("shift expects non-empty list".to_string());
                    }
                    Ok(list[0].clone())
                }
                _ => Err("shift expects a list".to_string()),
            }
        }
        "unshift" => {
            require_args(name, &args, 2)?;
            match &args[0] {
                Value::List(list) => {
                    let mut result = vec![args[1].clone()];
                    result.extend_from_slice(list);
                    Ok(Value::List(result))
                }
                _ => Err("unshift expects a list".to_string()),
            }
        }
        "remove" => {
            require_args(name, &args, 2)?;
            match &args[0] {
                Value::List(list) => {
                    let index = index_arg(name, &args[1])?;
                    if index >= list.len() {
                        return Err("remove index out of bounds".to_string());
                    }
                    let mut result = list.clone();
                    result.remove(index);
                    Ok(Value::List(result))
                }
                _ => Err("remove expects a list".to_string()),
            }
        }
        "flatten" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::List(list) => {
                    let mut result = Vec::new();
                    for item in list {
                        match item {
                            Value::List(inner) => result.extend_from_slice(inner),
                            other => result.push(other.clone()),
                        }
                    }
                    Ok(Value::List(result))
                }
                _ => Err("flatten expects a list".to_string()),
            }
        }
        "sin" => {
            require_args(name, &args, 1)?;
            let value = number_arg(name, &args[0])?;
            Ok(Value::Number(value.sin()))
        }
        "cos" => {
            require_args(name, &args, 1)?;
            let value = number_arg(name, &args[0])?;
            Ok(Value::Number(value.cos()))
        }
        "tan" => {
            require_args(name, &args, 1)?;
            let value = number_arg(name, &args[0])?;
            Ok(Value::Number(value.tan()))
        }
        "log" => {
            require_args(name, &args, 1)?;
            let value = number_arg(name, &args[0])?;
            if value <= 0.0 {
                return Err("log expects positive number".to_string());
            }
            Ok(Value::Number(value.ln()))
        }
        "exp" => {
            require_args(name, &args, 1)?;
            let value = number_arg(name, &args[0])?;
            Ok(Value::Number(value.exp()))
        }
        "random" => {
            require_args(name, &args, 0)?;
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            let seed = (nanos as f64) / 1_000_000_000.0;
            Ok(Value::Number((seed * 31337.0) % 1.0))
        }
        "random_int" => {
            require_args(name, &args, 2)?;
            let min = number_arg(name, &args[0])? as i64;
            let max = number_arg(name, &args[1])? as i64;
            if min > max {
                return Err("random_int min cannot exceed max".to_string());
            }
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            let range = (max - min + 1) as u32;
            let result = min + ((nanos % range as u32) as i64);
            Ok(Value::Number(result as f64))
        }
        "keys" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Map(map) => {
                    let keys: Vec<Value> = map.keys().map(|k| Value::Text(k.clone())).collect();
                    Ok(Value::List(keys))
                }
                _ => Err("keys expects a map".to_string()),
            }
        }
        "values" => {
            require_args(name, &args, 1)?;
            match &args[0] {
                Value::Map(map) => {
                    let values: Vec<Value> = map.values().cloned().collect();
                    Ok(Value::List(values))
                }
                _ => Err("values expects a map".to_string()),
            }
        }
        "merge" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::Map(map1), Value::Map(map2)) => {
                    let mut result = map1.clone();
                    for (key, value) in map2 {
                        result.insert(key.clone(), value.clone());
                    }
                    Ok(Value::Map(result))
                }
                _ => Err("merge expects two maps".to_string()),
            }
        }
        "get_or_default" => {
            require_args(name, &args, 3)?;
            match &args[0] {
                Value::Map(map) => {
                    let key = match &args[1] {
                        Value::Text(k) => k,
                        _ => return Err("get_or_default expects text key".to_string()),
                    };
                    let value = map.get(key).cloned().unwrap_or_else(|| args[2].clone());
                    Ok(value)
                }
                _ => Err("get_or_default expects a map".to_string()),
            }
        }
        "length" => {
            require_args(name, &args, 1)?;
            let len = match &args[0] {
                Value::Text(text) => text.chars().count(),
                Value::List(list) => list.len(),
                Value::Map(map) => map.len(),
                _ => return Err("length expects text, list, or map".to_string()),
            };
            Ok(Value::Number(len as f64))
        }
        "get" => {
            require_args(name, &args, 2)?;
            match (&args[0], &args[1]) {
                (Value::List(list), Value::Number(index)) => {
                    let idx = *index as usize;
                    list.get(idx)
                        .cloned()
                        .ok_or_else(|| "get index out of bounds".to_string())
                }
                (Value::Map(map), Value::Text(key)) => map
                    .get(key)
                    .cloned()
                    .ok_or_else(|| "get key not found".to_string()),
                _ => Err("get expects list/index or map/key".to_string()),
            }
        }
        _ => Err(format!("Unknown builtin: {name}")),
    }
}

fn require_args(name: &str, args: &[Value], count: usize) -> Result<(), String> {
    if args.len() != count {
        Err(format!(
            "{name} expects {count} argument(s), got {}",
            args.len()
        ))
    } else {
        Ok(())
    }
}

fn number_arg(name: &str, value: &Value) -> Result<f64, String> {
    match value {
        Value::Number(value) => Ok(*value),
        _ => Err(format!("{name} expects numbers")),
    }
}

fn index_arg(name: &str, value: &Value) -> Result<usize, String> {
    let number = number_arg(name, value)?;
    if number < 0.0 || number.fract() != 0.0 {
        return Err(format!("{name} expects whole-number indexes"));
    }
    Ok(number as usize)
}

fn interpolate_text(source: &str, scope: &Scope) -> Result<Value, String> {
    let mut result = String::new();
    let bytes = source.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let end = source[i + 1..]
                .find('}')
                .ok_or_else(|| format!("Unclosed interpolation in string: '{source}'"))?;
            let name = &source[i + 1..i + 1 + end];
            let value = scope
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Unknown variable: {name}"))?;
            result.push_str(&value.to_string());
            i += end + 2;
        } else {
            result.push(source.as_bytes()[i] as char);
            i += 1;
        }
    }

    Ok(Value::Text(result))
}

fn value_equals(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::Text(a), Value::Text(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Null, Value::Null) => true,
        (Value::List(a), Value::List(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b)
                    .all(|(left, right)| value_equals(left, right))
        }
        (Value::Map(a), Value::Map(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(key, value)| b.get(key).is_some_and(|other| value_equals(value, other)))
        }
        _ => false,
    }
}

fn compare_values<F>(left: &Value, right: &Value, comparator: F) -> Result<bool, String>
where
    F: Fn(f64, f64) -> bool,
{
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(comparator(*a, *b)),
        (Value::Text(a), Value::Text(b)) => {
            let order = a.partial_cmp(b).unwrap_or(Ordering::Equal);
            Ok(match order {
                Ordering::Less => comparator(-1.0, 0.0),
                Ordering::Equal => comparator(0.0, 0.0),
                Ordering::Greater => comparator(1.0, 0.0),
            })
        }
        _ => Err(format!("Cannot compare {left} and {right}")),
    }
}
/*                    let mut evaluated_args = Vec::new();
                    for arg in args {
                        evaluated_args.push(evaluate_expression(&arg, scope)?);
                    }
                    let mut callable_scope = scope.clone();
                    invoke_function(&mut callable_scope, &function, evaluated_args)
                }
                _ => Err(format!("Value is not callable: {callee_value}")),
            }
        }
        Expr::Binary { left, op, right } => {
            let left_value = evaluate_expression(left, scope)?;
            let right_value = evaluate_expression(right, scope)?;
            match op.as_str() {
                "+" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Text(a), Value::Text(b)) => Ok(Value::Text(format!("{a}{b}"))),
                    (Value::Text(a), Value::Number(b)) => Ok(Value::Text(format!("{a}{b}"))),
                    (Value::Number(a), Value::Text(b)) => Ok(Value::Text(format!("{a}{b}"))),
                    _ => Err(format!("Cannot add {left_value} and {right_value}")),
                },
                "-" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    _ => Err(format!("Cannot subtract {left_value} and {right_value}")),
                },
                "*" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    _ => Err(format!("Cannot multiply {left_value} and {right_value}")),
                },
                "/" => match (&left_value, &right_value) {
                    (Value::Number(a), Value::Number(b)) if *b != 0.0 => Ok(Value::Number(a / b)),
                    _ => Err("Division by zero".to_string()),
                },
                "<" => Ok(Value::Bool(compare_values(&left_value, &right_value, |a, b| a < b)?)),
                ">" => Ok(Value::Bool(compare_values(&left_value, &right_value, |a, b| a > b)?)),
                "<=" => Ok(Value::Bool(compare_values(&left_value, &right_value, |a, b| a <= b)?)),
                ">=" => Ok(Value::Bool(compare_values(&left_value, &right_value, |a, b| a >= b)?)),
                "==" => Ok(Value::Bool(value_equals(&left_value, &right_value))),
                "!=" => Ok(Value::Bool(!value_equals(&left_value, &right_value))),
                "or" => Ok(Value::Bool(left_value.as_bool() || right_value.as_bool())),
                _ => Err(format!("Unsupported operator: '{op}'")),
            }
        }
    }
}

fn invoke_function(scope: &mut Scope, function: &FunctionValue, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != function.params.len() {
        return Err(format!("Function expected {} arguments, got {}", function.params.len(), args.len()));
    }
    let mut local_scope = scope.clone();
    for (param, value) in function.params.iter().zip(args) {
        local_scope.insert(param.clone(), value);
    }
    for statement in &function.body {
        match statement {
            Stmt::Return(expression) => return evaluate_expression(expression, &local_scope),
            _ => {
                if let Err(err) = execute_statement(statement, &mut local_scope) {
                    if let Some(rest) = err.strip_prefix("RETURN:") {
                        return Ok(Value::Text(rest.to_string()));
                    }
                    return Err(err);
                }
            }
        }
    }
    Ok(Value::Null)
}

fn interpolate_text(source: &str, scope: &Scope) -> Result<Value, String> {
    let mut result = String::new();
    let bytes = source.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let end = source[i + 1..].find('}').ok_or_else(|| format!("Unclosed interpolation in string: '{source}'"))?;
            let name = &source[i + 1..i + 1 + end];
            let value = scope.get(name).cloned().ok_or_else(|| format!("Unknown variable: {name}"))?;
            result.push_str(&value.to_string());
            i += end + 2;
        } else {
            result.push(source.as_bytes()[i] as char);
            i += 1;
        }
    }

    Ok(Value::Text(result))
}

fn value_equals(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::Text(a), Value::Text(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}

fn compare_values<F>(left: &Value, right: &Value, comparator: F) -> Result<bool, String>
where
    F: Fn(f64, f64) -> bool,
{
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(comparator(*a, *b)),
        (Value::Text(a), Value::Text(b)) => {
            let order = a.partial_cmp(b).unwrap_or(Ordering::Equal);
            Ok(match order {
                Ordering::Less => comparator(-1.0, 0.0),
                Ordering::Equal => comparator(0.0, 0.0),
                Ordering::Greater => comparator(1.0, 0.0),
            })
        }
        _ => Err(format!("Cannot compare {left} and {right}")),
    }
}

*/
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        if let Some(stem) = env::current_exe().ok().and_then(|path| {
            path.file_stem()
                .map(|value| value.to_string_lossy().to_string())
        }) {
            if stem != "lizard" && stem != "lz" {
                let source_path = env::current_exe()
                    .ok()
                    .map(|path| path.with_extension("lz"));
                if let Some(source_path) = source_path {
                    let source = fs::read_to_string(&source_path).unwrap_or_else(|error| {
                        file_error(source_path.to_str().unwrap_or("program.lz"), error)
                    });
                    run_source(&source, source_path.to_str().unwrap_or("program.lz"));
                    return;
                }
            }
        }
        run_repl();
        return;
    }

    if args[0] == "--version" || args[0] == "-V" {
        println!(
            "LIZARD {}\nRuntime: Native\nPlatform: Windows x64",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }

    if args[0] == "--help" || args[0] == "-h" || args[0] == "help" {
        print_help();
        return;
    }

    if args[0] == "-e" {
        let source = args
            .get(1)
            .unwrap_or_else(|| usage_error("-e requires source code"));
        run_source(source, "<command line>");
        return;
    }

    if args[0] == "doctor" {
        run_doctor();
        return;
    }

    if matches!(args[0].as_str(), "update" | "upgrade") {
        run_update(args.iter().any(|arg| arg == "--check"));
        return;
    }

    if args[0] == "uninstall" {
        run_uninstall();
        return;
    }

    if args[0] == "new" {
        let name = args
            .get(1)
            .unwrap_or_else(|| usage_error("new requires a project name"));
        create_project(name);
        return;
    }

    if args[0] == "test" {
        println!("LIZARD test: use `cargo test` while developing the compiler.");
        return;
    }

    let command = match args[0].as_str() {
        "run" | "check" | "fmt" | "build" | "repl" => args[0].as_str(),
        _ if args[0].ends_with(".lz") || args.len() == 1 => "run",
        _ => usage_error(&format!("unknown command '{}'", args[0])),
    };

    if command == "repl" {
        run_repl();
        return;
    }

    let (path, write_format, output) = match command {
        "build" => {
            let input = args
                .iter()
                .skip(1)
                .find(|arg| arg.ends_with(".lz"))
                .cloned()
                .or_else(find_project_entry)
                .unwrap_or_else(|| usage_error("build requires a .lz file or lizard.json"));
            let output = args
                .windows(2)
                .find(|pair| pair[0] == "--output")
                .map(|pair| pair[1].clone());
            (input, false, output)
        }
        "run" if args.len() == 1 => (
            if args[0].ends_with(".lz") {
                args[0].clone()
            } else {
                find_project_entry()
                    .unwrap_or_else(|| usage_error("run requires a .lz file or lizard.json"))
            },
            false,
            None,
        ),
        "run" | "check" => (
            args.get(1)
                .cloned()
                .unwrap_or_else(|| usage_error("missing .lz file")),
            false,
            None,
        ),
        "fmt" => {
            let write = args.iter().any(|arg| arg == "--write");
            (
                args.iter()
                    .skip(1)
                    .find(|arg| !arg.starts_with('-'))
                    .cloned()
                    .unwrap_or_else(|| usage_error("fmt requires a .lz file")),
                write,
                None,
            )
        }
        _ => unreachable!(),
    };

    let source = fs::read_to_string(&path).unwrap_or_else(|error| file_error(&path, error));
    let program = Program::parse(&source).unwrap_or_else(|error| compile_error(&path, error));

    match command {
        "check" => println!("Checked {path}"),
        "fmt" => {
            let formatted = format_source(&source);
            if write_format {
                fs::write(&path, formatted).unwrap_or_else(|error| file_error(&path, error));
            } else {
                print!("{formatted}");
            }
        }
        "build" => build_native(&path, output.as_deref()),
        "run" => execute_program(program),
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn legacy_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: lizard <file.lz> | lizard check <file.lz> | lizard fmt [--write] <file.lz>"
        );
        process::exit(1);
    }

    if matches!(args[1].as_str(), "--version" | "-V" | "version") {
        println!("LIZARD {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let (command, file_path, write) = match args[1].as_str() {
        "check" | "fmt" => {
            let write = args[1] == "fmt" && args.get(2).is_some_and(|arg| arg == "--write");
            let file_index = if write { 3 } else { 2 };
            let file = args.get(file_index).unwrap_or_else(|| {
                eprintln!("Missing input file");
                process::exit(1);
            });
            (args[1].as_str(), file.clone(), write)
        }
        _ => ("run", args[1].clone(), false),
    };

    let file = &file_path;
    let source = fs::read_to_string(file).unwrap_or_else(|error| {
        eprintln!("Failed to read {}: {error}", file);
        process::exit(1);
    });

    let program = Program::parse(&source).unwrap_or_else(|error| {
        eprintln!("LIZARD ERROR\n\n{error}");
        process::exit(1);
    });

    if command == "check" {
        println!("Checked {file}");
        return;
    }

    if command == "fmt" {
        let formatted = format_source(&source);
        if write {
            fs::write(file, formatted).unwrap_or_else(|error| {
                eprintln!("Failed to write {}: {error}", file);
                process::exit(1);
            });
        } else {
            print!("{formatted}");
        }
        return;
    }

    let mut scope = Scope::new();
    if let Err(error) = program.execute(&mut scope) {
        eprintln!("LIZARD ERROR\n\n{error}");
        process::exit(1);
    }
}

fn format_source(source: &str) -> String {
    let mut level = 0usize;
    let mut output = String::new();

    for raw_line in source.lines() {
        let text = raw_line.trim();
        if text.is_empty() {
            if !output.ends_with("\n\n") && !output.is_empty() {
                output.push('\n');
            }
            continue;
        }

        if text == "end" || text == "else" || text.starts_with("else ") {
            level = level.saturating_sub(1);
        }

        output.push_str(&"    ".repeat(level));
        output.push_str(text);
        output.push('\n');

        if text.starts_with("if ")
            || text.starts_with("repeat ")
            || text.starts_with("each ")
            || text.starts_with("fn ")
            || text == "else"
        {
            level += 1;
        }
    }

    output
}

fn execute_program(program: Program) {
    let mut scope = Scope::new();
    if let Err(error) = program.execute(&mut scope) {
        eprintln!("LIZARD ERROR\n\n{error}");
        process::exit(1);
    }
}

fn run_source(source: &str, file: &str) {
    let program = Program::parse(source).unwrap_or_else(|error| compile_error(file, error));
    execute_program(program);
}

fn run_repl() {
    println!("LIZARD {}\nRuntime: Native", env!("CARGO_PKG_VERSION"));
    let stdin = std::io::stdin();
    let mut scope = Scope::new();

    loop {
        print!("> ");
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let source = line.trim();
        if source == "exit" || source == "quit" {
            break;
        }
        if source.is_empty() {
            continue;
        }
        if matches!(source, "--help" | "-h" | "help") {
            print_help();
            continue;
        }
        if matches!(source, "--version" | "-V" | "version") {
            println!(
                "LIZARD {}\nRuntime: Native\nPlatform: Windows x64",
                env!("CARGO_PKG_VERSION")
            );
            continue;
        }
        if source == "doctor" {
            run_doctor();
            continue;
        }

        match Program::parse(source) {
            Ok(program) => {
                if let Err(error) = program.execute(&mut scope) {
                    eprintln!("LIZARD ERROR\n\n{error}");
                }
            }
            Err(_) => match parse_expression(source) {
                Ok(expression) => match evaluate_expression(&expression, &scope) {
                    Ok(value) => println!("{value}"),
                    Err(error) => eprintln!("LIZARD ERROR\n\n{error}"),
                },
                Err(error) => eprintln!("LIZARD ERROR\n\n{error}"),
            },
        }
    }
}

fn run_update(check_only: bool) {
    let release_url = "https://github.com/surjolive/LIZARD/releases/latest";
    println!("LIZARD update check");
    println!("Current version: {}", env!("CARGO_PKG_VERSION"));
    println!("Latest release: {release_url}");

    if check_only {
        println!("Run `lz update` to install the latest release.");
        return;
    }

    let installer_url = if env::consts::OS == "windows" {
        "https://raw.githubusercontent.com/surjolive/LIZARD/master/install.ps1"
    } else {
        "https://raw.githubusercontent.com/surjolive/LIZARD/master/install.sh"
    };
    let started = if env::consts::OS == "windows" {
        let command = format!(
            "$path = Join-Path $env:TEMP 'lizard-update.ps1'; Invoke-WebRequest -UseBasicParsing -Uri '{installer_url}' -OutFile $path; Start-Sleep -Milliseconds 750; & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $path"
        );
        Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &command,
            ])
            .spawn()
            .is_ok()
    } else {
        let command = format!("sleep 1; curl -fsSL '{installer_url}' | sh");
        Command::new("sh").args(["-c", &command]).spawn().is_ok()
    };

    if started {
        println!(
            "Update started. The installer will replace the binaries after this command exits."
        );
        println!("Open a new terminal and run `lz --version` when it completes.");
    } else {
        eprintln!(
            "Could not start the updater. Download the latest release manually from {release_url}."
        );
        process::exit(4);
    }
}

fn build_native(source_path: &str, requested_output: Option<&str>) {
    let current_exe =
        env::current_exe().unwrap_or_else(|error| file_error("current executable", error));
    let source =
        fs::read_to_string(source_path).unwrap_or_else(|error| file_error(source_path, error));
    Program::parse(&source).unwrap_or_else(|error| compile_error(source_path, error));

    let output = requested_output.map(String::from).unwrap_or_else(|| {
        format!(
            "{}.exe",
            std::path::Path::new(source_path)
                .with_extension("")
                .display()
        )
    });
    let output_path = std::path::PathBuf::from(output);
    fs::copy(&current_exe, &output_path)
        .unwrap_or_else(|error| file_error(output_path.to_str().unwrap_or("output.exe"), error));

    let adjacent_source = output_path.with_extension("lz");
    if std::path::Path::new(source_path) != adjacent_source {
        fs::copy(source_path, &adjacent_source).unwrap_or_else(|error| {
            file_error(adjacent_source.to_str().unwrap_or("program.lz"), error)
        });
    }
    println!("Build successful.\nOutput: {}", output_path.display());
}

fn find_project_entry() -> Option<String> {
    let config = fs::read_to_string("lizard.json").ok()?;
    let main = config
        .split("\"main\"")
        .nth(1)?
        .split(':')
        .nth(1)?
        .split('"')
        .nth(1)?;
    Some(main.to_string())
}

fn create_project(name: &str) {
    let root = std::path::Path::new(name);
    fs::create_dir_all(root.join("src")).unwrap_or_else(|error| file_error(name, error));
    fs::create_dir_all(root.join("tests")).unwrap_or_else(|error| file_error(name, error));
    fs::write(root.join("lizard.json"), format!("{{\n  \"name\": \"{name}\",\n  \"version\": \"{}\",\n  \"main\": \"src/main.lz\"\n}}\n", env!("CARGO_PKG_VERSION"))).unwrap_or_else(|error| file_error(name, error));
    fs::write(root.join("src/main.lz"), "say \"Hello from LIZARD!\"\n")
        .unwrap_or_else(|error| file_error(name, error));
    fs::write(
        root.join("README.md"),
        format!("# {name}\n\nRun with `lizard run`.\n"),
    )
    .unwrap_or_else(|error| file_error(name, error));
    println!("Created LIZARD project {name}");
}

fn run_doctor() {
    let executable = env::current_exe().ok();
    println!("LIZARD Doctor\n");
    println!(
        "{} LIZARD executable found",
        if executable.is_some() { "✓" } else { "✗" }
    );
    println!("✓ Runtime: Native");
    println!("✓ Version: {}", env!("CARGO_PKG_VERSION"));
    println!("✓ Platform: {}", env::consts::OS);
    println!("\nLIZARD installation is healthy.");
}

fn run_uninstall() {
    let uninstaller_url = if env::consts::OS == "windows" {
        "https://raw.githubusercontent.com/surjolive/LIZARD/master/uninstall.ps1"
    } else {
        "https://raw.githubusercontent.com/surjolive/LIZARD/master/uninstall.sh"
    };
    let started = if env::consts::OS == "windows" {
        let command = format!(
            "$path = Join-Path $env:TEMP 'lizard-uninstall.ps1'; Invoke-WebRequest -UseBasicParsing -Uri '{uninstaller_url}' -OutFile $path; Start-Sleep -Milliseconds 750; & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $path"
        );
        Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &command,
            ])
            .spawn()
            .is_ok()
    } else {
        let command = format!("sleep 1; curl -fsSL '{uninstaller_url}' | sh");
        Command::new("sh").args(["-c", &command]).spawn().is_ok()
    };

    if started {
        println!("Uninstallation started. LIZARD will be removed after this command exits.");
        println!("The uninstaller will remove all LIZARD binaries and PATH entries.");
    } else {
        eprintln!(
            "Could not start the uninstaller. Please remove LIZARD manually from %LOCALAPPDATA%\\LIZARD directory."
        );
        process::exit(4);
    }
}

fn print_help() {
    println!(
        "LIZARD Programming Language\n\nUsage:\n    lz [command] [file]\n\nCommands:\n    run       Run a LIZARD program\n    build     Build a native executable\n    repl      Start the REPL\n    update    Update LIZARD to the latest release\n    upgrade   Alias for update\n    uninstall Uninstall LIZARD\n    check     Check source code\n    fmt       Format source code\n    test      Run project tests\n    new       Create a project\n    doctor    Diagnose installation\n\nOptions:\n    --version\n    --help\n    -e <code>"
    );
}

fn usage_error(message: &str) -> ! {
    eprintln!("LIZARD CLI ERROR: {message}\nUse `lizard --help` for usage.");
    process::exit(2);
}

fn file_error(path: &str, error: impl fmt::Display) -> ! {
    eprintln!("LIZARD FILE ERROR\n\nFile: {path}\n{error}");
    process::exit(4);
}

fn compile_error(path: &str, error: String) -> ! {
    eprintln!("LIZARD ERROR\n\nFile: {path}\n{error}");
    process::exit(3);
}
