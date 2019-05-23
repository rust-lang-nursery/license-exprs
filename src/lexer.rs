use spdx;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    LicenseId(&'a str),
    ExceptionId(&'a str),
    LicenseRef(Option<&'a str>, &'a str),
    Plus,
    OpenParen,
    CloseParen,
    With,
    And,
    Or,
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN")
    }
}

impl<'a> Token<'a> {
    fn len(&self) -> usize {
        match self {
            Token::LicenseId(s) => s.len(),
            Token::ExceptionId(e) => e.len(),
            Token::LicenseRef(None, l) => "LicenseRef-".len() + l.len(),
            Token::LicenseRef(Some(d), l) => {
                "DocumentRef-".len() + d.len() + ":LicenseRef-".len() + l.len()
            }
            Token::With => 4,
            Token::And => 3,
            Token::Or => 2,
            Token::Plus | Token::OpenParen | Token::CloseParen => 1,
        }
    }
}

pub struct Lexer<'a> {
    inner: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        Lexer {
            inner: text,
            offset: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(usize, Token<'a>, usize), failure::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        lazy_static! {
            static ref TEXTTOKEN: regex::Regex = regex::Regex::new(r"^[-a-zA-Z0-9.:]+").unwrap();
            static ref IDSTRING: regex::Regex = regex::Regex::new(r"^[-a-zA-Z0-9.]+").unwrap();
            static ref DOCREFLICREF: regex::Regex =
                regex::Regex::new(r"^DocumentRef-([-a-zA-Z0-9.]+):LicenseRef-([-a-zA-Z0-9.]+)")
                    .unwrap();
            static ref LICREF: regex::Regex =
                regex::Regex::new(r"^LicenseRef-([-a-zA-Z0-9.]+)").unwrap();
        }

        // Jump over any whitespace, updating `self.inner` and `self.offset` appropriately
        let white_len = match self.inner.find(|c: char| !c.is_whitespace()) {
            Some(idx) => idx,
            None => self.inner.len(),
        };
        self.inner = &self.inner[white_len..];
        self.offset += white_len;

        match self.inner.chars().next() {
            None => None,
            Some('+') => Some(Ok(Token::Plus)),
            Some('(') => Some(Ok(Token::OpenParen)),
            Some(')') => Some(Ok(Token::CloseParen)),
            _ => match TEXTTOKEN.find(self.inner) {
                None => Some(Err(failure::err_msg("Hello"))),
                Some(m) => {
                    if m.as_str() == "WITH" {
                        Some(Ok(Token::With))
                    } else if m.as_str() == "AND" {
                        Some(Ok(Token::And))
                    } else if m.as_str() == "OR" {
                        Some(Ok(Token::Or))
                    } else if spdx::LICENSES.binary_search(&m.as_str()).is_ok() {
                        Some(Ok(Token::LicenseId(m.as_str())))
                    } else if spdx::EXCEPTIONS.binary_search(&m.as_str()).is_ok() {
                        Some(Ok(Token::ExceptionId(m.as_str())))
                    } else {
                        if let Some(c) = DOCREFLICREF.captures(m.as_str()) {
                            Some(Ok(Token::LicenseRef(
                                Some(c.get(1).unwrap().as_str()),
                                c.get(2).unwrap().as_str(),
                            )))
                        } else if let Some(c) = LICREF.captures(m.as_str()) {
                            Some(Ok(Token::LicenseRef(None, c.get(1).unwrap().as_str())))
                        } else {
                            Some(Err(failure::err_msg("Hello")))
                        }
                    }
                }
            },
        }
        .map(|res| {
            res.map(|tok| {
                let len = tok.len();
                let start = self.offset;
                self.inner = &self.inner[len..];
                self.offset += len;
                (start, tok, start + len)
            })
        })
    }
}

#[test]
fn lex_all_the_things() {
    let text = "MIT OR + () Apache-2.0 WITH AND LicenseRef-World Classpath-exception-2.0 DocumentRef-Test:LicenseRef-Hello";
    let mut lexer = Lexer::new(text);
    assert_eq!(lexer.next(), Some(Token::LicenseId("MIT")));
    assert_eq!(lexer.next(), Some(Token::Or));
    assert_eq!(lexer.next(), Some(Token::Plus));
    assert_eq!(lexer.next(), Some(Token::OpenParen));
    assert_eq!(lexer.next(), Some(Token::CloseParen));
    assert_eq!(lexer.next(), Some(Token::LicenseId("Apache-2.0")));
    assert_eq!(lexer.next(), Some(Token::With));
    assert_eq!(lexer.next(), Some(Token::And));
    assert_eq!(lexer.next(), Some(Token::LicenseRef(None, "World")));
    assert_eq!(
        lexer.next(),
        Some(Token::ExceptionId("Classpath-exception-2.0"))
    );
    assert_eq!(lexer.next(), Some(Token::LicenseRef(Some("Test"), "Hello")));
    assert_eq!(lexer.next(), None);
}
