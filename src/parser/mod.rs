use core::panic;
use std::{any, vec};
use std::ffi::c_float;
use std::fmt::{Debug, format};

use crate::tokenizer::{self, CanCheckEq, Identifier, Keyword, Symbol, Token, Tokenizer};
use crate::tokenizer::Token::{IdentifierToken,IntegerToken,KeywordToken,StringToken,SymbolToken};





// building blocks
// programming structure

#[derive(Debug)]
pub struct Class{
    class_name:tokenizer::Identifier,
    class_var_decs:Vec<ClassVarDec>,
    subroutine_decs:Vec<SubRoutineDec>,
}


#[derive(Debug)]
enum StaticOrField{
    Static,
    Field,
}

#[derive(Debug)]
pub struct ClassVarDec{
 
        type_of:TypeOf,
        static_or_field:StaticOrField,
        first_var_name:tokenizer::Identifier,
        var_names:Vec<tokenizer::Identifier>
}

#[derive(Debug)]
pub enum TypeOf{
    BuiltIn(tokenizer::Keyword),
    ClassName(tokenizer::Identifier)
}

#[derive(Debug)]
struct Var{
    type_of:TypeOf,
    var_name:tokenizer::Identifier,
}


#[derive(Debug)]
pub enum ConstructorFunctionMethod{

    Constructor,
    Function,
    Method



}




#[derive(Debug)]
pub struct SubRoutineDec{
    return_type:ReturnType,
    constructor_function_or_method:ConstructorFunctionMethod,
    subroutine_name:tokenizer::Identifier,
    parameters:Option<Vec<Parameters>>,
    subroutine_body:Option<SubRoutineBody>,
}


#[derive(Debug)]
pub struct Parameters{
    type_of:Option<TypeOf>,
    var_name:Option<tokenizer::Identifier>,
}


#[derive(Debug)]
pub struct SubRoutineBody{
    var_dec:VarDec,
    statements:Vec<Statements>
}


#[derive(Debug)]
pub struct VarDec{
    type_of:Option<TypeOf>,
    optional_more_vars:Vec<tokenizer::Identifier>
}






#[derive(Debug)]
enum ReturnType{

    Void,
    OtherReturn(tokenizer::Identifier),

}





// different statementS

#[derive(Debug)]
struct IfStatement{
    expression:Expression,
    statements:Vec<Statements>


}

#[derive(Debug)]
struct WhileStatement{
    expression:Expression,
    statements:Vec<Statements>
}


#[derive(Debug)]
struct DoStatement{
    subroutine_call:tokenizer::Identifier
}

#[derive(Debug)]
struct LetStatement{
    expression_equal:Expression,
    index_array:Option<Expression>,
    var_name:tokenizer::Identifier,
}

#[derive(Debug)]
struct ReturnStatement{
    expression:Option<Expression>
}

#[derive(Debug)]
enum Statements{
    
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    Let(Box<LetStatement>),

}


// expressions
#[derive(Debug)]
struct Expression{
    term:Term,
}

#[derive(Debug)]
struct Term{
    term:Vec<tokenizer::Token>,
    expression_list:Option<ExpressionList>,
    expression_idx:Option<Box<Expression>>,
    calling_subroutine:Option<tokenizer::Identifier>    
}

#[derive(Debug)]
struct ExpressionList{
    expressions:Box<Expression>
}




pub trait Parser{
    fn begin_compilation(&mut self);
    fn compile_class(&mut self) -> Option<Class>;
    fn compile_class_var_dec(&mut self) -> Option<ClassVarDec>;
    fn compile_subroutine(&mut self) -> Option<SubRoutineDec>;
    fn compile_subroutine_var_dec(&mut self) -> Option<Vec<VarDec>>;
    fn compile_parameter_list(&mut self) -> Option<Vec<Parameters>>;
    fn compile_subroutine_body(&mut self) -> Option<SubRoutineBody>;
    fn compile_statements(&self);
    fn compile_if_statements(&self);
    fn compile_while_statements(&self);
    fn compile_type_of(&mut self) -> Option<TypeOf>;
    // helpers

    fn get_var_name(&mut self) -> Option<tokenizer::Identifier>;

}

#[allow(dead_code)]
pub struct JackParser<T:tokenizer::Tokenizer>{
    tokenizer:T,
    statements:Vec<Statements>,
    errors:Vec<String>
}



impl JackParser<tokenizer::JackTokenizer>{

    pub fn report_error(&mut self,msg:String){
        self.errors.push(msg);
    }
    pub fn new(tokenizer:tokenizer::JackTokenizer) -> Self{
        JackParser { tokenizer, statements: vec![], errors: vec![] }
    }


    pub fn expected_token<U:CanCheckEq + Debug>(&mut self,token_expected:U,add_to_err_msg:Option<&str>,report_error:bool) -> bool{

        let Some(current_token) = self.tokenizer.current_token_type() else {
        panic!("Called expect_token with no token read");
        };  
        if !token_expected.matches_token(current_token){
            let extra = add_to_err_msg.unwrap_or("");

            if report_error{
            self.report_error(format!("unexpected token found expected {:?},\n Line: {},\n {}",token_expected,self.tokenizer.get_line_number(),extra));
            }
            return false
        }
        true
        
    }
    fn format_unexpect_token_err_msg(& self,token:&Token) -> String{

        format!("unexpected token found {:?}",token)

    }

    fn get_type_of(&mut self) -> Option<TypeOf>{

        match self.tokenizer.current_token_type(){

            Some(token) => {
                // token should be of type class name or of the different keyword tokens listed.
                match token{
                    IdentifierToken(class_name) => {
                        Some(TypeOf::ClassName(class_name.clone()))   
                    }
                    KeywordToken(Keyword::Int) => {
                        Some(TypeOf::BuiltIn(Keyword::Int))
                    }
                    KeywordToken(Keyword::Char) => {
                        Some(TypeOf::BuiltIn(Keyword::Char))
                    }
                    KeywordToken(Keyword::Boolean) => {
                        Some(TypeOf::BuiltIn(Keyword::Boolean))
                    }
                    unexpected_token => {
                    self.report_error(self.format_unexpect_token_err_msg(unexpected_token));
                    return None;
                    }
                }
            }
            None => {
                self.report_error(format!("Expected more tokens line {}",self.tokenizer.get_line_number()));
                return None;
            }
        }

    }

}






impl Parser for JackParser<tokenizer::JackTokenizer> {
    
    
    fn begin_compilation(&mut self) {
        
        loop{

            if !self.tokenizer.has_more_tokens(){
                break;
            }

            self.tokenizer.advance_token();
            let Some(current_token) = self.tokenizer.current_token_type() else{
                continue;
            };
            

            


            match current_token {
            
            KeywordToken(Keyword::Class) => {
                self.compile_class();
            },

            KeywordToken(_) => {
                self.report_error(format!("Error file did not begin with keyword class {}",self.tokenizer.get_line_number()));
                self.compile_class();
            },

            // unknown as began with no class keyword
            SymbolToken(_) => {
                self.report_error(format!("Error file did not begin with keyword class line: {}",self.tokenizer.get_line_number()));

                self.compile_class();

            },

            IntegerToken(_) => {
                self.report_error(format!("Error file did not begin with keyword class line: {}\n ",self.tokenizer.get_line_number()));

                self.compile_class();

            },
            
            StringToken(_) => {
                self.report_error(format!("Error file did not begin with keyword class line: {}\n ",self.tokenizer.get_line_number()));

                self.compile_class();
                
            },

            IdentifierToken(_) => {
                self.report_error(format!("Error file did not begin with keyword class line: {}\n ",self.tokenizer.get_line_number()));

                self.compile_class();

            },
                    
            }
            self.tokenizer.skip_file();

        }

    }


    // assumes current token type is of Keyword::Class
    fn compile_class(&mut self) -> Option<Class> {

            // ensure more tokens in file
            if !self.tokenizer.has_more_tokens(){
                self.report_error(String::from("unexpected only class keyword in file"));
                return None
            }

            // ensure class name is with the same name as file name
            self.tokenizer.advance_token();


            self.expected_token(tokenizer::Identifier::SequenceOfChars(self.tokenizer.file_stem_current()),Some("Class Name must be same as file name"),true);







            self.tokenizer.advance_token();
            self.expected_token(Symbol::LeftCurlyBracket,None,true);
            



            let mut class_var_decs:Vec<ClassVarDec> = Vec::new();

            loop{
                let Some(var_dec)  = self.compile_class_var_dec()else{
                    break;
                };
                class_var_decs.push(var_dec);
            }


            let mut subroutine_decs:Vec<SubRoutineDec> = Vec::new();
            loop{
            let Some(subroutine_dec) = self.compile_subroutine() else{
                break
            };
                subroutine_decs.push(subroutine_dec);
            }




            println!("subroutine: {:?}",subroutine_decs);

            let class = Some(Class { class_name:tokenizer::Identifier::SequenceOfChars(self.tokenizer.file_stem_current()), class_var_decs, subroutine_decs });


            println!("class {:?}",class);
            class

    }


    fn compile_subroutine(&mut self) -> Option<SubRoutineDec>{

        let is_constructor = self.expected_token(tokenizer::Keyword::Constructor, None,false);

        let is_function = self.expected_token(tokenizer::Keyword::Function, None, false);

        let is_method = self.expected_token(tokenizer::Keyword::Method, None, false);

        let constructor_function_or_method:ConstructorFunctionMethod;
        if !is_constructor && !is_function && !is_method{
            return None;
        }else{
            if is_constructor{
                constructor_function_or_method = ConstructorFunctionMethod::Constructor;
            }
            else if is_function{
                constructor_function_or_method = ConstructorFunctionMethod::Function;
            }
            else{
                constructor_function_or_method = ConstructorFunctionMethod::Method;
            }
        }
        
        self.tokenizer.advance_token();

        let void_type = self.expected_token(Keyword::Void, None, false);
        let return_type:ReturnType;


        if void_type{
            return_type = ReturnType::Void;
        }else{
            match self.tokenizer.current_token_type(){

                Some(token) => {
                    match token{
                        IdentifierToken(type_dec) => {
                            return_type = ReturnType::OtherReturn(type_dec.clone());
                            if is_constructor{
                                if type_dec != &tokenizer::Identifier::SequenceOfChars(self.tokenizer.file_stem_current()){
                                    self.report_error(String::from("Expected return type of constructor to match file name"));
                                    return None;
                                }
                            }
                        },
                        
                        unexpected_token => {
                        self.report_error(self.format_unexpect_token_err_msg(unexpected_token));
                        return None;
                        }
                    }
                }

                None => {
                    self.report_error(String::from("unexpected end of file"));
                    return None;
                },
            } 


        }

        self.tokenizer.advance_token();

        let current_token = self.tokenizer.current_token_type();

        let subroutine_name = match current_token{
            Some(token) => {
                match token{

                    IdentifierToken(dec) => dec.clone(),
                    _ => tokenizer::Identifier::SequenceOfChars(String::from("")),
                }
            }
            None => {
                self.report_error(String::from("unexpected end of file"));
                return None;
            }
        };


        let parameters = self.compile_parameter_list();
        println!("Parameters {:?}",parameters);

        let subroutine_body = self.compile_subroutine_body();
        // println!("After params, {:?}",self.tokenizer.current_token_type());

        Some(SubRoutineDec { return_type, constructor_function_or_method, subroutine_name, parameters, subroutine_body: None })


    }

    // compiles var declarations for a class for the jack programming language.
    fn compile_class_var_dec(&mut self) -> Option<ClassVarDec>{
        
        // advance token
        self.tokenizer.advance_token();
        let static_or_field:StaticOrField;
        

        // expectations
        let is_static = self.expected_token(Keyword::Static,None,false);
        let is_field = self.expected_token(Keyword::Field,None,false);

        if !is_field && !is_static{
            // if neither token no class var declarations
            return None;
        }else{
            // else assignments
            if is_field{
                static_or_field  = StaticOrField::Field;
            }else{
                static_or_field = StaticOrField::Static;
            }
        }
        self.tokenizer.advance_token();
        let Some(type_of) = self.get_type_of()else{
            return None;
        };
        

        self.tokenizer.advance_token();
        // if no var name then user has not type one
        // should at least be on var name
        let Some(first_var_name) = self.get_var_name() else{
            self.report_error(String::from("Expected token"));
            return None;
        };


        // we build up an array for other var names that may be defined after the first one
        let mut var_names:Vec<tokenizer::Identifier> = Vec::new();
        loop{

            
            self.tokenizer.advance_token();
            // expect comma
            if self.expected_token(Symbol::Comma,None,false) {

                self.tokenizer.advance_token();
                // if comma we ensure there is an identifier and if not push and return
                let Some(var) = self.get_var_name() else{
                    self.report_error(String::from("unexpected , with no proceeding var name"));
                    return Some(ClassVarDec { type_of, static_or_field, first_var_name, var_names })
                };
                var_names.push(var);
                continue;
            }

            self.expected_token(Symbol::SemiColon, Some("expected ; SEMICOLON"),true);

            break;
        }


        Some(ClassVarDec { type_of, static_or_field, first_var_name, var_names})
       

    }




    
    fn compile_parameter_list(&mut self)-> Option<Vec<Parameters>> {

       
        self.tokenizer.advance_token();
        self.expected_token(tokenizer::Symbol::LeftParentis, None, true);

        let mut parameters:Vec<Parameters> = Vec::new();
        
        loop{
            let mut parameter:Parameters = Parameters { type_of: None, var_name: None };
            self.tokenizer.advance_token();
              

            let mut current_token = self.tokenizer.current_token_type().expect("Expected token not end of file");

            if *current_token == tokenizer::Token::SymbolToken(Symbol::Comma){
                self.tokenizer.advance_token();
                current_token = self.tokenizer.current_token_type().expect("Expected token not end of file");
            } 
           


            
            if current_token == &tokenizer::Token::SymbolToken(Symbol::RightParentis){
                break
            }

            let type_of = self.compile_type_of();


            self.tokenizer.advance_token();
            current_token = self.tokenizer.current_token_type().expect("Expected token not end of file");

             match current_token {
                SymbolToken(sym) => {
                    if *sym == Symbol::RightParentis{
                        self.report_error(String::from("Expected identifier for type"));
                        break;
                    }
                }
                
                IdentifierToken(identifier) => {
                    parameter = Parameters { type_of, var_name: Some(identifier.clone()) };
                }

                unexpected_token => {
                    self.report_error(self.format_unexpect_token_err_msg(unexpected_token));

                    return None;
                }
            }
            parameters.push(parameter);

        }
        

        Some(parameters)

    }


    fn compile_type_of(&mut self) -> Option<TypeOf>{
             let current_token = self.tokenizer.current_token_type().expect("Expected token not end of file");
               match current_token {
    
                KeywordToken(keyword) => {

                    if *keyword == Keyword::Int{
                        return Some(TypeOf::BuiltIn(Keyword::Int))
                    }
                    if *keyword == Keyword::Char{
                        return Some(TypeOf::BuiltIn(Keyword::Char));
                    }

                    if *keyword == Keyword::Boolean{
                        return Some(TypeOf::BuiltIn(Keyword::Boolean));
                    }

                    return None
                    

                }
                IdentifierToken(type_name) => {

                    return Some(TypeOf::ClassName(type_name.clone()));

                }
                unexpected_token => {
                    self.report_error(self.format_unexpect_token_err_msg(unexpected_token));
                    return None;
                }
            }

    }


    fn compile_subroutine_body(&mut self) -> Option<SubRoutineBody> {

        // expect { bracket for beginning of body
        self.tokenizer.advance_token();
        self.expected_token(Symbol::LeftCurlyBracket, None,false);

        self.tokenizer.advance_token();
        

    //    while let Some(current_token ) = self.tokenizer.current_token_type(){

            

            let var_decs = self.compile_subroutine_var_dec();

            println!("Var dec look here {:?}",var_decs);

    //    }

        None

    }


    fn compile_subroutine_var_dec(&mut self) -> Option<Vec<VarDec>> {
            
            println!("current token vec dec{:?}",self.tokenizer.current_token_type());

            let mut var_decs:Vec<VarDec> = Vec::new();
            loop{
            let mut var_dec:VarDec = VarDec { type_of: None, optional_more_vars:Vec::new()  };
            if !self.expected_token(Keyword::Var,None, false){
                break
            }
            self.tokenizer.advance_token();
            let Some(type_of) = self.compile_type_of()else{
                self.report_error(String::from("expected type after var keyword"));
                return None
            };
            var_dec.type_of = Some(type_of);

            loop{
            self.tokenizer.advance_token();

            match self.tokenizer.current_token_type(){

                Some(token) => {
                    match token{
                        IdentifierToken(var_name) => {
                            var_dec.optional_more_vars.push(var_name.clone());
                        }
                        SymbolToken(symbol) => {
                            if symbol == &Symbol::Comma{
                                continue;
                            }
                            if symbol == &Symbol::SemiColon{
                                // parse through to getting next token as var
                                self.tokenizer.advance_token();
                                break;
                            }
                            self.report_error(format!("unexpected symbol found {:?}",symbol));
                        }

                        unexpected_token => {
                            self.report_error(self.format_unexpect_token_err_msg(unexpected_token));
                            return None;
                        }
                    }

                }
                None => {
                    self.report_error(String::from("Expected identifier got EOF"));
                    
                    return None
                }

            }
            }

            var_decs.push(var_dec);
            
            }
                
            Some(var_decs)
    }

    fn compile_statements(&self) {
        
    }

    fn compile_if_statements(&self) {
        
    }
    fn compile_while_statements(&self) {
        
    }





    // helpers


    fn get_var_name(&mut self) -> Option<tokenizer::Identifier>{

         match self.tokenizer.current_token_type(){
            Some(token) => {
                match token {

                    // needs to identifier
                
                IdentifierToken(var_name) => {
                    return Some(var_name.clone())
                }   

                unexpected_token => {
                self.report_error(self.format_unexpect_token_err_msg(unexpected_token));
                   
                return None 
                }
                }


            }

            None => {
                self.report_error(String::from("Expected more tokens"));
                return None
            }

        }


    }





}





