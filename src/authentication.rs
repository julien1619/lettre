// Copyright 2014 Alexis Mousset. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides authentication mecanisms

use serialize::base64::{self, ToBase64, FromBase64};
use serialize::hex::ToHex;
use crypto::hmac::Hmac;
use crypto::md5::Md5;
use crypto::mac::Mac;

use NUL;
use error::Error;
use extension::Extension;

/// Represents an authentication mecanism
pub trait Mecanism {
    /// Returns the matching `Extension`
    fn extension() -> Extension;
    /// Returns the initial response support
    fn supports_initial_response() -> bool;
    /// Returns the response
    fn response(username: &str, password: &str, challenge: Option<&str>) -> Result<String, Error>;
}

/// PLAIN authentication mecanism
/// RFC 4616: https://tools.ietf.org/html/rfc4616
#[derive(Copy,Clone)]
pub struct Plain;

impl Mecanism for Plain {
    fn extension() -> Extension {
        Extension::PlainAuthentication
    }

    fn supports_initial_response() -> bool {
        true
    }

    fn response(username: &str, password: &str, challenge: Option<&str>) -> Result<String, Error> {
        match challenge {
            Some(_) => Err(Error::ClientError("This mecanism does not expect a challenge")),
            None => Ok(format!("{}{}{}{}", NUL, username, NUL, password).as_bytes().to_base64(base64::STANDARD)),
        }
    }
}

/// CRAM-MD5 authentication mecanism
/// RFC 2195: https://tools.ietf.org/html/rfc2195
#[derive(Copy,Clone)]
pub struct CramMd5;

impl Mecanism for CramMd5 {
    fn extension() -> Extension {
        Extension::CramMd5Authentication
    }

    fn supports_initial_response() -> bool {
        false
    }

    fn response(username: &str, password: &str, challenge: Option<&str>) -> Result<String, Error> {
        let encoded_challenge = match challenge {
            Some(challenge) => challenge,
            None => return Err(Error::ClientError("This mecanism does expect a challenge")),
        };

        let decoded_challenge = match encoded_challenge.from_base64() {
            Ok(challenge) => challenge,
            Err(error) => return Err(Error::ChallengeParsingError(error)),
        };

        let mut hmac = Hmac::new(Md5::new(), password.as_bytes());
        hmac.input(&decoded_challenge);

        Ok(format!("{} {}", username, hmac.result().code().to_hex()).as_bytes().to_base64(base64::STANDARD))
    }
}

#[cfg(test)]
mod test {
    use super::{Mecanism, Plain, CramMd5};

    #[test]
    fn test_plain() {
        assert_eq!(Plain::response("username", "password", None).unwrap(), "AHVzZXJuYW1lAHBhc3N3b3Jk");
    }

    #[test]
    fn test_cram_md5() {
        assert_eq!(CramMd5::response("alice", "wonderland",
            Some("PDE3ODkzLjEzMjA2NzkxMjNAdGVzc2VyYWN0LnN1c2FtLmluPg==")).unwrap(),
            "YWxpY2UgNjRiMmE0M2MxZjZlZDY4MDZhOTgwOTE0ZTIzZTc1ZjA=");
    }
}