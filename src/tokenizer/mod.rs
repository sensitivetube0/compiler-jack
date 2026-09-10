
use std::fs::{File};
use std::io::{BufReader};



pub trait Tokenizer {


    fn advance_token(&mut self);
    

    fn has_more_tokens(&self) -> bool;
    

    fn current_token_type(&self) -> Token;
}



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char, 
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}


impl Keyword {
   

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "class"       => Some(Keyword::Class),
            "constructor" => Some(Keyword::Constructor),
            "function"    => Some(Keyword::Function),
            "method"      => Some(Keyword::Method),
            "field"       => Some(Keyword::Field),
            "static"      => Some(Keyword::Static),
            "var"         => Some(Keyword::Var),
            "int"         => Some(Keyword::Int),
            "char"        => Some(Keyword::Char),
            "boolean"     => Some(Keyword::Boolean),
            "void"        => Some(Keyword::Void),
            "true"        => Some(Keyword::True),
            "false"       => Some(Keyword::False),
            "null"        => Some(Keyword::Null),
            "this"        => Some(Keyword::This),
            "let"         => Some(Keyword::Let),
            "do"          => Some(Keyword::Do),
            "if"          => Some(Keyword::If),
            "else"        => Some(Keyword::Else),
            "while"       => Some(Keyword::While),
            "return"      => Some(Keyword::Return),
            _             => None,
        }
    }
}





pub enum Token{

    KeywordToken(Keyword)

}



pub struct JackTokenizer{

    current_token:Option<Token>,
    files:Vec<BufReader<File>>

}


impl Tokenizer for JackTokenizer{

    fn advance_token(&mut self) {
        self.current_token = Some(Token::KeywordToken(Keyword::Class));
    
    }
    fn current_token_type(&self) -> Token{
        Token::KeywordToken(Keyword::Char)
    }
    fn has_more_tokens(&self) -> bool{
        false
    }

}
