// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Speech* crate version *1.0.8+20181005*, where *20181005* is the exact revision of the *speech:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Speech* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/speech-to-text/docs/quickstart-protocol).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/speech1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Speech.html) ... 
//! 
//! * [operations](struct.Operation.html)
//!  * [*get*](struct.OperationGetCall.html) and [*list*](struct.OperationListCall.html)
//! * speech
//!  * [*longrunningrecognize*](struct.SpeechLongrunningrecognizeCall.html) and [*recognize*](struct.SpeechRecognizeCall.html)
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
//! * **[Hub](struct.Speech.html)**
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
//! let r = hub.operations().get(...).doit()
//! let r = hub.speech().longrunningrecognize(...).doit()
//! let r = hub.operations().list(...).doit()
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
//! google-speech1 = "*"
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
//! extern crate google_speech1 as speech1;
//! use speech1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use speech1::Speech;
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
//! let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.operations().list()
//!              .page_token("et")
//!              .page_size(-18)
//!              .name("kasd")
//!              .filter("accusam")
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

/// Central instance to access all Speech related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_speech1 as speech1;
/// use speech1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use speech1::Speech;
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
/// let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().list()
///              .page_token("takimata")
///              .page_size(-70)
///              .name("amet.")
///              .filter("erat")
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
pub struct Speech<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Speech<C, A> {}

impl<'a, C, A> Speech<C, A>
    where  C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Speech<C, A> {
        Speech {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://speech.googleapis.com/".to_string(),
            _root_url: "https://speech.googleapis.com/".to_string(),
        }
    }

    pub fn operations(&'a self) -> OperationMethods<'a, C, A> {
        OperationMethods { hub: &self }
    }
    pub fn speech(&'a self) -> SpeechMethods<'a, C, A> {
        SpeechMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://speech.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://speech.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// - Simple to use and understand for most users
/// - Flexible enough to meet unexpected needs
/// 
/// # Overview
/// 
/// The `Status` message contains three pieces of data: error code, error message,
/// and error details. The error code should be an enum value of
/// google.rpc.Code, but it may accept additional error codes if needed.  The
/// error message should be a developer-facing English message that helps
/// developers *understand* and *resolve* the error. If a localized user-facing
/// error message is needed, put the localized message in the error details or
/// localize it in the client. The optional error details may contain arbitrary
/// information about the error. There is a predefined set of error detail types
/// in the package `google.rpc` that can be used for common error conditions.
/// 
/// # Language mapping
/// 
/// The `Status` message is the logical representation of the error model, but it
/// is not necessarily the actual wire format. When the `Status` message is
/// exposed in different client libraries and different wire protocols, it can be
/// mapped differently. For example, it will likely be mapped to some exceptions
/// in Java, but more likely mapped to some error codes in C.
/// 
/// # Other uses
/// 
/// The error model and the `Status` message can be used in a variety of
/// environments, either with or without APIs, to provide a
/// consistent developer experience across different environments.
/// 
/// Example uses of this error model include:
/// 
/// - Partial errors. If a service needs to return partial errors to the client,
///     it may embed the `Status` in the normal response to indicate the partial
///     errors.
/// 
/// - Workflow errors. A typical workflow has multiple steps. Each step may
///     have a `Status` message for error reporting.
/// 
/// - Batch operations. If a client uses batch request and batch response, the
///     `Status` message should be used directly inside batch response, one for
///     each error sub-response.
/// 
/// - Asynchronous operations. If an API call embeds asynchronous operation
///     results in its response, the status of those operations should be
///     represented directly using the `Status` message.
/// 
/// - Logging. If some API errors are stored in logs, the message `Status` could
///     be used directly after any stripping needed for security/privacy reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
}

impl Part for Status {}


/// A speech recognition result corresponding to a portion of the audio.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechRecognitionResult {
    /// Output only. May contain one or more recognition hypotheses (up to the
    /// maximum specified in `max_alternatives`).
    /// These alternatives are ordered in terms of accuracy, with the top (first)
    /// alternative being the most probable, as ranked by the recognizer.
    pub alternatives: Option<Vec<SpeechRecognitionAlternative>>,
}

impl Part for SpeechRecognitionResult {}


/// The only message returned to the client by the `Recognize` method. It
/// contains the result as zero or more sequential `SpeechRecognitionResult`
/// messages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recognize speech](struct.SpeechRecognizeCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognizeResponse {
    /// Output only. Sequential list of transcription results corresponding to
    /// sequential portions of audio.
    pub results: Option<Vec<SpeechRecognitionResult>>,
}

impl ResponseResult for RecognizeResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](struct.OperationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    pub operations: Option<Vec<Operation>>,
}

impl ResponseResult for ListOperationsResponse {}


/// Provides "hints" to the speech recognizer to favor specific words and phrases
/// in the results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechContext {
    /// *Optional* A list of strings containing words and phrases "hints" so that
    /// the speech recognition is more likely to recognize them. This can be used
    /// to improve the accuracy for specific words and phrases, for example, if
    /// specific commands are typically spoken by the user. This can also be used
    /// to add additional words to the vocabulary of the recognizer. See
    /// [usage limits](/speech-to-text/quotas#content).
    pub phrases: Option<Vec<String>>,
}

impl Part for SpeechContext {}


/// Contains audio data in the encoding specified in the `RecognitionConfig`.
/// Either `content` or `uri` must be supplied. Supplying both or neither
/// returns google.rpc.Code.INVALID_ARGUMENT. See
/// [content limits](/speech-to-text/quotas#content).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognitionAudio {
    /// The audio data bytes encoded as specified in
    /// `RecognitionConfig`. Note: as with all bytes fields, protobuffers use a
    /// pure binary representation, whereas JSON representations use base64.
    pub content: Option<String>,
    /// URI that points to a file that contains audio data bytes as specified in
    /// `RecognitionConfig`. The file must not be compressed (for example, gzip).
    /// Currently, only Google Cloud Storage URIs are
    /// supported, which must be specified in the following format:
    /// `gs://bucket_name/object_name` (other URI formats return
    /// google.rpc.Code.INVALID_ARGUMENT). For more information, see
    /// [Request URIs](https://cloud.google.com/storage/docs/reference-uris).
    pub uri: Option<String>,
}

impl Part for RecognitionAudio {}


/// Word-specific information for recognized words.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WordInfo {
    /// Output only. Time offset relative to the beginning of the audio,
    /// and corresponding to the end of the spoken word.
    /// This field is only set if `enable_word_time_offsets=true` and only
    /// in the top hypothesis.
    /// This is an experimental feature and the accuracy of the time offset can
    /// vary.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// Output only. The word corresponding to this set of information.
    pub word: Option<String>,
    /// Output only. Time offset relative to the beginning of the audio,
    /// and corresponding to the start of the spoken word.
    /// This field is only set if `enable_word_time_offsets=true` and only
    /// in the top hypothesis.
    /// This is an experimental feature and the accuracy of the time offset can
    /// vary.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
    /// Output only. A distinct integer value is assigned for every speaker within
    /// the audio. This field specifies which one of those speakers was detected to
    /// have spoken this word. Value ranges from '1' to diarization_speaker_count.
    /// speaker_tag is set if enable_speaker_diarization = 'true' and only in the
    /// top alternative.
    #[serde(rename="speakerTag")]
    pub speaker_tag: Option<i32>,
}

impl Part for WordInfo {}


/// The top-level message sent by the client for the `LongRunningRecognize`
/// method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [longrunningrecognize speech](struct.SpeechLongrunningrecognizeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LongRunningRecognizeRequest {
    /// *Required* The audio data to be recognized.
    pub audio: Option<RecognitionAudio>,
    /// *Required* Provides information to the recognizer that specifies how to
    /// process the request.
    pub config: Option<RecognitionConfig>,
}

impl RequestValue for LongRunningRecognizeRequest {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operations](struct.OperationGetCall.html) (response)
/// * [longrunningrecognize speech](struct.SpeechLongrunningrecognizeCall.html) (response)
/// * [list operations](struct.OperationListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<Status>,
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    pub done: Option<bool>,
    /// The normal response of the operation in case of success.  If the original
    /// method returns no data on success, such as `Delete`, the response is
    /// `google.protobuf.Empty`.  If the original method is standard
    /// `Get`/`Create`/`Update`, the response should be the resource.  For other
    /// methods, the response should have the type `XxxResponse`, where `Xxx`
    /// is the original method name.  For example, if the original method name
    /// is `TakeSnapshot()`, the inferred response type is
    /// `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, String>>,
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should have the format of `operations/some/unique/name`.
    pub name: Option<String>,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, String>>,
}

impl Resource for Operation {}
impl ResponseResult for Operation {}


/// The top-level message sent by the client for the `Recognize` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recognize speech](struct.SpeechRecognizeCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognizeRequest {
    /// *Required* The audio data to be recognized.
    pub audio: Option<RecognitionAudio>,
    /// *Required* Provides information to the recognizer that specifies how to
    /// process the request.
    pub config: Option<RecognitionConfig>,
}

impl RequestValue for RecognizeRequest {}


/// Alternative hypotheses (a.k.a. n-best list).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpeechRecognitionAlternative {
    /// Output only. The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative of a non-streaming
    /// result or, of a streaming result where `is_final=true`.
    /// This field is not guaranteed to be accurate and users should not rely on it
    /// to be always provided.
    /// The default of 0.0 is a sentinel value indicating `confidence` was not set.
    pub confidence: Option<f32>,
    /// Output only. Transcript text representing the words that the user spoke.
    pub transcript: Option<String>,
    /// Output only. A list of word-specific information for each recognized word.
    /// Note: When `enable_speaker_diarization` is true, you will see all the words
    /// from the beginning of the audio.
    pub words: Option<Vec<WordInfo>>,
}

impl Part for SpeechRecognitionAlternative {}


/// Provides information to the recognizer that specifies how to process the
/// request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecognitionConfig {
    /// *Required* The language of the supplied audio as a
    /// [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag.
    /// Example: "en-US".
    /// See [Language Support](/speech-to-text/docs/languages)
    /// for a list of the currently supported language codes.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Encoding of audio data sent in all `RecognitionAudio` messages.
    /// This field is optional for `FLAC` and `WAV` audio files and required
    /// for all other audio formats. For details, see AudioEncoding.
    pub encoding: Option<String>,
    /// *Optional* If 'true', adds punctuation to recognition result hypotheses.
    /// This feature is only available in select languages. Setting this for
    /// requests in other languages has no effect at all.
    /// The default 'false' value does not add punctuation to result hypotheses.
    /// Note: This is currently offered as an experimental service, complimentary
    /// to all users. In the future this may be exclusively available as a
    /// premium feature.
    #[serde(rename="enableAutomaticPunctuation")]
    pub enable_automatic_punctuation: Option<bool>,
    /// *Optional* If `true`, the top result includes a list of words and
    /// the start and end time offsets (timestamps) for those words. If
    /// `false`, no word-level time offset information is returned. The default is
    /// `false`.
    #[serde(rename="enableWordTimeOffsets")]
    pub enable_word_time_offsets: Option<bool>,
    /// *Optional* Maximum number of recognition hypotheses to be returned.
    /// Specifically, the maximum number of `SpeechRecognitionAlternative` messages
    /// within each `SpeechRecognitionResult`.
    /// The server may return fewer than `max_alternatives`.
    /// Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of
    /// one. If omitted, will return a maximum of one.
    #[serde(rename="maxAlternatives")]
    pub max_alternatives: Option<i32>,
    /// *Optional* Set to true to use an enhanced model for speech recognition.
    /// If `use_enhanced` is set to true and the `model` field is not set, then
    /// an appropriate enhanced model is chosen if:
    /// 1. project is eligible for requesting enhanced models
    /// 2. an enhanced model exists for the audio
    /// 
    /// If `use_enhanced` is true and an enhanced version of the specified model
    /// does not exist, then the speech is recognized using the standard version
    /// of the specified model.
    /// 
    /// Enhanced speech models require that you opt-in to data logging using
    /// instructions in the
    /// [documentation](/speech-to-text/docs/enable-data-logging). If you set
    /// `use_enhanced` to true and you have not enabled audio logging, then you
    /// will receive an error.
    #[serde(rename="useEnhanced")]
    pub use_enhanced: Option<bool>,
    /// Sample rate in Hertz of the audio data sent in all
    /// `RecognitionAudio` messages. Valid values are: 8000-48000.
    /// 16000 is optimal. For best results, set the sampling rate of the audio
    /// source to 16000 Hz. If that's not possible, use the native sample rate of
    /// the audio source (instead of re-sampling).
    /// This field is optional for `FLAC` and `WAV` audio files and required
    /// for all other audio formats. For details, see AudioEncoding.
    #[serde(rename="sampleRateHertz")]
    pub sample_rate_hertz: Option<i32>,
    /// *Optional* If set to `true`, the server will attempt to filter out
    /// profanities, replacing all but the initial character in each filtered word
    /// with asterisks, e.g. "f***". If set to `false` or omitted, profanities
    /// won't be filtered out.
    #[serde(rename="profanityFilter")]
    pub profanity_filter: Option<bool>,
    /// *Optional* Which model to select for the given request. Select the model
    /// best suited to your domain to get best results. If a model is not
    /// explicitly specified, then we auto-select a model based on the parameters
    /// in the RecognitionConfig.
    /// <table>
    ///   <tr>
    ///     <td><b>Model</b></td>
    ///     <td><b>Description</b></td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>command_and_search</code></td>
    ///     <td>Best for short queries such as voice commands or voice search.</td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>phone_call</code></td>
    ///     <td>Best for audio that originated from a phone call (typically
    ///     recorded at an 8khz sampling rate).</td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>video</code></td>
    ///     <td>Best for audio that originated from from video or includes multiple
    ///         speakers. Ideally the audio is recorded at a 16khz or greater
    ///         sampling rate. This is a premium model that costs more than the
    ///         standard rate.</td>
    ///   </tr>
    ///   <tr>
    ///     <td><code>default</code></td>
    ///     <td>Best for audio that is not one of the specific audio models.
    ///         For example, long-form audio. Ideally the audio is high-fidelity,
    ///         recorded at a 16khz or greater sampling rate.</td>
    ///   </tr>
    /// </table>
    pub model: Option<String>,
    /// *Optional* array of SpeechContext.
    /// A means to provide context to assist the speech recognition. For more
    /// information, see [Phrase Hints](/speech-to-text/docs/basics#phrase-hints).
    #[serde(rename="speechContexts")]
    pub speech_contexts: Option<Vec<SpeechContext>>,
}

impl Part for RecognitionConfig {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the `Speech` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_speech1 as speech1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use speech1::Speech;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Speech<C, A>,
}

impl<'a, C, A> MethodsBuilder for OperationMethods<'a, C, A> {}

impl<'a, C, A> OperationMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, C, A> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the
    /// server doesn't support this method, it returns `UNIMPLEMENTED`.
    /// 
    /// NOTE: the `name` binding allows API services to override the binding
    /// to use different resource name schemes, such as `users/*/operations`. To
    /// override the binding, API services can add a binding such as
    /// `"/v1/{name=users/*}/operations"` to their service configuration.
    /// For backwards compatibility, the default name includes the operations
    /// collection id, however overriding users must ensure the name binding
    /// is the parent resource, without the operations collection id.
    pub fn list(&self) -> OperationListCall<'a, C, A> {
        OperationListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *speech* resources.
/// It is not used directly, but through the `Speech` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_speech1 as speech1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use speech1::Speech;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `longrunningrecognize(...)` and `recognize(...)`
/// // to build up your call.
/// let rb = hub.speech();
/// # }
/// ```
pub struct SpeechMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Speech<C, A>,
}

impl<'a, C, A> MethodsBuilder for SpeechMethods<'a, C, A> {}

impl<'a, C, A> SpeechMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs asynchronous speech recognition: receive results via the
    /// google.longrunning.Operations interface. Returns either an
    /// `Operation.error` or an `Operation.response` which contains
    /// a `LongRunningRecognizeResponse` message.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn longrunningrecognize(&self, request: LongRunningRecognizeRequest) -> SpeechLongrunningrecognizeCall<'a, C, A> {
        SpeechLongrunningrecognizeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs synchronous speech recognition: receive results after all audio
    /// has been sent and processed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn recognize(&self, request: RecognizeRequest) -> SpeechRecognizeCall<'a, C, A> {
        SpeechRecognizeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Gets the latest state of a long-running operation.  Clients can use this
/// method to poll the operation result at intervals as recommended by the API
/// service.
///
/// A builder for the *get* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_speech1 as speech1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use speech1::Speech;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().get("name")
///              .doit();
/// # }
/// ```
pub struct OperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Speech<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationGetCall<'a, C, A> {}

impl<'a, C, A> OperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, Operation)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "speech.operations.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
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

        let mut url = self.hub._base_url.clone() + "v1/operations/{+name}";
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
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::GET;
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OperationGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OperationGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists operations that match the specified filter in the request. If the
/// server doesn't support this method, it returns `UNIMPLEMENTED`.
/// 
/// NOTE: the `name` binding allows API services to override the binding
/// to use different resource name schemes, such as `users/*/operations`. To
/// override the binding, API services can add a binding such as
/// `"/v1/{name=users/*}/operations"` to their service configuration.
/// For backwards compatibility, the default name includes the operations
/// collection id, however overriding users must ensure the name binding
/// is the parent resource, without the operations collection id.
///
/// A builder for the *list* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_speech1 as speech1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use speech1::Speech;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().list()
///              .page_token("sea")
///              .page_size(-90)
///              .name("dolores")
///              .filter("gubergren")
///              .doit();
/// # }
/// ```
pub struct OperationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Speech<C, A>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _name: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationListCall<'a, C, A> {}

impl<'a, C, A> OperationListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, ListOperationsResponse)>, Error = cmn::Error>> {
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "speech.operations.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "pageToken", "pageSize", "name", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/operations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }


        use http::Uri;
        let url = url.parse::<Uri>().unwrap();



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
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::GET;
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


    /// The standard list page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OperationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The standard list page size.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> OperationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The name of the operation's parent resource.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> OperationListCall<'a, C, A> {
        self._name = Some(new_value.to_string());
        self
    }
    /// The standard list filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> OperationListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> OperationListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OperationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Performs asynchronous speech recognition: receive results via the
/// google.longrunning.Operations interface. Returns either an
/// `Operation.error` or an `Operation.response` which contains
/// a `LongRunningRecognizeResponse` message.
///
/// A builder for the *longrunningrecognize* method supported by a *speech* resource.
/// It is not used directly, but through a `SpeechMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_speech1 as speech1;
/// use speech1::LongRunningRecognizeRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use speech1::Speech;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LongRunningRecognizeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.speech().longrunningrecognize(req)
///              .doit();
/// # }
/// ```
pub struct SpeechLongrunningrecognizeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Speech<C, A>,
    _request: LongRunningRecognizeRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpeechLongrunningrecognizeCall<'a, C, A> {}

impl<'a, C, A> SpeechLongrunningrecognizeCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, Operation)>, Error = cmn::Error>> {
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "speech.speech.longrunningrecognize",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/speech:longrunningrecognize";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: LongRunningRecognizeRequest) -> SpeechLongrunningrecognizeCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SpeechLongrunningrecognizeCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> SpeechLongrunningrecognizeCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> SpeechLongrunningrecognizeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Performs synchronous speech recognition: receive results after all audio
/// has been sent and processed.
///
/// A builder for the *recognize* method supported by a *speech* resource.
/// It is not used directly, but through a `SpeechMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_speech1 as speech1;
/// use speech1::RecognizeRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use speech1::Speech;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Speech::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RecognizeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.speech().recognize(req)
///              .doit();
/// # }
/// ```
pub struct SpeechRecognizeCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Speech<C, A>,
    _request: RecognizeRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpeechRecognizeCall<'a, C, A> {}

impl<'a, C, A> SpeechRecognizeCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, RecognizeResponse)>, Error = cmn::Error>> {
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "speech.speech.recognize",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/speech:recognize";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
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
    pub fn request(mut self, new_value: RecognizeRequest) -> SpeechRecognizeCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SpeechRecognizeCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> SpeechRecognizeCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> SpeechRecognizeCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


