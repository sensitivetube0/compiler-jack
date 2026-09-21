
// imports
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{File};
use std::io::{self, BufRead, BufReader,Write,Read};
use std::path::Path;
use crate::handle_file_or_dir_given::FilesInPathBuf;
use std::fs::OpenOptions;





// Tokenizer trait that is implemented by are JackTokenizer
pub trait Tokenizer {
    fn advance_token(&mut self);
    fn has_more_tokens(&self) -> bool;
    fn skip_file(&mut self);
    fn current_token_type(&self) -> Option<&Token>;
}

pub trait CanCheckEq{
    fn matches_token(&self,token_type:&Token) -> bool;
}

#[derive(Debug,PartialEq, Eq)]
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

impl CanCheckEq for Keyword {
        fn matches_token(&self,token_type:&Token) -> bool {
            match token_type {
                Token::KeywordToken(actual_kw) => actual_kw == self,
            _ => false,
            }
        }
}

#[derive(Debug,PartialEq,Clone)]
pub enum Identifier{
    SequenceOfChars(String)
}

impl CanCheckEq for Identifier {

    fn matches_token(&self,token_type:&Token) -> bool {
            match token_type {
                Token::IdentifierToken(actual_kw) => actual_kw == self,
            _ => false,
            }
        }

}



#[derive(Debug,PartialEq, Eq)]
pub enum Symbol{


    RightCurlyBracket,
    LeftCurlyBracket,
    RightParentis,
    LeftParentis,
    RightSquareBracket,
    LeftSquareBracket,
    Period,
    Comma,
    SemiColon,
    Plus,
    Minus,
    Multiply,
    Divide,
    AmperSand,
    Pipe,
    LessThan,
    GreaterThan,
    Equals,
    Tilde,
}

impl CanCheckEq for Symbol {
    fn matches_token(&self,token_type:&Token) -> bool {
            match token_type {
                Token::SymbolToken(actual_kw) => actual_kw == self,
            _ => false,
            }
        }
}

#[derive(Debug,PartialEq, Eq)]
pub enum Integer{
    Integer(i32), // stores the integer
}

impl CanCheckEq for Integer {


  
    fn matches_token(&self,token_type:&Token) -> bool {
            match token_type {
                Token::IntegerToken(actual_kw) => actual_kw == self,
            _ => false,
            }
        }

}


#[derive(Debug,PartialEq)]
pub enum StringConstant{
    String(String), // needs to store the String constant
}

// holds Token of all the different enums
#[derive(Debug,PartialEq)]
pub enum Token{
    KeywordToken(Keyword),
    SymbolToken(Symbol),
    IntegerToken(Integer),
    StringToken(StringConstant),
    IdentifierToken(Identifier)
}



impl Token{

    fn get_variant_name(&self) -> &'static str{

        match self{
            Token::KeywordToken(_) => "Keyword",
            Token::SymbolToken(_) => "Symbol",
            Token::IntegerToken(_) => "Integer",
            Token::StringToken(_) => "String",
            Token::IdentifierToken(_) => "Identifier",
        }
    }
    fn get_value_from_token(&self) -> String{


        match self{
            Token::KeywordToken(keyword_token) => keyword_token.to_str().to_string(),
            Token::SymbolToken(symbol_token) => symbol_token.to_str().to_string(),
            Token::IntegerToken(Integer::Integer(val)) => val.to_string(),
            Token::StringToken(StringConstant::String(val)) => val.to_string(),
            Token::IdentifierToken(Identifier::SequenceOfChars(val)) => val.to_string(  ),
        }





    }

}






impl Keyword {
   
   // returns true is &str is a keyword false otherwise
    pub fn is_keyword(s: &str) -> bool{
        matches!(s,
            "class" |
            "constructor" |
            "function"  | 
            "method"   |  
            "field"  |
            "static"  |    
            "var"    |                  
            "int"   |          
            "char"  |       
            "boolean"  |
            "void"   |      
            "true"   |      
            "false"  | 
            "null"  |       
            "this"  |       
            "let"  |               
            "do"  |                    
            "if"   |                   
            "else"  |       
            "while"  |      
            "return"
        )
    }


    // takes in &str and returns Option<Keyword>
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
    pub fn to_str(&self) -> &'static str {
        match self {
            Keyword::Class       => "class",
            Keyword::Constructor => "constructor",
            Keyword::Function    => "function",
            Keyword::Method      => "method",
            Keyword::Field       => "field",
            Keyword::Static      => "static",
            Keyword::Var         => "var",
            Keyword::Int         => "int",
            Keyword::Char        => "char",
            Keyword::Boolean     => "boolean",
            Keyword::Void        => "void",
            Keyword::True        => "true",
            Keyword::False       => "false",
            Keyword::Null        => "null",
            Keyword::This        => "this",
            Keyword::Let         => "let",
            Keyword::Do          => "do",
            Keyword::If          => "if",
            Keyword::Else        => "else",
            Keyword::While       => "while",
            Keyword::Return      => "return",
        }
    }
    
}


impl Identifier{
    // takes in &str and converts to identifier token
    pub fn from_str(s:&str) -> Self{
        Identifier::SequenceOfChars(s.to_string())
    }
}





// const of all symbols
const SYMBOLS:[char;19] =   ['{',
            '}',
            '(',
            ')',
            '[',
            ']',
            '.',
            ',',
            ';',
            '+',
            '-',
            '*',
            '/',
            '&',
            '|',
            '<',
            '>',
            '=',
            '~'];



impl Symbol{

    // just checks if char is contained with predefined Array of symbols
    pub fn is_symbol_from_char(c:&char) -> bool{
          SYMBOLS.contains(c)
    }


    // takes in &str and converts to Option<Symbol>
    pub fn from_str(s:&str) -> Option<Self>{

        match s{

            "{" => Some(Self::LeftCurlyBracket),
            "}" => Some(Self::RightCurlyBracket),
            "(" => Some(Self::LeftParentis),
            ")" => Some(Self::RightParentis),
            "[" => Some(Self::LeftSquareBracket),
            "]" => Some(Self::RightSquareBracket),
            "." => Some(Self::Period),
            "," => Some(Self::Comma),
            ";" => Some(Self::SemiColon),
            "+" => Some(Self::Plus),
            "-" => Some(Self::Minus),
            "*" => Some(Self::Multiply),
            "/" => Some(Self::Divide),
            "&" => Some(Self::AmperSand),
            "|" => Some(Self::Pipe),
            "<" => Some(Self::LessThan),
            ">" => Some(Self::GreaterThan),
            "=" => Some(Self::Equals),
            "~" => Some(Self::Tilde),
             _ => None,
        }
    }

     pub fn to_str(&self) -> &'static str {
        match self {
            Self::LeftCurlyBracket  => "{",
            Self::RightCurlyBracket => "}",
            Self::LeftParentis      => "(",
            Self::RightParentis     => ")",
            Self::LeftSquareBracket => "[",
            Self::RightSquareBracket=> "]",
            Self::Period            => ".",
            Self::Comma             => ",",
            Self::SemiColon         => ";",
            Self::Plus              => "+",
            Self::Minus             => "-",
            Self::Multiply          => "*",
            Self::Divide            => "/",
            Self::AmperSand         => "&",
            Self::Pipe              => "|",
            Self::LessThan          => "<",
            Self::GreaterThan       => ">",
            Self::Equals            => "=",
            Self::Tilde             => "~",
        }
    }



}



impl Integer{


    // takes in &str tries to parse to return bool if it is and int
    pub fn is_integer(s:&str) -> bool{
    match s.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
    }


    // converts &str into Option<Integer>
    pub fn from_str(s:&str) -> Option<Self>{
    
        
    match s.parse::<i32>() {
        Ok(num) => Some(Integer::Integer(num)),
        Err(_) => None,
    }
}

}


impl StringConstant{

    // takes &str ensures &str begins and ends with " char if not returns err if it does it returns StringConstant token
    pub fn from_str(s:&str) -> Result<Self,Box<dyn Error>>{

        let mut chars = s.chars();
        let Some('"') = chars.next() else{
            return Err("Expected first character to be \"".into())
        };

        let Some('"') = chars.next_back() else{
            return Err("Expected last character to be \"".into())
            
        };


        Ok(StringConstant::String(chars.as_str().to_string()))
    }

}









// fields required for JackTokenizer struct
pub struct JackTokenizer{

    current_token:Option<Token>,
    files:FilesInPathBuf,
    next_file_to_read_idx:usize,
    current_file:BufReader<File>,
    more_tokens_to_read:MoreTokensToRead,
    line_number:u32,
}

struct MoreTokensToRead{

    more_tokens:bool,

}


impl Tokenizer for JackTokenizer{

   
    fn skip_file(&mut self){
        if self.more_files_to_read(){
        self.line_number = 1;
        self.open_next_file();
        }else{
            self.more_tokens_to_read.more_tokens = false;
            return;
        }
    }
    // self.more_tokens_to_read is only set to false when no more files or chars are available in all files given
    fn has_more_tokens(&self)-> bool {
        self.more_tokens_to_read.more_tokens
    }

    // advances token by setting self.current_token to next token found
    fn advance_token(&mut self){

        // let mut current_char:char;

       loop{

  
        if let Some(char) = self.peak_char(){ // checks the next char without reading

            if char == '\n'{
                // increments line number when found \n for error messaging
                self.line_number = self.line_number + 1; 
            }

            // if it is whitespace we read char and continue to next loop eating all white space until non whitespace is found
            if char.is_whitespace(){
                self.read_char();
                continue;
            }
           
            // breaks as non whitespace char found
            break;
        }
        
        // if we reach here it means peak_char returned none which means no chars left in file
        // we check if more files to read if so we open next one reset line number if not we set self.more_tokens_to_read = false and finish
        self.skip_file();
        if !self.more_tokens_to_read.more_tokens{
            return;
        }
        }


        let mut chars_for_token:String = String::with_capacity(5); // average number of chars in most known words that can be used
        let mut current_char = self.read_char(); // char will be non whitespace

        loop{
            chars_for_token.push(current_char);

            // if current_char is string literal special case as do not want to break on white space
            if current_char == '"'{
                // read next and push into vec
                current_char = self.read_char();
                chars_for_token.push(current_char);

                // if not " read into vec
                while current_char != '"'{

                // if peak char returns none it means we have reached end of file this is a error we handle gracefully later in StringConstant::from_str as last char is not "
                match self.peak_char(){
                    Some(_) => (),
                    None => break
                }

                current_char = self.read_char();
                chars_for_token.push(current_char);
                }
                break;
            }


            if Symbol::is_symbol_from_char(&current_char){
                break;
            }

            match self.peak_char() {
                Some(char) => {

                    if char == '\n'{
                    self.line_number = self.line_number + 1; // increment line number when \n found
                    }
                    if Symbol::is_symbol_from_char(&char){
                        break;
                    }

                    if char.is_whitespace(){
                        self.read_char(); // read off the whitespace as if \n do not want to increment again

                        break; // break full token found
                    }
                }
                None => break, // means we have reached end of file
            }

            current_char = self.read_char(); // read in next char
        }

        let token = self.create_token_from_str(&chars_for_token);
        self.current_token = Some(token);
        self.print_debug_xml_to_file();
        println!("token: {:?}",self.current_token);
       // println!("line: {}",self.line_number);
      
        return;
        
    }

    //returns current token
    fn current_token_type(&self) -> Option<&Token>{
        self.current_token.as_ref()
    }
 

}


impl JackTokenizer{




    pub fn get_line_number(&self) -> u32{
     self.line_number   
    }
    pub fn file_stem_current(&self) -> String {

        let current_file = self.files.get_index_in_paths(self.next_file_to_read_idx -1);
        

        current_file.file_stem().unwrap().to_string_lossy().into_owned()
    }

    fn print_debug_xml_to_file(&mut self){

        
        let print_debug_msg = "Error printing debug xml please try again at a later point";
        let current_file = self.files.get_index_in_paths(self.next_file_to_read_idx - 1);
        let file_name = current_file.file_name();

        let Some(file_name) = file_name else{
            panic!("{}",print_debug_msg);
        };  
        let file_stem = Path::new(file_name).file_stem().expect(print_debug_msg).to_str().expect(print_debug_msg);


        let debug_file_location = file_stem.to_string() + ".xml";

        let mut path = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(debug_file_location)
                    .expect(print_debug_msg);
        let current_token = self.current_token_type().expect("must advance token before trying to debug");
        let current_token_type = current_token.get_variant_name();
        let current_token_value = current_token.get_value_from_token();

        let msg_to_write = format!(
            "<{}>{}</{}>\n",current_token_type.to_lowercase(),current_token_value,current_token_type.to_lowercase()
        );
        write!(path,"{}",msg_to_write).expect("Failed to print debug");

    }

    // terminates program on error
    fn error_occurred_tokenizing<T>(&mut self,msg:T) -> !
    where
    T:std::fmt::Display
    {
    eprintln!("{}",msg);
    std::process::exit(1);
    }




    fn create_token_from_str(&mut self,token_string:&str) -> Token{


        // gets first char to match
        let first_char = token_string.chars().next();


        match first_char {
        // if is keyword return keyword Token
        _ if Keyword::is_keyword(token_string) => Token::KeywordToken(Keyword::from_str(token_string).expect("Ensured is keyword in match")),


        // if first and only char is symbol return symbol
        Some(c) if Symbol::is_symbol_from_char(&c) => Token::SymbolToken(Symbol::from_str(token_string).expect("Ensured is symbol in match")),


        // if all chars are integers return integer Token
        _ if Integer::is_integer(token_string) => Token::IntegerToken(Integer::from_str(token_string).expect("Ensured all chars are integers in match")),

        // if first char is " it means we should have a string Token
        Some('"') => {
            let Ok(key) = StringConstant::from_str(token_string)else{
                let current_file = self.files.get_index_in_paths(self.next_file_to_read_idx - 1);
       
                self.error_occurred_tokenizing(format!("file: {} unexpected string denominator line: {}",current_file.display(),self.line_number));
            };
            Token::StringToken(key)
        
        },

        // ensures that identifier does not begin with digit
        Some(c) if c.is_ascii_digit() => self.error_occurred_tokenizing(format!("identifier should not begin with digit line {}",self.line_number)),

        // if nothing else has to be identifier
        _ => Token::IdentifierToken(Identifier::from_str(token_string)),
}

    }   


    fn peak_char(&mut self) -> Option<char>{

        match self.current_file.fill_buf(){
            Ok(bytes)if !bytes.is_empty() => Some(bytes[0] as char),
            _ => None,
        }


    }




    fn read_char(&mut self) -> char{
    

        let mut buffer = [0u8;1];
        self.current_file.read_exact(&mut buffer).expect("should always ensure chars before hand");

        buffer[0] as char
    }

    fn more_files_to_read(&self) -> bool{
        if  self.next_file_to_read_idx > self.files.len_of() - 1{
            return false
        }
        true
    }


    fn open_next_file(&mut self){
    
        let next_file = self.files.get_index_in_paths(self.next_file_to_read_idx); 
        let Ok(file) = File::open(next_file) else{
            panic!("unexpected error opening file please retry");
        };
        
        let buf_reader = BufReader::new(file);
        self.current_file = buf_reader;

        self.next_file_to_read_idx = self.next_file_to_read_idx + 1;

    }

    pub fn new(files:FilesInPathBuf )-> Result<Self,io::Error>{
        let current_file = files.get_index_in_paths(0); // there should at least be one file

        let file = File::open(current_file)?;
        let buf_reader = BufReader::new(file);


        Ok(JackTokenizer{
            current_token:None,
            files,
            next_file_to_read_idx:1,
            current_file:buf_reader,
            more_tokens_to_read:MoreTokensToRead { more_tokens: true },
            line_number:1,
        })
    }
}
