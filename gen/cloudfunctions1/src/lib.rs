// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Functions* crate version *1.0.8+20181002*, where *20181002* is the exact revision of the *cloudfunctions:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Cloud Functions* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/functions).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/cloudfunctions1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CloudFunctions.html) ... 
//! 
//! * [operations](struct.Operation.html)
//!  * [*get*](struct.OperationGetCall.html) and [*list*](struct.OperationListCall.html)
//! * projects
//!  * [*locations functions call*](struct.ProjectLocationFunctionCallCall.html), [*locations functions create*](struct.ProjectLocationFunctionCreateCall.html), [*locations functions delete*](struct.ProjectLocationFunctionDeleteCall.html), [*locations functions generate download url*](struct.ProjectLocationFunctionGenerateDownloadUrlCall.html), [*locations functions generate upload url*](struct.ProjectLocationFunctionGenerateUploadUrlCall.html), [*locations functions get*](struct.ProjectLocationFunctionGetCall.html), [*locations functions list*](struct.ProjectLocationFunctionListCall.html), [*locations functions patch*](struct.ProjectLocationFunctionPatchCall.html) and [*locations list*](struct.ProjectLocationListCall.html)
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
//! * **[Hub](struct.CloudFunctions.html)**
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
//! let r = hub.projects().locations_functions_patch(...).doit()
//! let r = hub.operations().list(...).doit()
//! let r = hub.operations().get(...).doit()
//! let r = hub.projects().locations_functions_delete(...).doit()
//! let r = hub.projects().locations_functions_create(...).doit()
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
//! google-cloudfunctions1 = "*"
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
//! extern crate google_cloudfunctions1 as cloudfunctions1;
//! use cloudfunctions1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use cloudfunctions1::CloudFunctions;
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
//! let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
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

/// Central instance to access all CloudFunctions related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudfunctions1 as cloudfunctions1;
/// use cloudfunctions1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudfunctions1::CloudFunctions;
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
/// let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
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
pub struct CloudFunctions<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for CloudFunctions<C, A> {}

impl<'a, C, A> CloudFunctions<C, A>
    where  C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CloudFunctions<C, A> {
        CloudFunctions {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://cloudfunctions.googleapis.com/".to_string(),
            _root_url: "https://cloudfunctions.googleapis.com/".to_string(),
        }
    }

    pub fn operations(&'a self) -> OperationMethods<'a, C, A> {
        OperationMethods { hub: &self }
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
    /// It defaults to `https://cloudfunctions.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://cloudfunctions.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Request for the `CallFunction` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions call projects](struct.ProjectLocationFunctionCallCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CallFunctionRequest {
    /// Input to be passed to the function.
    pub data: Option<String>,
}

impl RequestValue for CallFunctionRequest {}


/// Describes EventTrigger, used to request events be sent from another
/// service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTrigger {
    /// Required. The type of event to observe. For example:
    /// `providers/cloud.storage/eventTypes/object.change` and
    /// `providers/cloud.pubsub/eventTypes/topic.publish`.
    /// 
    /// Event types match pattern `providers/*/eventTypes/*.*`.
    /// The pattern contains:
    /// 
    /// 1. namespace: For example, `cloud.storage` and
    ///    `google.firebase.analytics`.
    /// 2. resource type: The type of resource on which event occurs. For
    ///    example, the Google Cloud Storage API includes the type `object`.
    /// 3. action: The action that generates the event. For example, action for
    ///    a Google Cloud Storage Object is 'change'.
    /// These parts are lower case.
    #[serde(rename="eventType")]
    pub event_type: Option<String>,
    /// Required. The resource(s) from which to observe events, for example,
    /// `projects/_/buckets/myBucket`.
    /// 
    /// Not all syntactically correct values are accepted by all services. For
    /// example:
    /// 
    /// 1. The authorization model must support it. Google Cloud Functions
    ///    only allows EventTriggers to be deployed that observe resources in the
    ///    same project as the `CloudFunction`.
    /// 2. The resource type must match the pattern expected for an
    ///    `event_type`. For example, an `EventTrigger` that has an
    ///    `event_type` of "google.pubsub.topic.publish" should have a resource
    ///    that matches Google Cloud Pub/Sub topics.
    /// 
    /// Additionally, some services may support short names when creating an
    /// `EventTrigger`. These will always be returned in the normalized "long"
    /// format.
    /// 
    /// See each *service's* documentation for supported formats.
    pub resource: Option<String>,
    /// The hostname of the service that should be observed.
    /// 
    /// If no string is provided, the default service implementing the API will
    /// be used. For example, `storage.googleapis.com` is the default for all
    /// event types in the `google.storage` namespace.
    pub service: Option<String>,
    /// Specifies policy for failed executions.
    #[serde(rename="failurePolicy")]
    pub failure_policy: Option<FailurePolicy>,
}

impl Part for EventTrigger {}


/// Describes the retry policy in case of function's execution failure.
/// A function execution will be retried on any failure.
/// A failed execution will be retried up to 7 days with an exponential backoff
/// (capped at 10 seconds).
/// Retried execution is charged as any other execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Retry { _never_set: Option<bool> }

impl Part for Retry {}


/// Response of `GenerateSourceUploadUrl` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions generate upload url projects](struct.ProjectLocationFunctionGenerateUploadUrlCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateUploadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for a
    /// function source code upload. The uploaded file should be a zip archive
    /// which contains a function.
    #[serde(rename="uploadUrl")]
    pub upload_url: Option<String>,
}

impl ResponseResult for GenerateUploadUrlResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](struct.ProjectLocationListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// A list of locations that matches the specified filter in the request.
    pub locations: Option<Vec<Location>>,
}

impl ResponseResult for ListLocationsResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    pub location_id: Option<String>,
    /// Cross-service attributes for the location. For example
    /// 
    ///     {"cloud.googleapis.com/region": "us-east1"}
    pub labels: Option<HashMap<String, String>>,
    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    pub name: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    pub metadata: Option<HashMap<String, String>>,
}

impl Part for Location {}


/// Response of `CallFunction` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions call projects](struct.ProjectLocationFunctionCallCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CallFunctionResponse {
    /// Execution id of function invocation.
    #[serde(rename="executionId")]
    pub execution_id: Option<String>,
    /// Result populated for successful execution of synchronous function. Will
    /// not be populated if function does not return a result through context.
    pub result: Option<String>,
    /// Either system or user-function generated error. Set if execution
    /// was not successful.
    pub error: Option<String>,
}

impl ResponseResult for CallFunctionResponse {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions patch projects](struct.ProjectLocationFunctionPatchCall.html) (response)
/// * [list operations](struct.OperationListCall.html) (none)
/// * [get operations](struct.OperationGetCall.html) (response)
/// * [locations functions delete projects](struct.ProjectLocationFunctionDeleteCall.html) (response)
/// * [locations functions create projects](struct.ProjectLocationFunctionCreateCall.html) (response)
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


/// Describes SourceRepository, used to represent parameters related to
/// source repository where a function is hosted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceRepository {
    /// The URL pointing to the hosted repository where the function is defined.
    /// There are supported Cloud Source Repository URLs in the following
    /// formats:
    /// 
    /// To refer to a specific commit:
    /// `https://source.developers.google.com/projects/*/repos/*/revisions/*/paths/*`
    /// To refer to a moveable alias (branch):
    /// `https://source.developers.google.com/projects/*/repos/*/moveable-aliases/*/paths/*`
    /// In particular, to refer to HEAD use `master` moveable alias.
    /// To refer to a specific fixed alias (tag):
    /// `https://source.developers.google.com/projects/*/repos/*/fixed-aliases/*/paths/*`
    /// 
    /// You may omit `paths/*` if you want to use the main directory.
    pub url: Option<String>,
    /// Output only. The URL pointing to the hosted repository where the function
    /// were defined at the time of deployment. It always points to a specific
    /// commit in the format described above.
    #[serde(rename="deployedUrl")]
    pub deployed_url: Option<String>,
}

impl Part for SourceRepository {}


/// Describes HttpsTrigger, could be used to connect web hooks to function.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpsTrigger {
    /// Output only. The deployed url for the function.
    pub url: Option<String>,
}

impl Part for HttpsTrigger {}


/// Request of `GenerateDownloadUrl` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions generate download url projects](struct.ProjectLocationFunctionGenerateDownloadUrlCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateDownloadUrlRequest {
    /// The optional version of function. If not set, default, current version
    /// is used.
    #[serde(rename="versionId")]
    pub version_id: Option<String>,
}

impl RequestValue for GenerateDownloadUrlRequest {}


/// Describes the policy in case of function's execution failure.
/// If empty, then defaults to ignoring failures (i.e. not retrying them).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FailurePolicy {
    /// If specified, then the function will be retried in case of a failure.
    pub retry: Option<Retry>,
}

impl Part for FailurePolicy {}


/// Response of `GenerateDownloadUrl` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions generate download url projects](struct.ProjectLocationFunctionGenerateDownloadUrlCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateDownloadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for
    /// function source code download.
    #[serde(rename="downloadUrl")]
    pub download_url: Option<String>,
}

impl ResponseResult for GenerateDownloadUrlResponse {}


/// Response for the `ListFunctions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions list projects](struct.ProjectLocationFunctionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFunctionsResponse {
    /// If not empty, indicates that there may be more functions that match
    /// the request; this value should be passed in a new
    /// google.cloud.functions.v1.ListFunctionsRequest
    /// to get more functions.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The functions that match the request.
    pub functions: Option<Vec<CloudFunction>>,
}

impl ResponseResult for ListFunctionsResponse {}


/// Request of `GenerateSourceUploadUrl` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions generate upload url projects](struct.ProjectLocationFunctionGenerateUploadUrlCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateUploadUrlRequest { _never_set: Option<bool> }

impl RequestValue for GenerateUploadUrlRequest {}


/// Describes a Cloud Function that contains user computation executed in
/// response to an event. It encapsulate function and triggers configurations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations functions patch projects](struct.ProjectLocationFunctionPatchCall.html) (request)
/// * [locations functions get projects](struct.ProjectLocationFunctionGetCall.html) (response)
/// * [locations functions create projects](struct.ProjectLocationFunctionCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudFunction {
    /// Output only. Status of the function deployment.
    pub status: Option<String>,
    /// A source that fires events in response to a condition in another service.
    #[serde(rename="eventTrigger")]
    pub event_trigger: Option<EventTrigger>,
    /// Output only. The last update timestamp of a Cloud Function.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// User-provided description of a function.
    pub description: Option<String>,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time. This feature is currently in alpha, available only for
    /// whitelisted users.
    #[serde(rename="maxInstances")]
    pub max_instances: Option<i32>,
    /// **Beta Feature**
    /// 
    /// The source repository where a function is hosted.
    #[serde(rename="sourceRepository")]
    pub source_repository: Option<SourceRepository>,
    /// An HTTPS endpoint type of source that can be triggered via URL.
    #[serde(rename="httpsTrigger")]
    pub https_trigger: Option<HttpsTrigger>,
    /// The Google Cloud Storage URL, starting with gs://, pointing to the zip
    /// archive which contains the function.
    #[serde(rename="sourceArchiveUrl")]
    pub source_archive_url: Option<String>,
    /// Labels associated with this Cloud Function.
    pub labels: Option<HashMap<String, String>>,
    /// Output only.
    /// The version identifier of the Cloud Function. Each deployment attempt
    /// results in a new version of a function being created.
    #[serde(rename="versionId")]
    pub version_id: Option<String>,
    /// The name of the function (as defined in source code) that will be
    /// executed. Defaults to the resource name suffix, if not specified. For
    /// backward compatibility, if function with given name is not found, then the
    /// system will try to use function named "function".
    /// For Node.js this is name of a function exported by the module specified
    /// in `source_location`.
    #[serde(rename="entryPoint")]
    pub entry_point: Option<String>,
    /// The VPC Network that this cloud function can connect to. It can be
    /// either the fully-qualified URI, or the short name of the network resource.
    /// If the short network name is used, the network must belong to the same
    /// project. Otherwise, it must belong to a project within the same
    /// organization. The format of this field is either
    /// `projects/{project}/global/networks/{network}` or `{network}`, where
    /// {project} is a project id where the network is defined, and {network} is
    /// the short name of the network.
    /// 
    /// See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
    /// more information on connecting Cloud projects.
    /// 
    /// This feature is currently in alpha, available only for whitelisted users.
    pub network: Option<String>,
    /// A user-defined name of the function. Function names must be unique
    /// globally and match pattern `projects/*/locations/*/functions/*`
    pub name: Option<String>,
    /// The amount of memory in MB available for a function.
    /// Defaults to 256MB.
    #[serde(rename="availableMemoryMb")]
    pub available_memory_mb: Option<i32>,
    /// **Beta Feature**
    /// 
    /// Environment variables that shall be available during function execution.
    #[serde(rename="environmentVariables")]
    pub environment_variables: Option<HashMap<String, String>>,
    /// The Google Cloud Storage signed URL used for source uploading, generated
    /// by google.cloud.functions.v1.GenerateUploadUrl
    #[serde(rename="sourceUploadUrl")]
    pub source_upload_url: Option<String>,
    /// Output only. The email of the function's service account.
    #[serde(rename="serviceAccountEmail")]
    pub service_account_email: Option<String>,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    pub timeout: Option<String>,
    /// The runtime in which the function is going to run. If empty, defaults to
    /// Node.js 6.
    pub runtime: Option<String>,
}

impl RequestValue for CloudFunction {}
impl ResponseResult for CloudFunction {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the `CloudFunctions` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudfunctions1 as cloudfunctions1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudfunctions1::CloudFunctions;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
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



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `CloudFunctions` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudfunctions1 as cloudfunctions1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use cloudfunctions1::CloudFunctions;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_functions_call(...)`, `locations_functions_create(...)`, `locations_functions_delete(...)`, `locations_functions_generate_download_url(...)`, `locations_functions_generate_upload_url(...)`, `locations_functions_get(...)`, `locations_functions_list(...)`, `locations_functions_patch(...)` and `locations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, C, A> {
        ProjectLocationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a function with the given name from the specified project. If the
    /// given function is used by some trigger, the trigger will be updated to
    /// remove this function.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the function which should be deleted.
    pub fn locations_functions_delete(&self, name: &str) -> ProjectLocationFunctionDeleteCall<'a, C, A> {
        ProjectLocationFunctionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a function with the given name from the requested project.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the function which details should be obtained.
    pub fn locations_functions_get(&self, name: &str) -> ProjectLocationFunctionGetCall<'a, C, A> {
        ProjectLocationFunctionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new function. If a function with the given name already exists in
    /// the specified project, the long running operation will return
    /// `ALREADY_EXISTS` error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `location` - The project and location in which the function should be created, specified
    ///                in the format `projects/*/locations/*`
    pub fn locations_functions_create(&self, request: CloudFunction, location: &str) -> ProjectLocationFunctionCreateCall<'a, C, A> {
        ProjectLocationFunctionCreateCall {
            hub: self.hub,
            _request: request,
            _location: location.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates existing function.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A user-defined name of the function. Function names must be unique
    ///            globally and match pattern `projects/*/locations/*/functions/*`
    pub fn locations_functions_patch(&self, request: CloudFunction, name: &str) -> ProjectLocationFunctionPatchCall<'a, C, A> {
        ProjectLocationFunctionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a signed URL for uploading a function source code.
    /// For more information about the signed URL usage see:
    /// https://cloud.google.com/storage/docs/access-control/signed-urls.
    /// Once the function source code upload is complete, the used signed
    /// URL should be provided in CreateFunction or UpdateFunction request
    /// as a reference to the function source code.
    /// 
    /// When uploading source code to the generated signed URL, please follow
    /// these restrictions:
    /// 
    /// * Source file type should be a zip file.
    /// * Source file size should not exceed 100MB limit.
    /// 
    /// When making a HTTP PUT request, these two headers need to be specified:
    /// 
    /// * `content-type: application/zip`
    /// * `x-goog-content-length-range: 0,104857600`
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The project and location in which the Google Cloud Storage signed URL
    ///              should be generated, specified in the format `projects/*/locations/*`.
    pub fn locations_functions_generate_upload_url(&self, request: GenerateUploadUrlRequest, parent: &str) -> ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A> {
        ProjectLocationFunctionGenerateUploadUrlCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Invokes synchronously deployed function. To be used for testing, very
    /// limited traffic allowed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the function to be called.
    pub fn locations_functions_call(&self, request: CallFunctionRequest, name: &str) -> ProjectLocationFunctionCallCall<'a, C, A> {
        ProjectLocationFunctionCallCall {
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
    /// Returns a signed URL for downloading deployed function source code.
    /// The URL is only valid for a limited period and should be used within
    /// minutes after generation.
    /// For more information about the signed URL usage see:
    /// https://cloud.google.com/storage/docs/access-control/signed-urls
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of function for which source code Google Cloud Storage signed
    ///            URL should be generated.
    pub fn locations_functions_generate_download_url(&self, request: GenerateDownloadUrlRequest, name: &str) -> ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A> {
        ProjectLocationFunctionGenerateDownloadUrlCall {
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
    /// Returns a list of functions that belong to the requested project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The project and location from which the function should be listed,
    ///              specified in the format `projects/*/locations/*`
    ///              If you want to list functions in all locations, use "-" in place of a
    ///              location.
    pub fn locations_functions_list(&self, parent: &str) -> ProjectLocationFunctionListCall<'a, C, A> {
        ProjectLocationFunctionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().get("name")
///              .doit();
/// # }
/// ```
pub struct OperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for OperationGetCall<'a, C, A> {}

impl<'a, C, A> OperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.operations.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
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

    hub: &'a CloudFunctions<C, A>,
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
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, ListOperationsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.operations.list",
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
                return Err(Error::FieldClash(field));
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
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


/// Lists information about the supported locations for this service.
///
/// A builder for the *locations.list* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_list("name")
///              .page_token("aliquyam")
///              .page_size(-66)
///              .filter("no")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationListCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, ListLocationsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "name", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}/locations";
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    /// The resource that owns the locations collection, if applicable.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The standard list page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectLocationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The standard list page size.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectLocationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The standard list filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> ProjectLocationListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a function with the given name from the specified project. If the
/// given function is used by some trigger, the trigger will be updated to
/// remove this function.
///
/// A builder for the *locations.functions.delete* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_delete("name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionDeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = HeaderValue::from_str(&format!("Authorization: Bearer {}", token.access_token)).unwrap();
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::DELETE;
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    /// The name of the function which should be deleted.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationFunctionDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a function with the given name from the requested project.
///
/// A builder for the *locations.functions.get* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_get("name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionGetCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, CloudFunction)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    /// The name of the function which details should be obtained.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationFunctionGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new function. If a function with the given name already exists in
/// the specified project, the long running operation will return
/// `ALREADY_EXISTS` error.
///
/// A builder for the *locations.functions.create* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// use cloudfunctions1::CloudFunction;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CloudFunction::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_create(req, "location")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _request: CloudFunction,
    _location: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionCreateCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionCreateCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("location", self._location.to_string()));
        for &field in ["alt", "location"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+location}/functions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+location}", "location")].iter() {
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
            for param_name in ["location"].iter() {
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CloudFunction) -> ProjectLocationFunctionCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project and location in which the function should be created, specified
    /// in the format `projects/*/locations/*`
    ///
    /// Sets the *location* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn location(mut self, new_value: &str) -> ProjectLocationFunctionCreateCall<'a, C, A> {
        self._location = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates existing function.
///
/// A builder for the *locations.functions.patch* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// use cloudfunctions1::CloudFunction;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CloudFunction::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_patch(req, "name")
///              .update_mask("diam")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _request: CloudFunction,
    _name: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionPatchCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionPatchCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, Operation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "name", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
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
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = HeaderValue::from_str(&format!("Authorization: Bearer {}", token.access_token)).unwrap();
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_fut: hyper::client::ResponseFuture = {
                let mut req = hyper::Request::new(hyper::Body::from(""));
                *req.method_mut() = hyper::Method::PATCH;
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CloudFunction) -> ProjectLocationFunctionPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// A user-defined name of the function. Function names must be unique
    /// globally and match pattern `projects/*/locations/*/functions/*`
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationFunctionPatchCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// Required list of fields to be updated in this request.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> ProjectLocationFunctionPatchCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a signed URL for uploading a function source code.
/// For more information about the signed URL usage see:
/// https://cloud.google.com/storage/docs/access-control/signed-urls.
/// Once the function source code upload is complete, the used signed
/// URL should be provided in CreateFunction or UpdateFunction request
/// as a reference to the function source code.
/// 
/// When uploading source code to the generated signed URL, please follow
/// these restrictions:
/// 
/// * Source file type should be a zip file.
/// * Source file size should not exceed 100MB limit.
/// 
/// When making a HTTP PUT request, these two headers need to be specified:
/// 
/// * `content-type: application/zip`
/// * `x-goog-content-length-range: 0,104857600`
///
/// A builder for the *locations.functions.generateUploadUrl* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// use cloudfunctions1::GenerateUploadUrlRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateUploadUrlRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_generate_upload_url(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _request: GenerateUploadUrlRequest,
    _parent: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, GenerateUploadUrlResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.generateUploadUrl",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/functions:generateUploadUrl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
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
            for param_name in ["parent"].iter() {
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GenerateUploadUrlRequest) -> ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The project and location in which the Google Cloud Storage signed URL
    /// should be generated, specified in the format `projects/*/locations/*`.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionGenerateUploadUrlCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Invokes synchronously deployed function. To be used for testing, very
/// limited traffic allowed.
///
/// A builder for the *locations.functions.call* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// use cloudfunctions1::CallFunctionRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CallFunctionRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_call(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionCallCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _request: CallFunctionRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionCallCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionCallCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, CallFunctionResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.call",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:call";
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CallFunctionRequest) -> ProjectLocationFunctionCallCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The name of the function to be called.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationFunctionCallCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionCallCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionCallCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionCallCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a signed URL for downloading deployed function source code.
/// The URL is only valid for a limited period and should be used within
/// minutes after generation.
/// For more information about the signed URL usage see:
/// https://cloud.google.com/storage/docs/access-control/signed-urls
///
/// A builder for the *locations.functions.generateDownloadUrl* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// use cloudfunctions1::GenerateDownloadUrlRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GenerateDownloadUrlRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_generate_download_url(req, "name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _request: GenerateDownloadUrlRequest,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, GenerateDownloadUrlResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.generateDownloadUrl",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}:generateDownloadUrl";
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GenerateDownloadUrlRequest) -> ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The name of function for which source code Google Cloud Storage signed
    /// URL should be generated.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionGenerateDownloadUrlCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a list of functions that belong to the requested project.
///
/// A builder for the *locations.functions.list* method supported by a *project* resource.
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
/// # extern crate google_cloudfunctions1 as cloudfunctions1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use cloudfunctions1::CloudFunctions;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudFunctions::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_functions_list("parent")
///              .page_token("aliquyam")
///              .page_size(-9)
///              .doit();
/// # }
/// ```
pub struct ProjectLocationFunctionListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudFunctions<C, A>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationFunctionListCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationFunctionListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, ListFunctionsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "cloudfunctions.projects.locations.functions.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/functions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
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
            for param_name in ["parent"].iter() {
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
                            return Err(Error::MissingToken(err))
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
            use futures::{ Future, Stream };
            use std::io::Write;
            let new_fut = req_fut.map(|mut _res| {
                ()
                /*
                if !res.status().is_success() {
                    let json_err = cmn::read_to_string(&res).unwrap();
                    if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                        json::from_str(&json_err).ok(),
                                                        json::from_str(&json_err).ok()) {
                        sleep(d);
                    }
                    dlg.finished(false);
                    return match json::from_str::<ErrorResponse>(&json_err){
                        Err(_) => Err(Error::Failure(res)),
                        Ok(serr) => Err(Error::BadRequest(serr))
                    }
                }
                let result_value = {
                    let json_response = cmn::read_to_string(&res).unwrap();

                    match json::from_str(&json_response) {
                        Ok(decoded) => (res, decoded),
                        Err(err) => {
                            dlg.response_json_decode_error(&json_response, &err);
                            return Err(Error::JsonDecodeError(json_response, err));
                        }
                    }
                };

                dlg.finished(true);
                return Ok(result_value)
            */
            }).map_err(|_err| {
                /*
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                */
                ()
            });
            hyper::rt::run(new_fut);
        }
    }


    /// The project and location from which the function should be listed,
    /// specified in the format `projects/*/locations/*`
    /// If you want to list functions in all locations, use "-" in place of a
    /// location.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationFunctionListCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The value returned by the last
    /// `ListFunctionsResponse`; indicates that
    /// this is a continuation of a prior `ListFunctions` call, and that the
    /// system should return the next page of data.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectLocationFunctionListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of functions to return per call.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectLocationFunctionListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationFunctionListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationFunctionListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationFunctionListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


