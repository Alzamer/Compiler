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
        '=', ';'
    ];
    let keyword = vec!["char", "double", "int", "void",
        "for", "if", "else", "struct", "return"
    ];
    let quotation_mark = vec!['\"'];
    let delimiter = vec![';'];

    struct Token {
        name: String,
        value: String
    }

    let mut chars = file_content.chars();

    enum State {
        Id,
        Number,
        OneLetterPunctuator,
        Punctuator,
        StringLiteral,
        Init
    }

    let mut current_state = State::Init;

    while let next_char = chars.next() {
        let mut current_type = "";

        if letter.contains(&next_char.unwrap()) {
            current_type = "letter";
        } else if digit.contains(&next_char.unwrap()) {
            current_type = "digit";
        } else if punctuator.contains(&next_char.unwrap()) {
            current_type = "punctuator";
        } else if quotation_mark.contains(&next_char.unwrap()){
            current_type = "quotation_mark";
        } else if delimiter.contains(&next_char.unwrap()){
            current_type = "delimiter";
        } else {
            current_type = "space";
        }

        match (current_state, current_type) {
            (State::Init, "letter") => {},
            (State::Init, "digit") => {},
            (State::Init, "quotation_mark") => {},
            (State::Init, "punctuator") => {},
            (State::Init, "space") => {},
            (State::Id, "space" | "delimiter") => {},
            (State::Id, "letter" | "digit") => {},
            (State::Number, "digit") => {},
            (State::Number, "space" | "delimiter") => {},
            (State::StringLiteral, "letter" | "digit" |
                "punctuator" | "space" | "delimiter") => {},
            (State::StringLiteral, "quotation_mark") => {},
            (State::OneLetterPunctuator, "punctuator") => {},
            (State::OneLetterPunctuator, "space" | "number" |
                "letter") => {},
            (State::Punctuator, "space" | "number" | "letter") => {},
        }
    }

    return ();
}