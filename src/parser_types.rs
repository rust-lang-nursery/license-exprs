use std::fmt;

pub struct Disjunction<'a> {
    pub head: Conjunction<'a>,
    pub tail: Option<Box<Disjunction<'a>>>,
}

impl<'a> fmt::Display for Disjunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.head.fmt(f)?;
        if let Some(ref tail) = self.tail {
            write!(f, " OR {}", tail)?;
        }
        Ok(())
    }
}

pub struct Conjunction<'a> {
    pub head: Term<'a>,
    pub tail: Option<Box<Conjunction<'a>>>,
}

impl<'a> fmt::Display for Conjunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.head.fmt(f)?;
        if let Some(ref tail) = self.tail {
            write!(f, " AND {}", tail)?;
        }
        Ok(())
    }
}

pub enum Term<'a> {
    License(License<'a>),
    Bracketed(Box<Disjunction<'a>>),
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Term::License(l) => l.fmt(f),
            Term::Bracketed(b) => write!(f, "({})", b),
        }
    }
}

pub struct License<'a> {
    pub id: LicenseId<'a>,
    pub exception: Option<Exception<'a>>,
    pub or_later: bool,
}

impl<'a> fmt::Display for License<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.id.fmt(f)?;
        if self.or_later {
            write!(f, "+")?;
        }
        if let Some(ref exe) = self.exception {
            write!(f, " WITH {}", exe)?;
        }
        Ok(())
    }
}

pub struct Exception<'a>(pub &'a str);

impl<'a> fmt::Display for Exception<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

pub enum LicenseId<'a> {
    SPDX(&'a str),
    Other(Option<&'a str>, &'a str),
}

impl<'a> fmt::Display for LicenseId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            LicenseId::SPDX(s) => s.fmt(f),
            LicenseId::Other(Some(d), l) => write!(f, "DocumentRef-{}:LicenseRef-{}", d, l),
            LicenseId::Other(None, l) => write!(f, "LicenseRef-{}", l),
        }
    }
}
