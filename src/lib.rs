//! # Google Sign-In
//!
//! This crate provides an API to verify Google's OAuth client id tokens
//! for use with Google is an authentication provider.
//!
//! Typically these tokens are generated by a web application using the
//! [Google Platform Library](https://developers.google.com/identity/sign-in/web/sign-in).
//!
//! ## Getting Started
//! Create a new client and configure it with your client id(s).
//!
//! ```
//! extern crate google_signin;
//!
//! # fn main() {
//! let mut client = google_signin::Client::new();
//! client.audiences.push("YOUR_CLIENT_ID.apps.googleusercontent.com".to_string()); // required
//! client.hosted_domains.push("YOUR_HOSTED_DOMAIN.tld".to_string()); // optional
//! # }
//! ```
//!
//! When you get an id token (typically in an HTTP request handler), you should verify
//! it using the client's `verify` method:
//!
//! ```
//! struct GoogleLogin {
//!     token: String,
//! }
//!
//! # async fn handler(client: &google_signin::Client, request: GoogleLogin) {
//! let mut certs_cache = google_signin::CachedCerts::new();
//! // Recommended: Let the crate handle everything for you
//! let id_info = client.verify(&request.token, &mut certs_cache).await.expect("Expected token to be valid");
//! println!("Success! Signed-in as {}", id_info.sub);
//!
//! // Alternative: Inspect the ID before verifying it
//! let id_info = client.get_slow_unverified(&request.token).await.expect("Expected token to exist");
//! let ok = id_info.verify(&client).is_ok();
//! println!("Ok: {}, Info: {:?}", ok, id_info);
//! # }
//! ```

extern crate serde;

mod client;
mod error;
mod token;

pub use client::CachedCerts;
pub use client::Client;
pub use error::Error;
pub use token::IdInfo;

#[cfg(test)]
mod test;
