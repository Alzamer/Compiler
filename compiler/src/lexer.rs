pub fn scan(file_content: String){
    let letter = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g',
        'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A',
        'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
        'V', 'W', 'X', 'Y', 'Z'
    ];
    let digit = vec!['0', '1', '2', '3', '4', '5', '6',
        '7', '8', '9'
    ];
    let punctuator = vec!['[', ']', '(', ')', '{', '}',
        '=', '<', '>'
    ];
    let keyword = vec!["char", "double", "int", "void",
        "for", "if", "else", "struct", "return"
    ];
    let quotation_mark = vec!['\"'];
    let delimiter = vec![';'];

    let chars = file_content.chars();
    #[derive(Copy)]
    #[derive(Clone)]
    enum State {
        Id,
        Number,
        Punctuator,
        StringLiteral,
        Idle
    }

    let mut current_state = State::Idle;
    let mut buffer = Vec::new();

    for next_char in chars {
        let current_type;

        if letter.contains(&next_char) {
            current_type = "letter";
        } else if digit.contains(&next_char) {
            current_type = "digit";
        } else if punctuator.contains(&next_char) {
            current_type = "punctuator";
        } else if quotation_mark.contains(&next_char){
            current_type = "quotation_mark";
        } else if delimiter.contains(&next_char){
            current_type = "delimiter";
        } else {
            current_type = "space";
        }

        match (current_state, current_type) {
            (State::Idle, "letter") => {
                current_state = State::Id;
                buffer.push(next_char);
            },
            (State::Idle, "digit") => {
                current_state = State::Number;
            },
            (State::Idle, "quotation_mark") => {
                current_state = State::StringLiteral;
            },
            (State::Idle, "punctuator") => {
                current_state = State::Punctuator;
            },
            (State::Id, "space" | "delimiter") => {
                current_state = State::Idle;
                if keyword.contains(&buffer.clone().into_iter().collect::<String>().as_str()){
                    println!("TOKEN KEYWORD {:?}", &buffer.clone().into_iter().collect::<String>().as_str());
                }
                else{
                    println!("TOKEN ID");
                }
                buffer.clear();
            },
            (State::Id, "punctuator") => {
                if keyword.contains(&buffer.clone().into_iter().collect::<String>().as_str()){
                    println!("TOKEN KEYWORD {:?}", &buffer.clone().into_iter().collect::<String>().as_str());
                }
                else{
                    println!("TOKEN ID");
                }
                buffer.clear();
                current_state = State::Idle;
                println!("{:?}", "TOKEN PUNCTUATOR");
            },
            (State::Id, "letter" | "digit") => {
                current_state = State::Id;
            buffer.push(next_char);
            },
            (State::Number, "digit") => {
                current_state = State::Number;
            },
            (State::Number, "space" | "delimiter") => {
                current_state = State::Idle;
                println!("{:?}", "TOKEN NUMBER");
            },
            (State::Number, "punctuator") => {
                println!("{:?}", "TOKEN NUMBER");
                current_state = State::Punctuator;
                println!("{:?}", "TOKEN PUNCTUATOR");                
            },
            (State::StringLiteral, "letter" | "digit" |
                "punctuator" | "space" | "delimiter") => {
                current_state = State::StringLiteral;
            },
            (State::StringLiteral, "quotation_mark") => {
                current_state = State::Idle;
                println!("{:?}", "TOKEN QUOTATION_MARK");
            },
            (State::Punctuator, "space") => {
                current_state = State::Idle;
                println!("{:?}", "TOKEN Punctuator");
            },
            (State::Punctuator, "letter") => {
                current_state = State::Id;
                println!("{:?}", "TOKEN Punctuator");
            },
            (State::Punctuator, "digit") => {
                current_state = State::Number;
                println!("{:?}", "TOKEN Punctuator");
            },
            (State::Punctuator, "punctuator") => {
                current_state = State::Punctuator;
                println!("{:?}", "TOKEN Punctuator");
            },
            _ => current_state = State::Idle,
        }
    }

    return ();
}