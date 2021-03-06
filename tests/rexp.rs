use rexp::{
    expr::{
        Atom,
        Num,
        // Quoted Constants
        Quote::{
            Quasi,
            Quote,
            //Splice,
            //UnQuote,
        },
        Sexp,
    },
    parse,
};

// Higher level parsing tests
// Sexp

#[test]
fn fails_on_empty_sexp() {
    assert!(parse::sexp("").is_err());
}

// Constants

#[test]
fn int_constant() {
    assert_eq!(
        parse::sexp("345"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Int(345)))))
    );
    assert_eq!(
        parse::sexp("-345"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Int(-345)))))
    );
}

#[test]
fn float_constant() {
    assert_eq!(
        parse::sexp("756.314"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Float(756.314)))))
    );
    assert_eq!(
        parse::sexp("-756.314"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Float(-756.314)))))
    );
}

#[test]
fn symbol_constant() {
    assert_eq!(
        parse::sexp("name"),
        Ok(("", Sexp::Constant(Atom::Symbol("name".to_owned()))))
    );
}

#[test]
fn string_constant() {
    assert_eq!(
        parse::sexp("\"This is a \\\"string\\\"!\""),
        Ok((
            "",
            Sexp::Constant(Atom::String("This is a \"string\"!".to_owned()))
        ))
    );
}

#[test]
fn quoted_int() {
    assert_eq!(
        parse::sexp("'345"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Int(345))))))
        ))
    );
    assert_eq!(
        parse::sexp("'-345"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Int(-345))))))
        ))
    );
    // Quasi
    assert_eq!(
        parse::sexp("`345"),
        Ok((
            "",
            Sexp::Quote(Quasi(Box::new(Sexp::Constant(Atom::Num(Num::Int(345))))))
        ))
    );
}

#[test]
fn cannot_splice_unquote_int() {
    assert!(parse::sexp(",345").is_err());

    assert!(parse::sexp("@345").is_err());
}

#[test]
fn quoted_float() {
    assert_eq!(
        parse::sexp("'756.314"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Float(
                756.314
            ))))))
        ))
    );
    assert_eq!(
        parse::sexp("'-756.314"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Float(
                -756.314
            ))))))
        ))
    );
    // Quasi
    assert_eq!(
        parse::sexp("`756.314"),
        Ok((
            "",
            Sexp::Quote(Quasi(Box::new(Sexp::Constant(Atom::Num(Num::Float(
                756.314
            ))))))
        ))
    );
}

#[test]
fn cannot_splice_or_quote_float() {
    assert!(parse::sexp(",756.314").is_err());

    assert!(parse::sexp("@756.314").is_err());
}

#[test]
fn quoted_string() {
    assert_eq!(
        parse::sexp("'\"this is a quoted string\""),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::String(
                "this is a quoted string".to_owned()
            )))))
        ))
    );
}

#[test]
fn quoted_symbol() {
    assert_eq!(
        parse::sexp("'symbol"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Symbol(
                "symbol".to_owned()
            )))))
        ))
    );
    assert_eq!(
        parse::sexp("'|this symbol has spaces|"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Symbol(
                "this symbol has spaces".to_owned()
            )))))
        ))
    );
}



// Lists

#[test]
fn simple_list_of_ints() {
    assert_eq!(
        parse::sexp("(1 2 3 4 5)"),
        Ok((
            "",
            Sexp::List(
                vec![
                    Sexp::Constant(Atom::Num(Num::Int(1))),
                    Sexp::Constant(Atom::Num(Num::Int(2))),
                    Sexp::Constant(Atom::Num(Num::Int(3))),
                    Sexp::Constant(Atom::Num(Num::Int(4))),
                    Sexp::Constant(Atom::Num(Num::Int(5))),
                ]
            )
        ))
    );
}

#[test]
fn simple_list_of_atoms() {
    assert_eq!(
        parse::sexp("(func \"some message\" 14 56.3 -3)"),
        Ok((
            "",
            Sexp::List(
                vec![
                    Sexp::Constant(Atom::Symbol("func".to_owned())),
                    Sexp::Constant(Atom::String("some message".to_owned())),
                    Sexp::Constant(Atom::Num(Num::Int(14))),
                    Sexp::Constant(Atom::Num(Num::Float(56.3))),
                    Sexp::Constant(Atom::Num(Num::Int(-3))),
                ]
            )
        ))
    );
}

#[test]
fn nested_list() {
    assert_eq!(
        parse::sexp("(lambda (msg) (println msg))"),
        Ok((
            "",
            Sexp::List(
                vec![
                    Sexp::Constant(Atom::Symbol("lambda".to_owned())),
                    Sexp::List(
                        vec![
                            Sexp::Constant(Atom::Symbol("msg".to_owned())),
                        ]
                    ),
                    Sexp::List(
                        vec![
                            Sexp::Constant(Atom::Symbol("println".to_owned())),
                            Sexp::Constant(Atom::Symbol("msg".to_owned())),
                        ]
                    ),
                ]
            )
        ))
    );
}



// Vectors

#[test]
fn empty_vector() {
    assert_eq!(parse::sexp("#()"), Ok(("", Sexp::Vector(vec![]))));
}

#[test]
fn vector_of_nums() {
    assert_eq!(
        parse::sexp("#(14 15 16)"),
        Ok(("", Sexp::Vector(vec![
            Sexp::Constant(Atom::Num(Num::Int(14))),
            Sexp::Constant(Atom::Num(Num::Int(15))),
            Sexp::Constant(Atom::Num(Num::Int(16))),
        ])))
    );
}

#[test]
fn vector_of_vecs_and_lists() {
    assert_eq!(
        parse::sexp("#(#(1 2 3) (this \"is\" a \"test\") #(4 5 6))"),
        Ok(("", Sexp::Vector(vec![
            Sexp::Vector(vec![
                Sexp::Constant(Atom::Num(Num::Int(1))),
                Sexp::Constant(Atom::Num(Num::Int(2))),
                Sexp::Constant(Atom::Num(Num::Int(3))),
            ]),
            Sexp::List(vec![
                Sexp::Constant(Atom::Symbol("this".to_owned())),
                Sexp::Constant(Atom::String("is".to_owned())),
                Sexp::Constant(Atom::Symbol("a".to_owned())),
                Sexp::Constant(Atom::String("test".to_owned())),
            ]),
            Sexp::Vector(vec![
                Sexp::Constant(Atom::Num(Num::Int(4))),
                Sexp::Constant(Atom::Num(Num::Int(5))),
                Sexp::Constant(Atom::Num(Num::Int(6))),
            ]),
        ])))
    );
}



// Quoted Lists, Quoted Symbols, Quoted Quotes, and miscellaneous

#[test]
fn quoted_list() {
    assert_eq!(
        parse::sexp("'()"),
        Ok(("", Sexp::Quote(Quote(Box::new(Sexp::List(vec![]))))))
    );
}
