// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *IAM Credentials* crate version *1.0.8+20181004*, where *20181004* is the exact revision of the *iamcredentials:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *IAM Credentials* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/iam/docs/creating-short-lived-service-account-credentials).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/iamcredentials1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.IAMCredentials.html) ... 
//! 
//! * projects
//!  * [*service accounts generate access token*](struct.ProjectServiceAccountGenerateAccessTokenCall.html), [*service accounts generate id token*](struct.ProjectServiceAccountGenerateIdTokenCall.html), [*service accounts generate identity binding access token*](struct.ProjectServiceAccountGenerateIdentityBindingAccessTokenCall.html), [*service accounts sign blob*](struct.ProjectServiceAccountSignBlobCall.html) and [*service accounts sign jwt*](struct.ProjectServiceAccountSignJwtCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.IAMCredentials.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.projects().service_accounts_generate_access_token(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-iamcredentials1 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_iamcredentials1 as iamcredentials1;
//! use iamcredentials1::GenerateAccessTokenRequest;
//! use iamcredentials1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use iamcredentials1::IAMCredentials;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = GenerateAccessTokenRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().service_accounts_generate_access_token(req, "name")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate http;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;
extern crate futures;

mod cmn;

use futures::{ Future, Stream };
use futures::future::Either;
use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all IAMCredentials related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::GenerateAccessTokenRequest;
/// use iamcredentials1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use iamcredentials1::IAMCredentials;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateAccessTokenRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_generate_access_token(req, "name")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct IAMCredentials<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for IAMCredentials<C, A> {}

impl<'a, C, A> IAMCredentials<C, A>
    where  C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> IAMCredentials<C, A> {
        IAMCredentials {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://iamcredentials.googleapis.com/".to_string(),
            _root_url: "https://iamcredentials.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C, A> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://iamcredentials.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://iamcredentials.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts generate access token projects](struct.ProjectServiceAccountGenerateAccessTokenCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateAccessTokenResponse {
    /// Token expiration time.
    /// The expiration time is always set.
    #[serde(rename="expireTime")]
    pub expire_time: Option<String>,
    /// The OAuth 2.0 access token.
    #[serde(rename="accessToken")]
    pub access_token: Option<String>,
}

impl ResponseResult for GenerateAccessTokenResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign jwt projects](struct.ProjectServiceAccountSignJwtCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignJwtResponse {
    /// The ID of the key used to sign the JWT.
    #[serde(rename="keyId")]
    pub key_id: Option<String>,
    /// The signed JWT.
    #[serde(rename="signedJwt")]
    pub signed_jwt: Option<String>,
}

impl ResponseResult for SignJwtResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign jwt projects](struct.ProjectServiceAccountSignJwtCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignJwtRequest {
    /// The JWT payload to sign: a JSON object that contains a JWT Claims Set.
    pub payload: Option<String>,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    /// 
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`
    pub delegates: Option<Vec<String>>,
}

impl RequestValue for SignJwtRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts generate identity binding access token projects](struct.ProjectServiceAccountGenerateIdentityBindingAccessTokenCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateIdentityBindingAccessTokenRequest {
    /// Code to identify the scopes to be included in the OAuth 2.0 access token.
    /// See https://developers.google.com/identity/protocols/googlescopes for more
    /// information.
    /// At least one value required.
    pub scope: Option<Vec<String>>,
    /// Required. Input token.
    /// Must be in JWT format according to
    /// RFC7523 (https://tools.ietf.org/html/rfc7523)
    /// and must have 'kid' field in the header.
    /// Supported signing algorithms: RS256 (RS512, ES256, ES512 coming soon).
    /// Mandatory payload fields (along the lines of RFC 7523, section 3):
    /// - iss: issuer of the token. Must provide a discovery document at
    ///        $iss/.well-known/openid-configuration . The document needs to be
    ///        formatted according to section 4.2 of the OpenID Connect Discovery
    ///        1.0 specification.
    /// - iat: Issue time in seconds since epoch. Must be in the past.
    /// - exp: Expiration time in seconds since epoch. Must be less than 48 hours
    ///        after iat. We recommend to create tokens that last shorter than 6
    ///        hours to improve security unless business reasons mandate longer
    ///        expiration times. Shorter token lifetimes are generally more secure
    ///        since tokens that have been exfiltrated by attackers can be used for
    ///        a shorter time. you can configure the maximum lifetime of the
    ///        incoming token in the configuration of the mapper.
    ///        The resulting Google token will expire within an hour or at "exp",
    ///        whichever is earlier.
    /// - sub: JWT subject, identity asserted in the JWT.
    /// - aud: Configured in the mapper policy. By default the service account
    ///        email.
    /// 
    /// Claims from the incoming token can be transferred into the output token
    /// accoding to the mapper configuration. The outgoing claim size is limited.
    /// Outgoing claims size must be less than 4kB serialized as JSON without
    /// whitespace.
    /// 
    /// Example header:
    /// {
    ///   "alg": "RS256",
    ///   "kid": "92a4265e14ab04d4d228a48d10d4ca31610936f8"
    /// }
    /// Example payload:
    /// {
    ///   "iss": "https://accounts.google.com",
    ///   "iat": 1517963104,
    ///   "exp": 1517966704,
    ///   "aud": "https://iamcredentials.googleapis.com/google.iam.credentials.v1.CloudGaia",
    ///   "sub": "113475438248934895348",
    ///   "my_claims": {
    ///     "additional_claim": "value"
    ///   }
    /// }
    pub jwt: Option<String>,
}

impl RequestValue for GenerateIdentityBindingAccessTokenRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts generate identity binding access token projects](struct.ProjectServiceAccountGenerateIdentityBindingAccessTokenCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateIdentityBindingAccessTokenResponse {
    /// Token expiration time.
    /// The expiration time is always set.
    #[serde(rename="expireTime")]
    pub expire_time: Option<String>,
    /// The OAuth 2.0 access token.
    #[serde(rename="accessToken")]
    pub access_token: Option<String>,
}

impl ResponseResult for GenerateIdentityBindingAccessTokenResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign blob projects](struct.ProjectServiceAccountSignBlobCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignBlobResponse {
    /// The ID of the key used to sign the blob.
    #[serde(rename="keyId")]
    pub key_id: Option<String>,
    /// The signed blob.
    #[serde(rename="signedBlob")]
    pub signed_blob: Option<String>,
}

impl ResponseResult for SignBlobResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign blob projects](struct.ProjectServiceAccountSignBlobCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignBlobRequest {
    /// The bytes to sign.
    pub payload: Option<String>,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    /// 
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`
    pub delegates: Option<Vec<String>>,
}

impl RequestValue for SignBlobRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts generate access token projects](struct.ProjectServiceAccountGenerateAccessTokenCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateAccessTokenRequest {
    /// The desired lifetime duration of the access token in seconds.
    /// Must be set to a value less than or equal to 3600 (1 hour). If a value is
    /// not specified, the token's lifetime will be set to a default value of one
    /// hour.
    pub lifetime: Option<String>,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    /// 
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`
    pub delegates: Option<Vec<String>>,
    /// Code to identify the scopes to be included in the OAuth 2.0 access token.
    /// See https://developers.google.com/identity/protocols/googlescopes for more
    /// information.
    /// At least one value required.
    pub scope: Option<Vec<String>>,
}

impl RequestValue for GenerateAccessTokenRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts generate id token projects](struct.ProjectServiceAccountGenerateIdTokenCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateIdTokenResponse {
    /// The OpenId Connect ID token.
    pub token: Option<String>,
}

impl ResponseResult for GenerateIdTokenResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts generate id token projects](struct.ProjectServiceAccountGenerateIdTokenCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateIdTokenRequest {
    /// Include the service account email in the token. If set to `true`, the
    /// token will contain `email` and `email_verified` claims.
    #[serde(rename="includeEmail")]
    pub include_email: Option<bool>,
    /// The audience for the token, such as the API or account that this token
    /// grants access to.
    pub audience: Option<String>,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    /// 
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`
    pub delegates: Option<Vec<String>>,
}

impl RequestValue for GenerateIdTokenRequest {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `IAMCredentials` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_iamcredentials1 as iamcredentials1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use iamcredentials1::IAMCredentials;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `service_accounts_generate_access_token(...)`, `service_accounts_generate_id_token(...)`, `service_accounts_generate_identity_binding_access_token(...)`, `service_accounts_sign_blob(...)` and `service_accounts_sign_jwt(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a IAMCredentials<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the service account for which the credentials
    ///            are requested, in the following format:
    ///            `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    pub fn service_accounts_generate_identity_binding_access_token(&self, request: GenerateIdentityBindingAccessTokenRequest, name: &str) -> ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A> {
        ProjectServiceAccountGenerateIdentityBindingAccessTokenCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Signs a blob using a service account's system-managed private key.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the service account for which the credentials
    ///            are requested, in the following format:
    ///            `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    pub fn service_accounts_sign_blob(&self, request: SignBlobRequest, name: &str) -> ProjectServiceAccountSignBlobCall<'a, C, A> {
        ProjectServiceAccountSignBlobCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Signs a JWT using a service account's system-managed private key.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the service account for which the credentials
    ///            are requested, in the following format:
    ///            `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    pub fn service_accounts_sign_jwt(&self, request: SignJwtRequest, name: &str) -> ProjectServiceAccountSignJwtCall<'a, C, A> {
        ProjectServiceAccountSignJwtCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates an OpenID Connect ID token for a service account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the service account for which the credentials
    ///            are requested, in the following format:
    ///            `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    pub fn service_accounts_generate_id_token(&self, request: GenerateIdTokenRequest, name: &str) -> ProjectServiceAccountGenerateIdTokenCall<'a, C, A> {
        ProjectServiceAccountGenerateIdTokenCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates an OAuth 2.0 access token for a service account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the service account for which the credentials
    ///            are requested, in the following format:
    ///            `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    pub fn service_accounts_generate_access_token(&self, request: GenerateAccessTokenRequest, name: &str) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C, A> {
        ProjectServiceAccountGenerateAccessTokenCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// 
///
/// A builder for the *serviceAccounts.generateIdentityBindingAccessToken* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::GenerateIdentityBindingAccessTokenRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use iamcredentials1::IAMCredentials;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateIdentityBindingAccessTokenRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_generate_identity_binding_access_token(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a IAMCredentials<C, A>,
    _request: GenerateIdentityBindingAccessTokenRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A> {}

impl<'a, C, A> ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, GenerateIdentityBindingAccessTokenResponse)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "iamcredentials.projects.serviceAccounts.generateIdentityBindingAccessToken",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:generateIdentityBindingAccessToken";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::MissingAPIKey));
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        use http::Uri;
        let url = url.parse::<Uri>().unwrap();

        let mut json_mime_type = "application/json";
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::POST;
                *req.uri_mut() = url.clone();
                {
                    let headers_mut = req.headers_mut();
                    headers_mut.insert(
                        hyper::header::USER_AGENT,
                        http::header::HeaderValue::from_str(
                            &self.hub._user_agent.clone()
                        ).unwrap()
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    headers_mut.insert(
                        hyper::header::CONTENT_TYPE,
                        HeaderValue::from_str(&json_mime_type.clone()).unwrap()
                    );
                    headers_mut.insert(
                        hyper::header::CONTENT_LENGTH,
                        HeaderValue::from_str(&format!("{}", request_size as u64)).unwrap()
                    );
                }
                let mut buffer = Vec::new();
                request_value_reader.read_to_end(&mut buffer).unwrap();
                {
                    *req.body_mut() = hyper::Body::from(buffer);
                }

                dlg.pre_request();
                let client = hyper::client::Client::new();
                client.request(req)
            };
            use std::io::Write;
            let final_fut = req_fut.map(|mut res| {
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    match json::from_str::<ErrorResponse>(&json_err) {
                        Err(_) => {
                            return Box::new(futures::future::err(Error::Failure(res)));
                        }
                        Ok(serr) => {
                            return Box::new(futures::future::err(Error::BadRequest(serr)));
                        }
                    }
                }
                let result_value: (http::Response<hyper::Body>, serde_json::Value) = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Box::new(futures::future::err(Error::JsonDecodeError(json_response, err)));
                        }
                    }
                };

                dlg.finished(true);
                return Box::new(futures::future::ok(Ok(result_value)))
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                */
                dlg.finished(false);
                // Just return some type of cmn::Error
                return Box::new(futures::future::err(Error::Cancelled));
            });
            return Box::new(final_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GenerateIdentityBindingAccessTokenRequest) -> ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountGenerateIdentityBindingAccessTokenCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Signs a blob using a service account's system-managed private key.
///
/// A builder for the *serviceAccounts.signBlob* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::SignBlobRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use iamcredentials1::IAMCredentials;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SignBlobRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_sign_blob(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectServiceAccountSignBlobCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a IAMCredentials<C, A>,
    _request: SignBlobRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectServiceAccountSignBlobCall<'a, C, A> {}

impl<'a, C, A> ProjectServiceAccountSignBlobCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, SignBlobResponse)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "iamcredentials.projects.serviceAccounts.signBlob",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:signBlob";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        use http::Uri;
        let url = url.parse::<Uri>().unwrap();

        let mut json_mime_type = "application/json";
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Box::new(futures::future::err(Error::MissingToken(err)));
                        }
                    }
                }
            };
            let auth_header = HeaderValue::from_str(&format!("Authorization: Bearer {}", token.access_token)).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::POST;
                *req.uri_mut() = url.clone();
                {
                    let headers_mut = req.headers_mut();
                    headers_mut.insert(
                        hyper::header::USER_AGENT,
                        http::header::HeaderValue::from_str(
                            &self.hub._user_agent.clone()
                        ).unwrap()
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    // TODO auth_header needs to not have the header name
                    headers_mut.insert(
                        hyper::header::AUTHORIZATION,
                        auth_header
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    headers_mut.insert(
                        hyper::header::CONTENT_TYPE,
                        HeaderValue::from_str(&json_mime_type.clone()).unwrap()
                    );
                    headers_mut.insert(
                        hyper::header::CONTENT_LENGTH,
                        HeaderValue::from_str(&format!("{}", request_size as u64)).unwrap()
                    );
                }
                let mut buffer = Vec::new();
                request_value_reader.read_to_end(&mut buffer).unwrap();
                {
                    *req.body_mut() = hyper::Body::from(buffer);
                }

                dlg.pre_request();
                let client = hyper::client::Client::new();
                client.request(req)
            };
            use std::io::Write;
            let final_fut = req_fut.map(|mut res| {
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    match json::from_str::<ErrorResponse>(&json_err) {
                        Err(_) => {
                            return Box::new(futures::future::err(Error::Failure(res)));
                        }
                        Ok(serr) => {
                            return Box::new(futures::future::err(Error::BadRequest(serr)));
                        }
                    }
                }
                let result_value: (http::Response<hyper::Body>, serde_json::Value) = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Box::new(futures::future::err(Error::JsonDecodeError(json_response, err)));
                        }
                    }
                };

                dlg.finished(true);
                return Box::new(futures::future::ok(Ok(result_value)))
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                */
                dlg.finished(false);
                // Just return some type of cmn::Error
                return Box::new(futures::future::err(Error::Cancelled));
            });
            return Box::new(final_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: SignBlobRequest) -> ProjectServiceAccountSignBlobCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountSignBlobCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectServiceAccountSignBlobCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountSignBlobCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectServiceAccountSignBlobCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Signs a JWT using a service account's system-managed private key.
///
/// A builder for the *serviceAccounts.signJwt* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::SignJwtRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use iamcredentials1::IAMCredentials;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SignJwtRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_sign_jwt(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectServiceAccountSignJwtCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a IAMCredentials<C, A>,
    _request: SignJwtRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectServiceAccountSignJwtCall<'a, C, A> {}

impl<'a, C, A> ProjectServiceAccountSignJwtCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, SignJwtResponse)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "iamcredentials.projects.serviceAccounts.signJwt",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:signJwt";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        use http::Uri;
        let url = url.parse::<Uri>().unwrap();

        let mut json_mime_type = "application/json";
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Box::new(futures::future::err(Error::MissingToken(err)));
                        }
                    }
                }
            };
            let auth_header = HeaderValue::from_str(&format!("Authorization: Bearer {}", token.access_token)).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::POST;
                *req.uri_mut() = url.clone();
                {
                    let headers_mut = req.headers_mut();
                    headers_mut.insert(
                        hyper::header::USER_AGENT,
                        http::header::HeaderValue::from_str(
                            &self.hub._user_agent.clone()
                        ).unwrap()
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    // TODO auth_header needs to not have the header name
                    headers_mut.insert(
                        hyper::header::AUTHORIZATION,
                        auth_header
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    headers_mut.insert(
                        hyper::header::CONTENT_TYPE,
                        HeaderValue::from_str(&json_mime_type.clone()).unwrap()
                    );
                    headers_mut.insert(
                        hyper::header::CONTENT_LENGTH,
                        HeaderValue::from_str(&format!("{}", request_size as u64)).unwrap()
                    );
                }
                let mut buffer = Vec::new();
                request_value_reader.read_to_end(&mut buffer).unwrap();
                {
                    *req.body_mut() = hyper::Body::from(buffer);
                }

                dlg.pre_request();
                let client = hyper::client::Client::new();
                client.request(req)
            };
            use std::io::Write;
            let final_fut = req_fut.map(|mut res| {
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    match json::from_str::<ErrorResponse>(&json_err) {
                        Err(_) => {
                            return Box::new(futures::future::err(Error::Failure(res)));
                        }
                        Ok(serr) => {
                            return Box::new(futures::future::err(Error::BadRequest(serr)));
                        }
                    }
                }
                let result_value: (http::Response<hyper::Body>, serde_json::Value) = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Box::new(futures::future::err(Error::JsonDecodeError(json_response, err)));
                        }
                    }
                };

                dlg.finished(true);
                return Box::new(futures::future::ok(Ok(result_value)))
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                */
                dlg.finished(false);
                // Just return some type of cmn::Error
                return Box::new(futures::future::err(Error::Cancelled));
            });
            return Box::new(final_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: SignJwtRequest) -> ProjectServiceAccountSignJwtCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountSignJwtCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectServiceAccountSignJwtCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountSignJwtCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectServiceAccountSignJwtCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generates an OpenID Connect ID token for a service account.
///
/// A builder for the *serviceAccounts.generateIdToken* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::GenerateIdTokenRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use iamcredentials1::IAMCredentials;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateIdTokenRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_generate_id_token(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectServiceAccountGenerateIdTokenCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a IAMCredentials<C, A>,
    _request: GenerateIdTokenRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectServiceAccountGenerateIdTokenCall<'a, C, A> {}

impl<'a, C, A> ProjectServiceAccountGenerateIdTokenCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, GenerateIdTokenResponse)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "iamcredentials.projects.serviceAccounts.generateIdToken",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:generateIdToken";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        use http::Uri;
        let url = url.parse::<Uri>().unwrap();

        let mut json_mime_type = "application/json";
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Box::new(futures::future::err(Error::MissingToken(err)));
                        }
                    }
                }
            };
            let auth_header = HeaderValue::from_str(&format!("Authorization: Bearer {}", token.access_token)).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::POST;
                *req.uri_mut() = url.clone();
                {
                    let headers_mut = req.headers_mut();
                    headers_mut.insert(
                        hyper::header::USER_AGENT,
                        http::header::HeaderValue::from_str(
                            &self.hub._user_agent.clone()
                        ).unwrap()
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    // TODO auth_header needs to not have the header name
                    headers_mut.insert(
                        hyper::header::AUTHORIZATION,
                        auth_header
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    headers_mut.insert(
                        hyper::header::CONTENT_TYPE,
                        HeaderValue::from_str(&json_mime_type.clone()).unwrap()
                    );
                    headers_mut.insert(
                        hyper::header::CONTENT_LENGTH,
                        HeaderValue::from_str(&format!("{}", request_size as u64)).unwrap()
                    );
                }
                let mut buffer = Vec::new();
                request_value_reader.read_to_end(&mut buffer).unwrap();
                {
                    *req.body_mut() = hyper::Body::from(buffer);
                }

                dlg.pre_request();
                let client = hyper::client::Client::new();
                client.request(req)
            };
            use std::io::Write;
            let final_fut = req_fut.map(|mut res| {
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    match json::from_str::<ErrorResponse>(&json_err) {
                        Err(_) => {
                            return Box::new(futures::future::err(Error::Failure(res)));
                        }
                        Ok(serr) => {
                            return Box::new(futures::future::err(Error::BadRequest(serr)));
                        }
                    }
                }
                let result_value: (http::Response<hyper::Body>, serde_json::Value) = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Box::new(futures::future::err(Error::JsonDecodeError(json_response, err)));
                        }
                    }
                };

                dlg.finished(true);
                return Box::new(futures::future::ok(Ok(result_value)))
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                */
                dlg.finished(false);
                // Just return some type of cmn::Error
                return Box::new(futures::future::err(Error::Cancelled));
            });
            return Box::new(final_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GenerateIdTokenRequest) -> ProjectServiceAccountGenerateIdTokenCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountGenerateIdTokenCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectServiceAccountGenerateIdTokenCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountGenerateIdTokenCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectServiceAccountGenerateIdTokenCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generates an OAuth 2.0 access token for a service account.
///
/// A builder for the *serviceAccounts.generateAccessToken* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_iamcredentials1 as iamcredentials1;
/// use iamcredentials1::GenerateAccessTokenRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use iamcredentials1::IAMCredentials;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = IAMCredentials::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateAccessTokenRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_accounts_generate_access_token(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectServiceAccountGenerateAccessTokenCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a IAMCredentials<C, A>,
    _request: GenerateAccessTokenRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectServiceAccountGenerateAccessTokenCall<'a, C, A> {}

impl<'a, C, A> ProjectServiceAccountGenerateAccessTokenCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, GenerateAccessTokenResponse)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "iamcredentials.projects.serviceAccounts.generateAccessToken",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:generateAccessToken";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        use http::Uri;
        let url = url.parse::<Uri>().unwrap();

        let mut json_mime_type = "application/json";
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Box::new(futures::future::err(Error::MissingToken(err)));
                        }
                    }
                }
            };
            let auth_header = HeaderValue::from_str(&format!("Authorization: Bearer {}", token.access_token)).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::POST;
                *req.uri_mut() = url.clone();
                {
                    let headers_mut = req.headers_mut();
                    headers_mut.insert(
                        hyper::header::USER_AGENT,
                        http::header::HeaderValue::from_str(
                            &self.hub._user_agent.clone()
                        ).unwrap()
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    // TODO auth_header needs to not have the header name
                    headers_mut.insert(
                        hyper::header::AUTHORIZATION,
                        auth_header
                    );
                }
                {
                    let headers_mut = req.headers_mut();

                    headers_mut.insert(
                        hyper::header::CONTENT_TYPE,
                        HeaderValue::from_str(&json_mime_type.clone()).unwrap()
                    );
                    headers_mut.insert(
                        hyper::header::CONTENT_LENGTH,
                        HeaderValue::from_str(&format!("{}", request_size as u64)).unwrap()
                    );
                }
                let mut buffer = Vec::new();
                request_value_reader.read_to_end(&mut buffer).unwrap();
                {
                    *req.body_mut() = hyper::Body::from(buffer);
                }

                dlg.pre_request();
                let client = hyper::client::Client::new();
                client.request(req)
            };
            use std::io::Write;
            let final_fut = req_fut.map(|mut res| {
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    match json::from_str::<ErrorResponse>(&json_err) {
                        Err(_) => {
                            return Box::new(futures::future::err(Error::Failure(res)));
                        }
                        Ok(serr) => {
                            return Box::new(futures::future::err(Error::BadRequest(serr)));
                        }
                    }
                }
                let result_value: (http::Response<hyper::Body>, serde_json::Value) = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Box::new(futures::future::err(Error::JsonDecodeError(json_response, err)));
                        }
                    }
                };

                dlg.finished(true);
                return Box::new(futures::future::ok(Ok(result_value)))
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                */
                dlg.finished(false);
                // Just return some type of cmn::Error
                return Box::new(futures::future::err(Error::Cancelled));
            });
            return Box::new(final_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GenerateAccessTokenRequest) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectServiceAccountGenerateAccessTokenCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


