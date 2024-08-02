#![doc = include_str!("../README.md")]

use std::{fmt, ops::Deref};

use constants::BLACK_LIST;
use error::ErrorKind;

mod constants;
mod error;

pub use crate::error::{Error, Result};

#[derive(Clone, Copy)]
enum CheckResult {
    Success,
    Invalid(ErrorKind),
}

// https://github.com/npm/validate-npm-package-name/blob/main/test/index.js
fn is_url_safe(c: char, starts_with_at: bool) -> CheckResult {
    if c == '/' && !starts_with_at {
        return CheckResult::Invalid(ErrorKind::NotUrlSafe);
    }
    if c == ':' {
        CheckResult::Invalid(ErrorKind::NotUrlSafe)
    } else {
        CheckResult::Success
    }
}

fn is_valid_char(c: char, starts_with_at: bool) -> CheckResult {
    if c.is_uppercase() {
        return CheckResult::Invalid(ErrorKind::ContainsCapitalLetter);
    }

    if c.is_whitespace()
        || c == '~'
        || c == ')'
        || c == '('
        || c == '\''
        || c == '!'
        || c == '*'
    {
        return CheckResult::Invalid(ErrorKind::InvalidCharacter);
    }

    is_url_safe(c, starts_with_at)
}

fn validate(name: &str) -> Result<&str> {
    if name.is_empty() {
        return Err(Error::from(ErrorKind::LessThanZero));
    }

    if name.len() > constants::MAX_PACKAGE_NAME_LENGTH as usize {
        return Err(Error::from(ErrorKind::LongerThanMax));
    }

    if name.starts_with('.') {
        return Err(Error::from(ErrorKind::StartsWithAPeriod));
    }

    if name.starts_with('_') {
        return Err(Error::from(ErrorKind::StartsWithAnUnderscore));
    }

    let starts_with_at = name.starts_with('@');

    for c in name.chars() {
        match is_valid_char(c, starts_with_at) {
            CheckResult::Success => {}
            CheckResult::Invalid(kind) => return Err(Error::from(kind)),
        }
    }

    if BLACK_LIST.contains(name) {
        return Err(Error::from(ErrorKind::InBlackList));
    }

    Ok(name)
}

/// Represents a valid npm package name.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ValidName<'s>(&'s str);

impl<'s> ValidName<'s> {
    /// Converts a string slice into a `ValidName`.
    ///
    /// # Examples
    ///
    /// ```
    /// use valid_npm_name::ValidName;
    ///
    /// let foo = ValidName::parse("foo").unwrap();
    /// assert_eq!("foo", foo.as_str());
    /// ```
    pub fn parse(name: &'s str) -> Result<Self> {
        validate(name).map(Self)
    }

    /// Extracts a string slice containing the entire name.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl Deref for ValidName<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl AsRef<str> for ValidName<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<'s> TryFrom<&'s str> for ValidName<'s> {
    type Error = Error;

    fn try_from(value: &'s str) -> Result<Self> {
        Self::parse(value)
    }
}

impl fmt::Display for ValidName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let foo = ValidName::parse("foo").unwrap();
        assert_eq!("foo", foo.to_string());

        let bar_name = ValidName::parse("bar").unwrap();
        let bar = bar_name.as_ref();
        assert_eq!("bar", bar);

        let baz = ValidName::parse("baz").unwrap();
        assert_eq!("baz", baz.as_str());

        let foo2: ValidName = "foo2".try_into().unwrap();
        assert_eq!("foo2", foo2.as_str());
    }

    #[test]
    fn valid() {
        let cases = vec![
            "some-package",
            "example.com",
            "under_score",
            "period.js",
            "123numeric",
            "@npm/thingy",
            "vite",
            "@vitejs/plugin-react",
            "@napi-rs/canvas",
        ];

        for case in cases {
            assert!(ValidName::parse(case).is_ok());
            assert_eq!(case, ValidName::parse(case).unwrap().as_str());
        }
    }

    impl PartialEq for ErrorKind {
        fn eq(&self, other: &Self) -> bool {
            self == other
        }
    }

    #[test]
    fn invalid() {
        let long_name = "ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyou-";
        let cases = vec![
            ("", ErrorKind::LessThanZero),
            (&long_name, ErrorKind::LongerThanMax),
            ("crazy!", ErrorKind::InvalidCharacter),
            ("@npm-zors/money!time.js", ErrorKind::InvalidCharacter),
            (".start-with-period", ErrorKind::InvalidCharacter),
            ("_start-with-underscore", ErrorKind::InvalidCharacter),
            ("contain:colons", ErrorKind::NotUrlSafe),
            (" leading-space", ErrorKind::InvalidCharacter),
            ("trailing-space ", ErrorKind::InvalidCharacter),
            ("s/l/a/s/h/e/s", ErrorKind::NotUrlSafe),
            ("node_modules", ErrorKind::InBlackList),
            ("favicon.ico", ErrorKind::InBlackList),
            ("http", ErrorKind::InBlackList),
            ("process", ErrorKind::InBlackList),
            ("CAPITAL-LETTERS", ErrorKind::ContainsCapitalLetter),
            ("assert/strict", ErrorKind::InBlackList),
            ("dns/promises", ErrorKind::InBlackList),
            ("fs/promises", ErrorKind::InBlackList),
            ("path/posix", ErrorKind::InBlackList),
            ("path/win32", ErrorKind::InBlackList),
            ("stream/consumers", ErrorKind::InBlackList),
            ("stream/promises", ErrorKind::InBlackList),
            ("stream/web", ErrorKind::InBlackList),
            ("timers/promises", ErrorKind::InBlackList),
            ("util/types", ErrorKind::InBlackList),
        ];

        for (name, _) in cases {
            assert!(ValidName::parse(name).is_err());
        }
    }
}
