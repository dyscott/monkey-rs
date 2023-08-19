use super::*;

#[test]
fn test_next_token_basic() {
    let input = String::from("=+(){},;");

    let tests = vec![
        (token!(=), "="),
        (token!(+), "+"),
        (token!('('), "("),
        (token!(')'), ")"),
        (token!('{'), "{"),
        (token!('}'), "}"),
        (token!(,), ","),
        (token!(;), ";"),
        (token!(EOF), "EOF"),
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let token = lexer.next_token();
        assert_eq!(token, expect.0);
        assert_eq!(token.to_string(), expect.1.to_string());
    }
}

#[test]
fn test_next_token_adv() {
    let input = String::from(
        "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
    ",
    );

    let tests = vec![
        (token!(LET), "let"),
        (token!(IDENT("five")), "five"),
        (token!(=), "="),
        (token!(INT(5)), "5"),
        (token!(;), ";"),
        (token!(LET), "let"),
        (token!(IDENT("ten")), "ten"),
        (token!(=), "="),
        (token!(INT(10)), "10"),
        (token!(;), ";"),
        (token!(LET), "let"),
        (token!(IDENT("add")), "add"),
        (token!(=), "="),
        (token!(FUNCTION), "fn"),
        (token!('('), "("),
        (token!(IDENT("x")), "x"),
        (token!(,), ","),
        (token!(IDENT("y")), "y"),
        (token!(')'), ")"),
        (token!('{'), "{"),
        (token!(IDENT("x")), "x"),
        (token!(+), "+"),
        (token!(IDENT("y")), "y"),
        (token!(;), ";"),
        (token!('}'), "}"),
        (token!(;), ";"),
        (token!(LET), "let"),
        (token!(IDENT("result")), "result"),
        (token!(=), "="),
        (token!(IDENT("add")), "add"),
        (token!('('), "("),
        (token!(IDENT("five")), "five"),
        (token!(,), ","),
        (token!(IDENT("ten")), "ten"),
        (token!(')'), ")"),
        (token!(;), ";"),
        (token!(EOF), "EOF"),
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let token = lexer.next_token();
        assert_eq!(token, expect.0);
        assert_eq!(token.to_string(), expect.1.to_string());
    }
}

#[test]
fn test_next_token_operators() {
    let input = String::from(
        "
        !-/*5;
        5 < 10 > 5;
    ",
    );

    let tests = vec![
        (token!(!), "!"),
        (token!(-), "-"),
        (token!(/), "/"),
        (token!(*), "*"),
        (token!(INT(5)), "5"),
        (token!(;), ";"),
        (token!(INT(5)), "5"),
        (token!(<), "<"),
        (token!(INT(10)), "10"),
        (token!(>), ">"),
        (token!(INT(5)), "5"),
        (token!(;), ";"),
        (token!(EOF), "EOF"),
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let token = lexer.next_token();
        assert_eq!(token, expect.0);
        assert_eq!(token.to_string(), expect.1.to_string());
    }
}

#[test]
fn test_next_token_conditions() {
    let input = String::from(
        "
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
    ",
    );

    let tests = vec![
        (token!(IF), "if"),
        (token!('('), "("),
        (token!(INT(5)), "5"),
        (token!(<), "<"),
        (token!(INT(10)), "10"),
        (token!(')'), ")"),
        (token!('{'), "{"),
        (token!(RETURN), "return"),
        (token!(TRUE), "true"),
        (token!(;), ";"),
        (token!('}'), "}"),
        (token!(ELSE), "else"),
        (token!('{'), "{"),
        (token!(RETURN), "return"),
        (token!(FALSE), "false"),
        (token!(;), ";"),
        (token!('}'), "}"),
        (token!(EOF), "EOF"),
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let token = lexer.next_token();
        assert_eq!(token, expect.0);
        assert_eq!(token.to_string(), expect.1.to_string());
    }
}

#[test]
fn test_next_token_equality() {
    let input = String::from(
        "
        10 == 10;
        10 != 9;
    ",
    );

    let tests = vec![
        (token!(INT(10)), "10"),
        (token!(==), "=="),
        (token!(INT(10)), "10"),
        (token!(;), ";"),
        (token!(INT(10)), "10"),
        (token!(!=), "!="),
        (token!(INT(9)), "9"),
        (token!(;), ";"),
        (token!(EOF), "EOF"),
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let token = lexer.next_token();
        assert_eq!(token, expect.0);
        assert_eq!(token.to_string(), expect.1.to_string());
    }
}

#[test]
fn test_next_token_string() {
    let input = String::from(
        "
        \"foobar\"
        \"foo bar\"
        \"\\\"foo\\\"bar\"
    ",
    );

    let tests = vec![
        (token!(STRING("foobar")), "foobar"),
        (token!(STRING("foo bar")), "foo bar"),
        (token!(STRING("\"foo\"bar")), "\"foo\"bar"),
        (token!(EOF), "EOF"),
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let token = lexer.next_token();
        assert_eq!(token, expect.0);
        assert_eq!(token.to_string(), expect.1.to_string());
    }
}
