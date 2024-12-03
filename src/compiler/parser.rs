//! A custom SpRE parser.
//!
//! Currently, the parser is manually implemented from a Context-Free Grammar
//! (CFG) definition. For grammar details, see relevant function documentation.

use std::collections::HashMap;

use super::ir::ast::{AbstractSyntaxTree, OperandKind, SpatialFormula};
use super::ir::ops::{
    FolOperatorKind, Operator, RangeKind, RegexOperatorKind, S4OperatorKind, S4mOperatorKind,
    S4uOperatorKind, SpatialOperatorKind,
};
use super::ir::Node;
use super::lexer::stream::TokenStream;
use super::lexer::token::{Token, TokenKind, TokenKind::*};
use super::listener::ErrorListener;

/// The SpRE parser.
///
/// The parser is responsible for tracking its current location on the
/// [`TokenStream`] and asserting the next token is the correct.
pub struct Parser {
    stream: TokenStream,
    listener: Option<ErrorListener>,
    current: usize,
}

impl Parser {
    /// Create a new [`Parser`].
    pub fn new(stream: TokenStream) -> Self {
        Parser {
            stream,
            listener: None,
            current: 0,
        }
    }

    /// Attach an [`ErrorListener`] to the [`Parser`].
    ///
    /// This attachment allows for better syntactical error reporting by the
    /// parsing process based on the provided listener. If an error listener is
    /// not attached to the [`Parser`], it panics.
    pub fn attach(mut self, listener: ErrorListener) -> Self {
        self.listener = Some(listener);
        self
    }

    /// Retrieve the next token from stream and compare against expected.
    ///
    /// If the next token from stream is not the expected token, then throw a
    /// fatal error messsage and exit as this situation is not recoverable in its
    /// current implementation.
    fn expect(&mut self, kind: TokenKind) -> Token {
        if self.stream.buffer[self.current].kind != kind {
            match &self.listener {
                Some(listener) => {
                    listener.exit(
                        format!(
                            "parser: {}:{}: expected {:?} but found {:?}",
                            self.stream.buffer[self.current].position.0,
                            self.stream.buffer[self.current].position.1,
                            kind,
                            self.stream.buffer[self.current].kind,
                        ),
                        1,
                    );
                }
                None => {
                    panic!();
                }
            }
        }

        self.current += 1;
        self.stream.buffer[self.current - 1].clone()
    }

    /// Fatally report a syntax error.
    ///
    /// The syntax error can derive from various sources. Therefore, the position
    /// of the offending token is provided as a general tip to debug.
    fn error(&self) {
        match &self.listener {
            Some(listener) => listener.exit(
                format!(
                    "parser: {}:{}: syntax error",
                    self.stream.buffer[self.current].position.0,
                    self.stream.buffer[self.current].position.1
                ),
                2,
            ),
            None => {
                panic!();
            }
        }
    }

    /// Lookahead into the [`TokenStream`] a specified amount.
    ///
    /// This method is used in order to make parsing decisions for rules that are
    /// recursive by definition or may have several productions.
    fn peek(&self, lookahead: usize) -> Option<&Token> {
        if self.current + (lookahead - 1) >= self.stream.size {
            return None;
        }

        Some(&self.stream.buffer[self.current + (lookahead - 1)])
    }

    /// Parse the [`TokenStream`] according to the SpRE CFG.
    ///
    /// This method parses the initialized [`TokenStream`] and produces a
    /// [`AbstractSyntaxTree`] (aka, an Abstract Syntax Tree) populated with the relevant
    /// information. In most cases this means dropping parentheses.
    pub fn parse(&mut self) -> AbstractSyntaxTree {
        let root = if let Some(token) = self.peek(1) {
            if token.kind != EndOfFile {
                self.parse_spre()
            } else {
                None
            }
        } else {
            None
        };

        self.expect(EndOfFile);

        AbstractSyntaxTree::new(root)
    }

    /// Parse a Regular Expression-based expression.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// phi ::= '(' phi ')' | phi '*' | phi phi | phi '|' phi | phi range
    ///       | '[' pi ']'
    /// ```
    ///
    /// Note: The following symbol(s) have a different semantic meaning derived
    /// at parse time:
    ///
    /// `|`: Alternation
    fn parse_spre(&mut self) -> Option<Node<SpatialFormula>> {
        let mut node = None;

        if let Some(token) = self.peek(1) {
            match token.kind {
                LeftParen => {
                    self.expect(LeftParen);
                    node = self.parse_spre();
                    self.expect(RightParen);
                }
                LeftBracket => {
                    self.expect(LeftBracket);
                    let tree = self.parse_s4u();
                    self.expect(RightBracket);

                    node = Some(Node::from(tree.unwrap()));
                }
                _ => self.error(),
            }
        };

        while let Some(token) = self.peek(1) {
            if token.kind != EndOfFile {
                match token.kind {
                    // kleene-star
                    Star => {
                        self.expect(Star);
                        node = Some(Node::unary(
                            Operator::RegexOperator(RegexOperatorKind::KleeneStar),
                            node.unwrap(),
                        ));
                    }

                    // concatenation
                    LeftParen | LeftBracket => {
                        let right = self.parse_spre();
                        node = Some(Node::binary(
                            Operator::RegexOperator(RegexOperatorKind::Concatenation),
                            node.unwrap(),
                            right.unwrap(),
                        ));
                    }

                    // alternation
                    Or => {
                        self.expect(Or);

                        let right = self.parse_spre();
                        node = Some(Node::binary(
                            Operator::RegexOperator(RegexOperatorKind::Alternation),
                            node.unwrap(),
                            right.unwrap(),
                        ))
                    }

                    // range
                    LeftBrace => {
                        let range = self.parse_range();
                        node = Some(Node::unary(
                            Operator::RegexOperator(RegexOperatorKind::Range(range.unwrap())),
                            node.unwrap(),
                        ));
                    }

                    _ => break,
                }
            } else {
                break;
            }
        }

        node
    }

    /// Parse an S4u-based expression.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// pi ::= '(' pi ')' | pi '&' pi | pi '|' pi | NonEmpty class
    ///      | NonEmpty '(' tau ')' | class
    ///      | 'E' '(' bindings ')' pi
    ///      | 'A' '(' bindings ')' pi
    ///      | psi < psi
    /// ```
    ///
    /// Note: The following symbol(s) have a different semantic meaning derived
    /// at parse time:
    ///
    /// `~`: Negation
    /// `&`: Conjunction
    /// `|`: Disjunction
    fn parse_s4u(&mut self) -> Option<SpatialFormula> {
        let mut node = None;

        if let Some(token) = self.peek(1) {
            match token.kind {
                LeftParen => {
                    self.expect(LeftParen);
                    node = self.parse_s4u();
                    self.expect(RightParen);
                }

                Not => {
                    self.expect(Not);

                    let child = self.parse_s4u();
                    node = Some(Node::unary(
                        Operator::SpatialOperator(SpatialOperatorKind::FolOperator(
                            FolOperatorKind::Negation,
                        )),
                        child.unwrap(),
                    ));
                }

                NonEmpty => {
                    self.expect(NonEmpty);

                    // The behavior of the NonEmpty operator is non-greedy.
                    // Therefore, it should consume only the next token and
                    // decide what to do from there. The two cases are as
                    // follows:
                    //
                    //   1. A class is seen: Consume the class and return.
                    //   2. A parenthesis is seen: Consume everything between the
                    //      parenthesis (i.e., an S4 expression).
                    let child = if let Some(token) = self.peek(1) {
                        match token.kind {
                            TokenKind::LeftBracket => self.parse_class(),
                            TokenKind::LeftParen => {
                                self.expect(LeftParen);
                                let child = self.parse_s4();
                                self.expect(RightParen);

                                child
                            }
                            _ => {
                                self.error();
                                None
                            }
                        }
                    } else {
                        self.error();
                        None
                    };

                    node = Some(Node::unary(
                        Operator::SpatialOperator(SpatialOperatorKind::S4uOperator(
                            S4uOperatorKind::NonEmpty,
                        )),
                        child.unwrap(),
                    ));
                }

                Exists => {
                    self.expect(Exists);
                    self.expect(LeftParen);
                    let table = self.parse_bindings();
                    self.expect(RightParen);

                    let child = self.parse_s4u();

                    node = Some(Node::unary(
                        Operator::SpatialOperator(SpatialOperatorKind::S4uOperator(
                            S4uOperatorKind::Exists(table),
                        )),
                        child.unwrap(),
                    ));
                }

                Forall => {
                    self.expect(Forall);
                    self.expect(LeftParen);
                    let table = self.parse_bindings();
                    self.expect(RightParen);

                    let child = self.parse_s4u();

                    node = Some(Node::unary(
                        Operator::SpatialOperator(SpatialOperatorKind::S4uOperator(
                            S4uOperatorKind::Forall(table),
                        )),
                        child.unwrap(),
                    ));
                }

                At | Integer | Real | Minus => {
                    let lhs = self.parse_s4m();

                    let mut op = None;
                    if let Some(token) = self.peek(1) {
                        match token.kind {
                            LeftChevron => {
                                self.expect(LeftChevron);
                                op = Some(Operator::SpatialOperator(
                                    SpatialOperatorKind::FolOperator(FolOperatorKind::LessThan),
                                ))
                            }
                            RightChevron => {
                                self.expect(RightChevron);
                                op = Some(Operator::SpatialOperator(
                                    SpatialOperatorKind::FolOperator(FolOperatorKind::GreaterThan),
                                ))
                            }
                            LeftChevronEqual => {
                                self.expect(LeftChevronEqual);
                                op = Some(Operator::SpatialOperator(
                                    SpatialOperatorKind::FolOperator(
                                        FolOperatorKind::LessThanEqualTo,
                                    ),
                                ))
                            }
                            RightChevronEqual => {
                                self.expect(RightChevronEqual);
                                op = Some(Operator::SpatialOperator(
                                    SpatialOperatorKind::FolOperator(
                                        FolOperatorKind::GreaterThanEqualTo,
                                    ),
                                ))
                            }
                            _ => self.error(),
                        };
                    }

                    let rhs = self.parse_s4m();
                    node = Some(Node::binary(op.unwrap(), lhs.unwrap(), rhs.unwrap()));
                }

                // class
                LeftBracket => {
                    node = self.parse_class();
                }
                _ => self.error(),
            }
        } else {
            self.error();
        }

        while let Some(token) = self.peek(1) {
            if token.kind != EndOfFile {
                match token.kind {
                    // conjunction
                    And => {
                        self.expect(And);

                        let right = self.parse_s4u();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::FolOperator(
                                FolOperatorKind::Conjunction,
                            )),
                            node.unwrap(),
                            right.unwrap(),
                        ));
                    }

                    // disjunction
                    Or => {
                        self.expect(Or);

                        let right = self.parse_s4u();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::FolOperator(
                                FolOperatorKind::Disjunction,
                            )),
                            node.unwrap(),
                            right.unwrap(),
                        ));
                    }

                    _ => break,
                }
            } else {
                break;
            }
        }

        node
    }

    /// Parse a set of bindings.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// bindings ::= Identifier Walrus class
    ///            | Identifier Walrus class Comma bindings
    /// ```
    ///
    fn parse_bindings(&mut self) -> HashMap<String, SpatialFormula> {
        let mut table = HashMap::new();

        let variable = self.expect(Identifier);
        self.expect(Walrus);
        let class = self.parse_class();

        // Insert the quantified variable.
        //
        // This creates a new entry with the name of the variable that is
        // associated with a [`class`].
        table.insert(variable.lexeme, class.unwrap());

        if let Some(token) = self.peek(1) {
            match token.kind {
                Comma => {
                    self.expect(Comma);
                    table.extend(self.parse_bindings());
                }
                _ => return table,
            }
        }

        table
    }

    /// Parse an S4m-based expression.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// psi ::= '(' psi ')' | Real | Integer | '\' Identifier '(' tau ')'
    ///       | '\' Identifier '(' tau ',' tau ')' | '-' psi
    ///       | psi '-' psi | psi '*' psi | psi '/' psi
    /// ```
    fn parse_s4m(&mut self) -> Option<SpatialFormula> {
        let mut node = None;

        if let Some(token) = self.peek(1) {
            match token.kind {
                LeftParen => {
                    self.expect(LeftParen);
                    node = self.parse_s4m();
                    self.expect(RightParen);
                }

                // function
                At => {
                    self.expect(At);
                    let name = self.expect(Identifier);

                    self.expect(LeftParen);
                    let child = self.parse_s4();

                    if let Some(peeked) = self.peek(1) {
                        match peeked.kind {
                            Comma => {
                                self.expect(Comma);
                                let right = self.parse_s4();

                                node = Some(Node::binary(
                                    Operator::SpatialOperator(SpatialOperatorKind::S4mOperator(
                                        S4mOperatorKind::Function(name.lexeme),
                                    )),
                                    child.unwrap(),
                                    right.unwrap(),
                                ));
                            }
                            _ => {
                                node = Some(Node::unary(
                                    Operator::SpatialOperator(SpatialOperatorKind::S4mOperator(
                                        S4mOperatorKind::Function(name.lexeme),
                                    )),
                                    child.unwrap(),
                                ))
                            }
                        }
                    }

                    self.expect(RightParen);
                }

                // reals
                Real => {
                    let number = self.expect(Real);
                    node = Some(Node::from(OperandKind::Number(
                        number.lexeme.parse().unwrap(),
                    )));
                }

                // integer
                Integer => {
                    let number = self.expect(Integer);
                    node = Some(Node::from(OperandKind::Number(
                        number.lexeme.parse().unwrap(),
                    )));
                }

                // inverse
                Minus => {
                    self.expect(Minus);
                    let child = self.parse_s4m();

                    node = Some(Node::unary(
                        Operator::SpatialOperator(SpatialOperatorKind::S4mOperator(
                            S4mOperatorKind::Inverse,
                        )),
                        child.unwrap(),
                    ));
                }

                _ => self.error(),
            }
        } else {
            self.error();
        }

        while let Some(token) = self.peek(1) {
            if token.kind != EndOfFile {
                match token.kind {
                    // addition
                    Plus => {
                        self.expect(Plus);

                        let rhs = self.parse_s4m();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::S4mOperator(
                                S4mOperatorKind::Addition,
                            )),
                            node.unwrap(),
                            rhs.unwrap(),
                        ));
                    }

                    // subtraction
                    Minus => {
                        self.expect(Minus);

                        let rhs = self.parse_s4m();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::S4mOperator(
                                S4mOperatorKind::Subtraction,
                            )),
                            node.unwrap(),
                            rhs.unwrap(),
                        ));
                    }

                    // multiplication
                    Star => {
                        self.expect(Star);

                        let rhs = self.parse_s4m();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::S4mOperator(
                                S4mOperatorKind::Multiplication,
                            )),
                            node.unwrap(),
                            rhs.unwrap(),
                        ));
                    }

                    // division
                    Slash => {
                        self.expect(Slash);

                        let rhs = self.parse_s4m();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::S4mOperator(
                                S4mOperatorKind::Division,
                            )),
                            node.unwrap(),
                            rhs.unwrap(),
                        ));
                    }

                    _ => break,
                }
            } else {
                break;
            }
        }

        node
    }

    /// Parse an S4-based expression.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// tau ::= '(' tau ')' | tau '&' tau | tau '|' tau | '!' tau | class
    /// ```
    ///
    /// Note: The following symbol(s) have a different semantic meaning derived
    /// at parse time:
    ///
    /// `&`: Intersection
    /// `|`: Union
    /// `!`: Complementation
    fn parse_s4(&mut self) -> Option<SpatialFormula> {
        let mut node = None;

        if let Some(token) = self.peek(1) {
            match token.kind {
                LeftParen => {
                    self.expect(LeftParen);
                    node = self.parse_s4();
                    self.expect(RightParen);
                }

                Identifier => {
                    let name = self.expect(Identifier);
                    node = Some(Node::from(OperandKind::Variable(name.lexeme)));
                }

                // complementation
                Not => {
                    self.expect(Not);

                    let child = self.parse_s4();
                    node = Some(Node::unary(
                        Operator::SpatialOperator(SpatialOperatorKind::S4Operator(
                            S4OperatorKind::Complement,
                        )),
                        child.unwrap(),
                    ));
                }

                // class
                LeftBracket => {
                    node = self.parse_class();
                }
                _ => self.error(),
            }
        } else {
            self.error();
        }

        while let Some(token) = self.peek(1) {
            if token.kind != EndOfFile {
                match token.kind {
                    // intersection
                    And => {
                        self.expect(And);

                        let right = self.parse_s4();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::S4Operator(
                                S4OperatorKind::Intersection,
                            )),
                            node.unwrap(),
                            right.unwrap(),
                        ));
                    }

                    // union
                    Or => {
                        self.expect(Or);

                        let right = self.parse_s4();
                        node = Some(Node::binary(
                            Operator::SpatialOperator(SpatialOperatorKind::S4Operator(
                                S4OperatorKind::Union,
                            )),
                            node.unwrap(),
                            right.unwrap(),
                        ));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        node
    }

    /// Parse a class.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// class ::= object
    /// ```
    fn parse_class(&mut self) -> Option<SpatialFormula> {
        self.parse_object()
    }

    /// Parse an object.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// object ::= '[' ':' Identifier ':' ']'
    /// ```
    fn parse_object(&mut self) -> Option<SpatialFormula> {
        self.expect(LeftBracket);
        self.expect(Colon);
        let name = self.expect(Identifier).lexeme;
        self.expect(Colon);
        self.expect(RightBracket);

        Some(Node::from(OperandKind::Symbol(name)))
    }

    /// Parse a range.
    ///
    /// This parse function captures the following grammar:
    ///
    /// ```text
    /// range ::= '{' Integer '}' | '{' Integer ',' '}'
    ///         | '{' Integer ',' Integer '}'
    /// ```
    fn parse_range(&mut self) -> Option<RangeKind> {
        self.expect(LeftBrace);
        let min = self.expect(Integer).lexeme.parse().unwrap();

        let range: Option<RangeKind> = if let Some(token) = self.peek(1) {
            if token.kind == Comma {
                self.expect(Comma);

                // TODO: It's possible that we match against both an Integer and
                // Real and provide feedback that the real cannot be used in a
                // range operation to the user.
                if let Some(token) = self.peek(1) {
                    if token.kind == Integer {
                        let max = self.expect(Integer).lexeme.parse().unwrap();
                        Some(RangeKind::Between(min, max))
                    } else {
                        Some(RangeKind::AtLeast(min))
                    }
                } else {
                    None
                }
            } else {
                Some(RangeKind::Exactly(min))
            }
        } else {
            None
        };

        self.expect(RightBrace);

        range
    }
}
