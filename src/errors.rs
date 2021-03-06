// Copyright 2018 the Deno authors. All rights reserved. MIT license.
use hyper;
pub use msg::ErrorKind;
use std;
use std::fmt;
use std::io;
use url;

pub type DenoResult<T> = std::result::Result<T, DenoError>;

#[derive(Debug)]
pub struct DenoError {
  repr: Repr,
}

#[derive(Debug)]
enum Repr {
  // Simple(ErrorKind),
  IoErr(io::Error),
  UrlErr(url::ParseError),
  HyperErr(hyper::Error),
}

impl DenoError {
  pub fn kind(&self) -> ErrorKind {
    match self.repr {
      // Repr::Simple(kind) => kind,
      Repr::IoErr(ref err) => {
        use std::io::ErrorKind::*;
        match err.kind() {
          NotFound => ErrorKind::NotFound,
          PermissionDenied => ErrorKind::PermissionDenied,
          ConnectionRefused => ErrorKind::ConnectionRefused,
          ConnectionReset => ErrorKind::ConnectionReset,
          ConnectionAborted => ErrorKind::ConnectionAborted,
          NotConnected => ErrorKind::NotConnected,
          AddrInUse => ErrorKind::AddrInUse,
          AddrNotAvailable => ErrorKind::AddrNotAvailable,
          BrokenPipe => ErrorKind::BrokenPipe,
          AlreadyExists => ErrorKind::AlreadyExists,
          WouldBlock => ErrorKind::WouldBlock,
          InvalidInput => ErrorKind::InvalidInput,
          InvalidData => ErrorKind::InvalidData,
          TimedOut => ErrorKind::TimedOut,
          Interrupted => ErrorKind::Interrupted,
          WriteZero => ErrorKind::WriteZero,
          Other => ErrorKind::Other,
          UnexpectedEof => ErrorKind::UnexpectedEof,
          _ => unreachable!(),
        }
      }
      Repr::UrlErr(ref err) => {
        use url::ParseError::*;
        match err {
          EmptyHost => ErrorKind::EmptyHost,
          IdnaError => ErrorKind::IdnaError,
          InvalidPort => ErrorKind::InvalidPort,
          InvalidIpv4Address => ErrorKind::InvalidIpv4Address,
          InvalidIpv6Address => ErrorKind::InvalidIpv6Address,
          InvalidDomainCharacter => ErrorKind::InvalidDomainCharacter,
          RelativeUrlWithoutBase => ErrorKind::RelativeUrlWithoutBase,
          RelativeUrlWithCannotBeABaseBase => {
            ErrorKind::RelativeUrlWithCannotBeABaseBase
          }
          SetHostOnCannotBeABaseUrl => ErrorKind::SetHostOnCannotBeABaseUrl,
          Overflow => ErrorKind::Overflow,
        }
      }
      Repr::HyperErr(ref err) => {
        // For some reason hyper::errors::Kind is private.
        if err.is_parse() {
          ErrorKind::HttpParse
        } else if err.is_user() {
          ErrorKind::HttpUser
        } else if err.is_canceled() {
          ErrorKind::HttpCanceled
        } else if err.is_closed() {
          ErrorKind::HttpClosed
        } else {
          ErrorKind::HttpOther
        }
      }
    }
  }
}

impl fmt::Display for DenoError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.repr {
      Repr::IoErr(ref err) => err.fmt(f),
      Repr::UrlErr(ref err) => err.fmt(f),
      Repr::HyperErr(ref err) => err.fmt(f),
      // Repr::Simple(..) => Ok(()),
    }
  }
}

impl std::error::Error for DenoError {
  fn description(&self) -> &str {
    match self.repr {
      Repr::IoErr(ref err) => err.description(),
      Repr::UrlErr(ref err) => err.description(),
      Repr::HyperErr(ref err) => err.description(),
      // Repr::Simple(..) => "FIXME",
    }
  }

  fn cause(&self) -> Option<&std::error::Error> {
    match self.repr {
      Repr::IoErr(ref err) => Some(err),
      Repr::UrlErr(ref err) => Some(err),
      Repr::HyperErr(ref err) => Some(err),
      // Repr::Simple(..) => None,
    }
  }
}

impl From<io::Error> for DenoError {
  #[inline]
  fn from(err: io::Error) -> DenoError {
    DenoError {
      repr: Repr::IoErr(err),
    }
  }
}

impl From<url::ParseError> for DenoError {
  #[inline]
  fn from(err: url::ParseError) -> DenoError {
    DenoError {
      repr: Repr::UrlErr(err),
    }
  }
}

impl From<hyper::Error> for DenoError {
  #[inline]
  fn from(err: hyper::Error) -> DenoError {
    DenoError {
      repr: Repr::HyperErr(err),
    }
  }
}
