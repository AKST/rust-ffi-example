use std::fmt;


#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Email {
    user:   String,
    domain: String,
}

pub type EmailResult<'a> = Result<&'a mut EmailParser, ParseError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseState { Start, UserName, Domain, End }

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char, ParseState),
    InvalidCharacterPush,
	PrematureFinalisation,
}

#[derive(Debug)]
pub struct EmailParser {
    username:  String,
    domain:    Vec<String>,
    state:     ParseState,
    last_char: Option<char>,
}


impl Email {
    pub fn new(username: String, domain: String) -> Email {
        Email { user: username, domain: domain }
    }

    pub fn from_string(line: String) -> Result<Email, ParseError> {
        let mut parser = EmailParser::new();
        let init: EmailResult = Ok(&mut parser);
        return line.chars()
            .fold(init, EmailParser::with_fold)
            .and_then(|b| b.finalise())
    }
}


impl fmt::Display for Email  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}



impl EmailParser {
    fn new() -> EmailParser {
        EmailParser {
            last_char: None,
            username: "".to_owned(),
            domain: "".to_owned(),
            state: ParseState::Start,
        }
    }

    fn push_char<'a>(&'a mut self, character: char) -> EmailResult<'a> {
        self.last_char = Some(character);
        match self.state {
            ParseState::UserName => {
                self.username.push(character);
                Ok(self)
            },
            ParseState::Domain => {
                self.domain.push(character);
                Ok(self)
            },
            _ => Err(ParseError::InvalidCharacterPush)
        }
    }

    fn is_last_char(&self, character: char) -> bool {
        match self.last_char {
            Some(c) => c == character,
            None    => false
        }
    }

    fn unexpected_char(&self, character: char) -> ParseError {
        ParseError::UnexpectedChar(character, self.state)
    }

	fn can_finish(&self) -> bool {
		let is_alphanumeric = match self.last_char {
			Some(c) => c.is_alphanumeric(),
			_______ => false,
		};
		self.state == ParseState::End || (self.state == ParseState::Domain && is_alphanumeric)
	}

    fn finalise(&self) -> Result<Email, ParseError> {
		if self.can_finish() {
			Ok(Email::new(self.username.clone(), self.domain.clone()))
		}
		else {
			Err(ParseError::PrematureFinalisation)
		}
    }

    // probably should a result like for chars
    fn with_fold<'a>(result: EmailResult<'a>, next: char) -> EmailResult<'a> {
        return result.and_then(|parser: &'a mut EmailParser| match parser.state {
            ParseState::Start => match next {
                c if c.is_whitespace() => Ok(parser),
                c if c.is_alphabetic() => {
                    parser.state = ParseState::UserName;
                    parser.push_char(c)
                },
                _ => Err(parser.unexpected_char(next)),
            },
            ParseState::UserName => match next {
                c if c.is_alphanumeric() => parser.push_char(c),
                '@' if parser.is_last_char('.') => Err(parser.unexpected_char('@')),
                '.' => parser.push_char('.'),
                '@' => {
                    parser.state = ParseState::Domain;
                    parser.last_char = Some('@');
                    Ok(parser)
                },
                _ => Err(parser.unexpected_char(next)),
            },
            ParseState::Domain => match next {
                c if c.is_whitespace() && parser.is_last_char('.') => Err(parser.unexpected_char(c)),
                c if c.is_alphanumeric() => parser.push_char(c),
                c if c.is_whitespace() => {
                    parser.state = ParseState::End;
                    parser.last_char = Some('@');
                    Ok(parser)
                },
                '.' => parser.push_char('.'),
                _ => Err(parser.unexpected_char(next)),
            },
            ParseState::End => match next {
                c if c.is_whitespace() => Ok(parser),
                _ => Err(parser.unexpected_char(next))
            }
        } as EmailResult<'a>);
    }
}
