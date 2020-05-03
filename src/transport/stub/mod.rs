//! The stub transport only logs message envelope and drops the content. It can be useful for
//! testing purposes.
//!
//! #### Stub Transport
//!
//! The stub transport returns provided result and drops the content. It can be useful for
//! testing purposes.
//!
//! ```rust
//! use lettre::{Message, Envelope, Transport, StubTransport};
//!
//! let email = Message::builder()
//!     .from("NoBody <nobody@domain.tld>".parse().unwrap())
//!     .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
//!     .to("Hei <hei@domain.tld>".parse().unwrap())
//!     .subject("Happy new year")
//!     .body("Be happy!")
//!     .unwrap();
//!
//! let mut sender = StubTransport::new_positive();
//! let result = sender.send(&email);
//! assert!(result.is_ok());
//! ```

use crate::{Envelope, Transport};

/// This transport logs the message envelope and returns the given response
#[derive(Debug, Clone, Copy)]
pub struct StubTransport {
    response: StubResult,
}

impl StubTransport {
    /// Creates a new transport that always returns the given response
    pub fn new(response: StubResult) -> StubTransport {
        StubTransport { response }
    }

    /// Creates a new transport that always returns a success response
    pub fn new_positive() -> StubTransport {
        StubTransport { response: Ok(()) }
    }
}

/// SMTP result type
pub type StubResult = Result<(), ()>;

impl<'a> Transport<'a> for StubTransport {
    type Result = StubResult;

    fn send_raw(&self, _envelope: &Envelope, _email: &[u8]) -> Self::Result {
        self.response
    }
}