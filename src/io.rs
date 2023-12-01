use log::trace;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::{fmt, io};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Source {
    #[default]
    Stdin,
    File(PathBuf),
}

impl Source {
    pub fn open(&self) -> io::Result<Box<dyn io::BufRead>> {
        match self {
            Source::Stdin => Ok(Box::new(io::stdin().lock())),
            Source::File(path) => Ok(Box::new(io::BufReader::new(std::fs::File::open(path)?))),
        }
    }

    pub fn read_string(&self) -> io::Result<String> {
        trace!("Reading from {}", self);
        let mut reader = self.open()?;
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        trace!("Read {} bytes", buffer.len());
        Ok(buffer)
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Source::Stdin => write!(f, "<stdin>"),
            Source::File(path) => write!(f, "{}", path.display()),
        }
    }
}

impl TryFrom<&str> for Source {
    type Error = io::Error;

    fn try_from(s: &str) -> Result<Self, io::Error> {
        if s == "-" {
            Ok(Source::Stdin)
        } else {
            Ok(Source::File(PathBuf::from(s).canonicalize()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_default() {
        let source = Source::default();
        assert_eq!(source, Source::Stdin);
    }

    #[test]
    fn test_source_from_stdin() {
        let source = Source::try_from("-").unwrap();
        assert_eq!(source, Source::Stdin);
    }

    #[test]
    fn test_source_from_path() {
        let source = Source::try_from("Cargo.toml").unwrap();
        assert_eq!(
            source,
            Source::File(PathBuf::from("Cargo.toml").canonicalize().unwrap())
        );
    }
}
