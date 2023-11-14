#[derive(Copy)]
#[derive(Clone)]
enum State {
    Id,
    Number,
    Punctuator,
    StringLiteral,
    ClosedStringLiteral,
    Idle
}

enum TokenType {
    Keyword,
    Identifier,
    Number,
    Punctuator,
}

struct Token {
    token_type: TokenType,
    value: String
}

struct FSM<'a> {
    current_state: State,
    buffer: Vec<char>,
    tokens: Vec<Token>,
    letter: Vec<char>,
    digit: Vec<char>,
    punctuator: Vec<char>,
    keyword: Vec<&'a str>,
    delimiter: Vec<char>,
    symbol_table: Vec<String>
}

impl FSM<'_> {
    fn execute(&mut self, input: String) -> () {
        let chars = input.chars();
        for next_char in chars {
            let current_type;

            if self.letter.contains(&next_char) {
                current_type = "letter";
            } else if self.digit.contains(&next_char) {
                current_type = "digit";
            } else if self.punctuator.contains(&next_char) {
                current_type = "punctuator";
            } else if self.delimiter.contains(&next_char){
                current_type = "delimiter";
            } else {
                println!("current char {:?}", next_char);
                current_type = "space";
            }

            match (self.current_state, current_type) {
                (State::Idle, "letter") => {
                    self.current_state = State::Id;
                    self.buffer.push(next_char);
                },
                (State::Idle, "digit") => {
                    self.current_state = State::Number;
                    self.buffer.push(next_char);
                },
                (State::Idle, "punctuator") => {
                    self.current_state = State::Punctuator;
                    self.buffer.push(next_char);
                },
                (State::Idle, "quotation_mark") => {
                    self.current_state = State::StringLiteral;
                    self.buffer.push(next_char);
                },
                (State::Id, "letter" | "digit") => {
                    self.buffer.push(next_char);
                },
                (State::Id, "punctuator") => {
                    self.current_state = State::Punctuator;
                    let temp: String = self.buffer.clone().into_iter().collect();
                    let mut temp_type;

                    if self.keyword.contains(&temp.as_str()) {
                        temp_type = TokenType::Keyword;
                        self.tokens.push(Token {
                            token_type: temp_type,
                            value: temp,
                        });
                        println!("PUSHUJE TOPKEN KEYWORD");
                    } else {
                        temp_type = TokenType::Identifier;
                        self.tokens.push(Token {
                            token_type: temp_type,
                            value: temp, // change to symbol table
                        });
                        println!("PUSHUJE TOPKEN IDENTIFIER");
                    }
                    self.buffer.clear();                 
                },
                (State::Id, "space" | "delimiter") => {
                    self.current_state = State::Idle;
                    let temp: String = self.buffer.clone().into_iter().collect();
                    let mut temp_type;

                    if self.keyword.contains(&temp.as_str()) {
                        temp_type = TokenType::Keyword;
                        self.tokens.push(Token {
                            token_type: temp_type,
                            value: temp,
                        });
                        println!("PUSHUJE TOPKEN KEYWORD");
                    } else {
                        temp_type = TokenType::Identifier;
                        self.tokens.push(Token {
                            token_type: temp_type,
                            value: temp, // change to symbol table
                        });
                        println!("PUSHUJE TOPKEN IDENTIFIER");
                    }
                    self.buffer.clear();
                },
                (State::Number, "digit") => {
                    self.buffer.push(next_char);
                },
                (State::Number, "space" | "delimiter") => {
                    self.current_state = State::Idle;
                    let temp: String = self.buffer.clone().into_iter().collect();
                    self.tokens.push(Token {
                        token_type: TokenType::Number,
                        value: temp,
                    });
                    self.buffer.clear();
                    println!("PUSHUJE TOPKEN NUMBER");
                },
                (State::Number, "punctuator") => {
                    self.current_state = State::Punctuator;
                    let temp: String = self.buffer.clone().into_iter().collect();
                    self.tokens.push(Token {
                        token_type: TokenType::Number,
                        value: temp,
                    });
                    self.buffer.clear();
                    self.buffer.push(next_char);
                    println!("PUSHUJE TOPKEN NUMBER");
                },
                (State::Punctuator, "punctuator") => {
                    self.current_state = State::Punctuator;
                    let temp: String = self.buffer.clone().into_iter().collect();
                    self.tokens.push(Token {
                        token_type: TokenType::Punctuator,
                        value: temp,
                    });
                    self.buffer.clear();
                    self.buffer.push(next_char);
                    println!("PUSHUJE TOPKEN Punctuator");
                },
                (State::Punctuator, "space" | "delimiter" | "punctuator"
                    | "digit" | "letter" | "quotation_mark") => {
                    self.current_state = State::Idle;
                    let temp: String = self.buffer.clone().into_iter().collect();
                    self.tokens.push(Token {
                        token_type: TokenType::Punctuator,
                        value: temp,
                    });
                    self.buffer.clear();
                    self.buffer.push(next_char);
                    println!("PUSHUJE TOPKEN PUNCTUATOR");
                },
                _ => self.current_state = State::Idle,
            }
        }
    }

    fn output(&self) {

    }
}

pub fn scan(file_content: String){
    let mut machine = FSM {
        current_state: State::Idle,
        buffer: vec![],
        tokens: vec![],
        letter: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
        'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z'],
        digit: vec!['0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9'],
        punctuator: vec!['[', ']', '(', ')', '{', '}', '=',
        '<', '>', '+', '-', ','],
        keyword: vec!["char", "double", "int", "void","for",
        "if", "else", "struct", "return"],
        delimiter: vec![ ';', '\n'],
        symbol_table: vec![],
    };

    machine.execute(file_content);
    machine.output();

    return ();
}