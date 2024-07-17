use std::collections::HashMap;
use rand::prelude::Rng;
use crate::scanning::{CellIndex, LiteralValue, Token, TokenType, Tokenizer};

type Table = HashMap<CellIndex, Cell>;
type VisitingList = Vec<CellIndex>;

trait Expression
{
    fn evaluate(&mut self, expr_cells: &Table, value_cells: &mut Table, visiting: &mut VisitingList) -> LiteralValue;
}

struct Binary(Box<dyn Expression>, Token, Box<dyn Expression>);

impl Binary
{
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self
    {
        Binary(left, operator, right)
    }
}

impl Expression for Binary
{
    fn evaluate(&mut self, expr_cells: &Table, value_cells: &mut Table, visiting: &mut VisitingList) -> LiteralValue
    {
        let left = self.0.evaluate(expr_cells, value_cells, visiting);
        let right = self.2.evaluate(expr_cells, value_cells, visiting);

        let num1 =
        {
            match left
            {
                LiteralValue::Float(f) => f,
                _ => panic!("Expected numbers in binary expression")
            }
        };

        let num2 =
        {
            match right
            {
                LiteralValue::Float(f) => f,
                _ => panic!("Expected numbers in binary expression")
            }
        };

        match self.1.get_type()
        {
            TokenType::Plus  => LiteralValue::Float(num1 + num2),
            TokenType::Minus => LiteralValue::Float(num1 - num2),
            TokenType::Star  => LiteralValue::Float(num1 * num2),
            TokenType::Slash => LiteralValue::Float(num1 / num2),

            _ => panic!("Expected an operator")
        }
    }
}

struct Unary(Token, Box<dyn Expression>);

impl Unary
{
    pub fn new(operator: Token, expression: Box<dyn Expression>) -> Self
    {
        Unary(operator, expression)
    }
}

impl Expression for Unary
{
    fn evaluate(&mut self, expr_cells: &Table, value_cells: &mut Table, visiting: &mut VisitingList) -> LiteralValue
    {
        let expression = self.1.evaluate(expr_cells, value_cells, visiting);

        let num =
        {
            match expression
            {
                LiteralValue::Float(f) => f,
                _ => panic!("Expected numbers in binary expression")
            }
        };


        match self.0.get_type()
        {
            TokenType::Plus  => LiteralValue::Float(num),
            TokenType::Minus => LiteralValue::Float(-num),

            _ => panic!("Expected '+' or '-' operator"),
        }
    }
}

struct FnExpression(String, Vec<Box<dyn Expression>>);

impl FnExpression
{
    pub fn new(name: String, params: Vec<Box<dyn Expression>>) -> Self
    {
        FnExpression(name, params)
    }
}

impl Expression for FnExpression
{
    fn evaluate(&mut self, expr_cells: &Table, value_cells: &mut Table, visiting: &mut VisitingList) -> LiteralValue
    {
        match self.0.as_str()
        {
            "random" =>
            {
                if !self.1.is_empty()
                {
                    panic!("Function `random` doesn't take any arguments");
                }

                return LiteralValue::Float(rand::thread_rng().gen::<i32>() as f32);
            },
            "randbetween" =>
            {
                if self.1.len() != 2
                {
                    panic!("Function `randbetween` takes only 2 arguments");
                }

                let num1 =
                {
                    match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                    {
                        LiteralValue::Float(f) => f,
                        _ => panic!("Expected numbers as `randbetween` params")
                    }
                };

                let num2 =
                {
                    match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                    {
                        LiteralValue::Float(f) => f,
                        _ => panic!("Expected numbers as `randbetween` params")
                    }
                };

                if num1 >= num2
                {
                    panic!("First argument in `randbetween` should be smaller that the second");
                }

                return LiteralValue::Float(rand::thread_rng().gen_range(num1..num2));
            },
            "sum" =>
            {
                let mut sum = 0.0;

                while !self.1.is_empty()
                {
                    let num =
                    {
                        match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                        {
                            LiteralValue::Float(f) => f,
                            _ => panic!("Expected numbers as `sum` params")
                        }
                    };

                    sum += num;
                }

                return LiteralValue::Float(sum);
            },
            "average" =>
            {
                if self.1.is_empty()
                {
                    panic!("Function `max` expect at least one argument");
                }

                let len = self.1.len();
                let mut sum = 0.0;

                while !self.1.is_empty()
                {
                    sum +=
                    {
                        match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                        {
                            LiteralValue::Float(f) => f,
                            _ => panic!("Expected numbers as `sum` params")
                        }
                    };
                }

                return LiteralValue::Float(sum/(len as f32));
            },
            "max" =>
            {
                if self.1.is_empty()
                {
                    panic!("Function `max` expect at least one argument");
                }

                let mut max =
                {
                    match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                    {
                        LiteralValue::Float(f) => f,
                        _ => panic!("Expected numbers as `max` params")
                    }
                };

                while !self.1.is_empty()
                {
                    let num =
                    {
                        match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                        {
                            LiteralValue::Float(f) => f,
                            _ => panic!("Expected numbers as `max` params")
                        }
                    };

                    if num > max
                    {
                        max = num;
                    }
                }

                return LiteralValue::Float(max);
            },
            "min" =>
            {
                if self.1.is_empty()
                {
                    panic!("Function `min` expect at least one argument");
                }

                let mut min =
                {
                    match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                    {
                        LiteralValue::Float(f) => f,
                        _ => panic!("Expected numbers as `min` params")
                    }
                };

                while !self.1.is_empty()
                {
                    let num =
                    {
                        match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                        {
                            LiteralValue::Float(f) => f,
                            _ => panic!("Expected numbers as `min` params")
                        }
                    };

                    if num < min
                    {
                        min = num;
                    }
                }

                return LiteralValue::Float(min);
            },
            "if" =>
            {
                if self.1.len() != 3
                {
                    panic!("Function `if` takes only 3 arguments");
                }

                let first =
                {
                    match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                    {
                        LiteralValue::Float(f) => f,
                        _ => panic!("Expected numbers as `if` params")
                    }
                };

                let second =
                {
                    match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                    {
                        LiteralValue::Float(f) => f,
                        _ => panic!("Expected numbers as `if` params")
                    }
                };

                let third =
                {
                    match self.1.remove(0).evaluate(expr_cells, value_cells, visiting)
                    {
                        LiteralValue::Float(f) => f,
                        _ => panic!("Expected numbers as `if` params")
                    }
                };

                LiteralValue::Float(if first != 0.0 { second } else { third })
            },
            "vlookup" =>
            {
                todo!();
            },
            "concatenate" =>
            {
                todo!();
            },
            _ => todo!("Not all FUNCTIONS are implemented")
        }
    }
}

struct Literal(Token);

impl Literal
{
    pub fn new(literal: Token) -> Self
    {
        Literal(literal)
    }
}

impl Expression for Literal
{
    fn evaluate(&mut self, _expr_cells: &Table, _value_cells: &mut Table, _visiting: &mut VisitingList) -> LiteralValue
    {
        match self.0.get_type()
        {
            TokenType::Number => self.0.literal.take().unwrap(),
            _ => todo!()
        }
    }
}

pub struct CellRef(Token);

impl CellRef
{
    pub fn new(token: Token) -> Self
    {
        return CellRef(token);
    }

    pub fn text_to_number(column_name: String) -> usize
    {
        let column_name = column_name.to_uppercase();

        let mut sum: usize = 0;
        let ac = 'A' as usize;
    
        for i in 0..column_name.len()
        {
            sum *= 26;
            sum += (column_name.chars().nth(i).unwrap() as usize) - ac + 1;
        }
    
        return sum - 1;
    }

    pub fn number_to_text(column: usize) -> String
    {
        let mut f = column as f32;
        let mut s = String::new();

        loop
        {
            let c = f % 26.0;
            s.insert(0, std::char::from_u32((if (c as u32) == 0 { 26 } else { c as u32 }) + 64).unwrap());

            let old = f;
            f = f32::ceil(f / 26.0) - 1.0;

            if old <= 26.0 { break; }
        }

        s
    }
}

impl Expression for CellRef
{
    fn evaluate(&mut self, expr_cells: &Table, value_cells: &mut Table, visiting: &mut VisitingList) -> LiteralValue
    {
        if let LiteralValue::CellRef(cell_index) = self.0.literal
            .as_ref()
            .unwrap()
        {
            let (row, column) = cell_index.get();

            let cell_index = CellIndex::new(row, column);

            let cell = value_cells
                .get(&cell_index)
                .or(expr_cells.get(&cell_index))
                .expect("Refering to an unknown cell");

            match cell
            {
                Cell::Expression(expr) =>
                {
                    if visiting.iter().find(|x| *x == &cell_index).is_some()
                    {
                        let mut path = String::new();

                        for item in (&visiting).iter()
                        {
                            let (row, column) = item.get();

                            path.push_str(&(CellRef::number_to_text(row) + &column.to_string() + " -> "));
                        }
                        let (row, column) = &visiting.first().unwrap().get();

                        path.push_str(&(CellRef::number_to_text(*row) + &column.to_string()));

                        panic!("Cycle detected, {:?}", path)
                    }

                    visiting.push(cell_index.clone());

                    let tokenizer = Tokenizer::new(expr.to_string());
                    let mut parser: Parser = Parser::new(tokenizer.get_tokens());
                    let mut expression = parser.parse();

                    let evaluated = expression.evaluate(expr_cells, value_cells, visiting);

                    visiting.remove(
                        visiting
                                .iter()
                                .position(|x| *x == cell_index)
                                .unwrap());

                    value_cells.insert(cell_index,
                        match evaluated
                        {
                            LiteralValue::Float(f) => Cell::Value(f.to_string()),
                            _ => unreachable!()
                        });

                    return evaluated;
                },
                Cell::Value(value) =>
                {
                    // This should be changed if string literals will be supported
                    return LiteralValue::Float(value.parse::<f32>().unwrap_or_default());
                }
            }
        }

        unreachable!()
    }
}

struct Group(Box<dyn Expression>);

impl Group
{
    pub fn new(literal: Box<dyn Expression>) -> Self
    {
        Group(literal)
    }
}

impl Expression for Group
{
    fn evaluate(&mut self, expr_cells: &Table, value_cells: &mut Table, visiting: &mut VisitingList) -> LiteralValue
    {
        self.0.evaluate(expr_cells, value_cells, visiting)
    }
}

#[derive(Debug)]
pub enum Cell
{
    Value(String),
    Expression(String),
}

pub struct Parser
{
    tokens: Vec<Token>,
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        return Parser
        {
            tokens
        };
    }

    pub fn parse_file(file_content: String) -> String
    {
        let mut expr_cells = HashMap::<CellIndex, Cell>::new();
        let mut value_cells = HashMap::<CellIndex, Cell>::new();

        let lines = file_content.split('\n').collect::<Vec<&str>>();

        for (row, line) in lines.iter().enumerate()
        {
            let columns = line.split('|').collect::<Vec<&str>>();

            for (column, cell) in columns.iter().enumerate()
            {
                if cell.starts_with('=')
                {
                    let mut content = cell.to_string();
                    content.remove(0); // Delete '='
                    expr_cells.insert(
                        CellIndex::new(row, column),
                        Cell::Expression(content));
                }
                else
                {
                    value_cells.insert(
                        CellIndex::new(row, column),
                        Cell::Value(cell.to_string()));
                }
            }
        }

        for (index, cell) in &expr_cells
        {
            match cell
            {
                Cell::Expression(expr) =>
                {
                    let tokenizer = Tokenizer::new(expr.to_string());
                    let tokens = tokenizer.get_tokens();
                    let mut parser = Parser::new(tokens);
                    let mut expression = parser.parse();
                    let b =  expression.evaluate(&expr_cells, &mut value_cells, &mut vec![]);
                    value_cells.insert((*index).clone(), Cell::Value(
                        match b
                        {
                            LiteralValue::Float(f) => f.to_string(),
                            _ => unreachable!()
                        }));
                },
                _ => ()
            };
        }

        let mut sorted: Vec<(&CellIndex, &Cell)> = value_cells.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        
        let mut output = String::new();
        let mut last_line = 0;
        for (index, cell) in sorted
        {
            let (row, _) = index.get();
            if row != last_line
            {
                last_line = row;
                output += "\n";
            }

            match cell
            {
                Cell::Value(val)       => output.push_str(&format!("{: <10}", val)),
                Cell::Expression(expr) => output.push_str(&format!("{: <10}", expr)),
            };

            output += "|";
        }

        output += "\n";

        output
    }

    fn parse(&mut self) -> Box<dyn Expression>
    {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Expression>
    {
        self.term()
    }

    fn term(&mut self) -> Box<dyn Expression>
    {
        let mut expr = self.factor();

        while self.next_token_is(&[TokenType::Plus, TokenType::Minus])
        {
            let op = self.consume();
            let right = self.factor();
            expr = Box::new(Binary::new(expr, op, right));
        }

        expr
    }

    fn factor(&mut self) -> Box<dyn Expression>
    {
        let mut expr = self.unary();

        while self.next_token_is(&[TokenType::Star, TokenType::Slash])
        {
            let op = self.consume();
            let right = self.unary();
            expr = Box::new(Binary::new(expr, op, right));
        }

        expr
    }

    fn unary(&mut self) -> Box<dyn Expression>
    {
        if self.next_token_is(&[TokenType::Plus, TokenType::Minus])
        {
            let op = self.consume();
            let expression = self.unary();
            return Box::new(Unary::new(op, expression));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expression>
    {
        if self.next_token_is(&[TokenType::Number])
        {
            return Box::new(Literal::new(self.consume()));
        }

        if self.next_token_is(&[TokenType::CellRef])
        {
            return Box::new(CellRef::new(self.consume()));
        }
        
        if self.next_token_is(&[TokenType::OpeningParenthese])
        {
            self.consume(); // Consume '('

            let group = Group::new(self.expression());

            if !self.next_token_is(&[TokenType::ClosingParenthese])
            {
                panic!("Expected ')'");
            }

            self.consume(); // Consume ')'

            return Box::new(group);
        }

        if self.next_token_is(&[TokenType::Function])
        {
            let name = String::from(self.consume().get_lexeme());

            if !self.next_token_is(&[TokenType::OpeningParenthese])
            {
                panic!("Expected '(' after function name");
            }

            self.consume(); // Consume '('

            let mut params = Vec::<Box<dyn Expression>>::new();

            if !self.next_token_is(&[TokenType::ClosingParenthese])
            {
                params.push(self.expression());
            }

            while !self.next_token_is(&[TokenType::ClosingParenthese])
            {
                self.consume(); // Consume ','
                params.push(self.expression());
            }

            self.consume(); // Consume ')'

            return Box::new(FnExpression::new(name, params));
        }

        panic!("Invalid expression: {}", self.consume().get_lexeme());
    }

    fn consume(&mut self) -> Token
    {
        self.tokens.remove(0)
    }

    fn next_token_is(&self, types: &[TokenType]) -> bool
    {
        if self.tokens.is_empty()
        {
            return false;
        }

        let next = self.tokens.iter().nth(0);
        if let Some(n) = next
        {
            if types.contains(&n.get_type())
            {
                return true;
            }
        }

        false
    }
}
