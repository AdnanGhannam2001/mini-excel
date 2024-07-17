use crate::parsing::CellRef;

pub const FUNCTIONS: &[&str] =&
[
    "random",
    "randbetween",
    "sum",
    "average",
    "max",
    "min",
    "if",
    "vlookup",
    "concatenate",
];

#[derive(Debug, Clone)]
pub enum LiteralValue
{
    Float(f32),
    CellRef(CellIndex),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType
{
    Number,
    Plus, Minus, Star, Slash,
    OpeningParenthese, ClosingParenthese,
    CellRef,
    Function, Comma
}

#[derive(Debug)]
pub struct Token
{
    r#type: TokenType,
    lexeme: String,
    pub literal: Option<LiteralValue>,
}

impl PartialEq for Token
{
    fn eq(&self, other: &Self) -> bool
    {
        self.r#type == other.r#type && self.lexeme == other.lexeme
    }
}

impl Token
{
    pub fn new(t: TokenType, lexeme: String, literal: Option<LiteralValue>) -> Self
    {
        return Token
        {
            r#type: t,
            lexeme,
            literal,
        };
    }

    pub fn get_type(&self) -> &TokenType
    {
        &self.r#type
    }

    pub fn get_lexeme(&self) -> &String
    {
        &self.lexeme
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord)]
pub struct CellIndex
{
    row   : usize,
    column: usize,
}

impl CellIndex
{
    pub fn new(row: usize, column: usize) -> Self
    {
        CellIndex
        {
            row,
            column,
        }
    }

    pub fn get(&self) -> (usize, usize)
    {
        (self.row, self.column)
    }
}

pub struct Tokenizer
{
    tokens : Vec::<Token>,
    content: String,
    start  : usize,
    current: usize,
}

impl Tokenizer
{
    pub fn new(content: String) -> Self
    {
        return Tokenizer
        {
            tokens: Vec::new(),
            content,
            start: 0,
            current: 0,
        };
    }

    pub fn get_tokens(mut self) -> Vec<Token>
    {
        while !self.is_at_end()
        {
            self.scan_token();
        }
        
        return self.tokens;
    }

    pub fn scan_token(&mut self) -> ()
    {
        while !self.is_at_end()
        {
            let c = self.get_current_char();
            match c
            {
                '\n' | ' ' | '\t' => self.start = self.current,

                '(' => self.add_token(TokenType::OpeningParenthese, String::from('(')),
                ')' => self.add_token(TokenType::ClosingParenthese, String::from(')')),

                '+' => self.add_token(TokenType::Plus, String::from('+')),
                '-' => self.add_token(TokenType::Minus, String::from('-')),
                '*' => self.add_token(TokenType::Star, String::from('*')),
                '/' => self.add_token(TokenType::Slash, String::from('/')),

                ',' => self.add_token(TokenType::Comma, String::from(',')),

                _ =>
                {
                    if Tokenizer::is_number(&c)
                    {
                        self.number();
                    }
                    else if Tokenizer::is_alpha(&c)
                    {
                        self.string();
                    }
                    else
                    {
                        let lexeme = self.content[self.start..self.current].to_string();
                        panic!("Unknown token: `{}` at: {}..{}", lexeme, self.start, self.current);
                    }
                },
            }
        }
    }

    fn number(&mut self) -> ()
    {
        while !self.is_at_end() && Tokenizer::is_number(&self.peak().unwrap()) { self.current += 1; }

        if !self.is_at_end() && self.peak().unwrap() == '.'
        {
            self.get_current_char(); // Consume '.'

            if self.is_at_end() || !Tokenizer::is_number(&self.get_current_char())
            {
                let lexeme = self.content[self.start..self.current].to_string();
                panic!("Invalid token while scanning number: `{}` at: {}..{}", lexeme, self.start, self.current);
            }

            while !self.is_at_end() && Tokenizer::is_number(&self.peak().unwrap()) { self.current += 1; }
        }

        let lexeme = self.content[self.start..self.current].to_string();
        self.add_token_with_literal(TokenType::Number,
            lexeme.clone(), LiteralValue::Float(lexeme.parse::<f32>().unwrap()));
        
        self.start = self.current;
    }

    fn string(&mut self) -> ()
    {
        while !self.is_at_end() && Tokenizer::is_alpha(&self.peak().unwrap()) { self.current += 1; }

        let ends_with_number = !self.is_at_end() && Tokenizer::is_number(&self.peak().unwrap());

        let numbers_count = self.current - self.start;

        while !self.is_at_end() && Tokenizer::is_number(&self.peak().unwrap()) { self.current += 1; }

        let lexeme = self.content[self.start..self.current].to_string();
        let func = FUNCTIONS.iter().find(|&&s| &s == &lexeme.to_ascii_lowercase().as_str());
        
        if !ends_with_number && func.is_none()
        {
            let lexeme = self.content[self.start..self.current].to_string();
            panic!("Invalid token while scanning cell_ref: `{}` at: {}..{}", lexeme, self.start, self.current);
        }

        if func.is_some()
        {
            self.add_token(TokenType::Function, lexeme);
        }
        else
        {
            self.add_token_with_literal(TokenType::CellRef,
                lexeme.clone(),
                    LiteralValue::CellRef(CellIndex::new(
                        CellRef::text_to_number(lexeme[..numbers_count].to_string()),
                        lexeme[numbers_count..].parse::<usize>().unwrap())));
        }

        self.start = self.current;
    }

    fn peak(&self) -> Option<char>
    {
        self.content.chars().nth(self.current)
    }

    fn get_current_char(&mut self) -> char
    {
        let c = self.content.chars().nth(self.current);
        self.current += 1;
        c.unwrap()
    }

    fn add_token(&mut self, t: TokenType, lexeme: String) -> ()
    {
        self.tokens.push(Token::new(t, lexeme, Option::None));
        self.start = self.current;
    }

    fn add_token_with_literal(&mut self, t: TokenType, lexeme: String, literal: LiteralValue) -> ()
    {
        self.tokens.push(Token::new(t, lexeme, Option::Some(literal)));
        self.start = self.current;
    }

    fn is_at_end(&self) -> bool
    {
        self.current >= self.content.len()
    }

    fn is_alpha(c: &char) -> bool
    {
        (c >= &'a' && c <= &'z') || (c >= &'A' && c <= &'Z')
    }

    fn is_number(c: &char) -> bool
    {
        return c >= &'0' && c <= &'9';
    }
}
