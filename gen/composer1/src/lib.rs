// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Cloud Composer* crate version *1.0.8+20181001*, where *20181001* is the exact revision of the *composer:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *Cloud Composer* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/composer/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/composer1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.CloudComposer.html) ... 
//! 
//! * projects
//!  * [*locations environments create*](struct.ProjectLocationEnvironmentCreateCall.html), [*locations environments delete*](struct.ProjectLocationEnvironmentDeleteCall.html), [*locations environments get*](struct.ProjectLocationEnvironmentGetCall.html), [*locations environments list*](struct.ProjectLocationEnvironmentListCall.html), [*locations environments patch*](struct.ProjectLocationEnvironmentPatchCall.html), [*locations operations delete*](struct.ProjectLocationOperationDeleteCall.html), [*locations operations get*](struct.ProjectLocationOperationGetCall.html) and [*locations operations list*](struct.ProjectLocationOperationListCall.html)
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
//! * **[Hub](struct.CloudComposer.html)**
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
//! let r = hub.projects().locations_operations_get(...).doit()
//! let r = hub.projects().locations_environments_delete(...).doit()
//! let r = hub.projects().locations_environments_create(...).doit()
//! let r = hub.projects().locations_environments_patch(...).doit()
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
//! google-composer1 = "*"
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
//! extern crate google_composer1 as composer1;
//! use composer1::Environment;
//! use composer1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use composer1::CloudComposer;
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
//! let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Environment::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_environments_patch(req, "name")
//!              .update_mask("sed")
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

/// Central instance to access all CloudComposer related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_composer1 as composer1;
/// use composer1::Environment;
/// use composer1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use composer1::CloudComposer;
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
/// let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Environment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_environments_patch(req, "name")
///              .update_mask("dolores")
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
pub struct CloudComposer<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for CloudComposer<C, A> {}

impl<'a, C, A> CloudComposer<C, A>
    where  C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> CloudComposer<C, A> {
        CloudComposer {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://composer.googleapis.com/".to_string(),
            _root_url: "https://composer.googleapis.com/".to_string(),
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
    /// It defaults to `https://composer.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://composer.googleapis.com/`.
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


/// Configuration information for an environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Output only.
    /// The Cloud Storage prefix of the DAGs for this environment. Although Cloud
    /// Storage objects reside in a flat namespace, a hierarchical file tree
    /// can be simulated using "/"-delimited object name prefixes. DAG objects for
    /// this environment reside in a simulated directory with the given prefix.
    #[serde(rename="dagGcsPrefix")]
    pub dag_gcs_prefix: Option<String>,
    /// The configuration settings for software inside the environment.
    #[serde(rename="softwareConfig")]
    pub software_config: Option<SoftwareConfig>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[serde(rename="nodeConfig")]
    pub node_config: Option<NodeConfig>,
    /// Output only.
    /// The URI of the Apache Airflow Web UI hosted within this environment (see
    /// [Airflow web interface](/composer/docs/how-to/accessing/airflow-web-interface)).
    #[serde(rename="airflowUri")]
    pub airflow_uri: Option<String>,
    /// Output only.
    /// The Kubernetes Engine cluster used to run this environment.
    #[serde(rename="gkeCluster")]
    pub gke_cluster: Option<String>,
    /// The number of nodes in the Kubernetes Engine cluster that will be
    /// used to run this environment.
    #[serde(rename="nodeCount")]
    pub node_count: Option<i32>,
}

impl Part for EnvironmentConfig {}


/// Specifies the selection and configuration of software inside the environment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareConfig {
    /// Output only.
    /// The version of the software running in the environment.
    /// This encapsulates both the version of Cloud Composer functionality and the
    /// version of Apache Airflow. It must match the regular expression
    /// `composer-[0-9]+\.[0-9]+(\.[0-9]+)?-airflow-[0-9]+\.[0-9]+(\.[0-9]+.*)?`.
    /// 
    /// The Cloud Composer portion of the version is a
    /// [semantic version](https://semver.org). The portion of the image version
    /// following _airflow-_ is an official Apache Airflow repository
    /// [release name](https://github.com/apache/incubator-airflow/releases).
    /// 
    /// See also [Release Notes](/composer/docs/release-notes).
    #[serde(rename="imageVersion")]
    pub image_version: Option<String>,
    /// Optional. Apache Airflow configuration properties to override.
    /// 
    /// Property keys contain the section and property names, separated by a hyphen,
    /// for example "core-dags_are_paused_at_creation". Section names must not
    /// contain hyphens ("-"), opening square brackets ("["),  or closing square
    /// brackets ("]"). The property name must not be empty and must not contain
    /// an equals sign ("=") or semicolon (";"). Section and property names must
    /// not contain a period ("."). Apache Airflow configuration property names
    /// must be written in [snake_case](https://en.wikipedia.org/wiki/Snake_case).
    /// Property values can contain any character, and can be written in any
    /// lower/upper case format.
    /// 
    /// Certain Apache Airflow configuration property values are
    /// [blacklisted](/composer/docs/how-to/managing/setting-airflow-configurations#airflow_configuration_blacklists),
    /// and cannot be overridden.
    #[serde(rename="airflowConfigOverrides")]
    pub airflow_config_overrides: Option<HashMap<String, String>>,
    /// Optional. Additional environment variables to provide to the Apache Airflow
    /// scheduler, worker, and webserver processes.
    /// 
    /// Environment variable names must match the regular expression
    /// `a-zA-Z_*`. They cannot specify Apache Airflow
    /// software configuration overrides (they cannot match the regular expression
    /// `AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+`), and they cannot match any of the
    /// following reserved names:
    /// 
    /// * `AIRFLOW_HOME`
    /// * `C_FORCE_ROOT`
    /// * `CONTAINER_NAME`
    /// * `DAGS_FOLDER`
    /// * `GCP_PROJECT`
    /// * `GCS_BUCKET`
    /// * `GKE_CLUSTER_NAME`
    /// * `SQL_DATABASE`
    /// * `SQL_INSTANCE`
    /// * `SQL_PASSWORD`
    /// * `SQL_PROJECT`
    /// * `SQL_REGION`
    /// * `SQL_USER`
    #[serde(rename="envVariables")]
    pub env_variables: Option<HashMap<String, String>>,
    /// Optional. Custom Python Package Index (PyPI) packages to be installed in
    /// the environment.
    /// 
    /// Keys refer to the lowercase package name such as "numpy"
    /// and values are the lowercase extras and version specifier such as
    /// "==1.12.0", "[devel,gcp_api]", or "[devel]>=1.8.2, <1.9.2". To specify a
    /// package without pinning it to a version specifier, use the empty string as
    /// the value.
    #[serde(rename="pypiPackages")]
    pub pypi_packages: Option<HashMap<String, String>>,
}

impl Part for SoftwareConfig {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](struct.ProjectLocationOperationListCall.html) (response)
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


/// An environment for running orchestration tasks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments patch projects](struct.ProjectLocationEnvironmentPatchCall.html) (request)
/// * [locations environments create projects](struct.ProjectLocationEnvironmentCreateCall.html) (request)
/// * [locations environments get projects](struct.ProjectLocationEnvironmentGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// Output only.
    /// The time at which this environment was last modified.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// Output only.
    /// The UUID (Universally Unique IDentifier) associated with this environment.
    /// This value is generated when the environment is created.
    pub uuid: Option<String>,
    /// The resource name of the environment, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub name: Option<String>,
    /// Optional. User-defined labels for this environment.
    /// The labels map can contain no more than 64 entries. Entries of the labels
    /// map are UTF8 strings that comply with the following restrictions:
    /// 
    /// * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62}
    /// * Values must conform to regexp:  [\p{Ll}\p{Lo}\p{N}_-]{0,63}
    /// * Both keys and values are additionally constrained to be <= 128 bytes in
    /// size.
    pub labels: Option<HashMap<String, String>>,
    /// Configuration parameters for this environment.
    pub config: Option<EnvironmentConfig>,
    /// The current state of the environment.
    pub state: Option<String>,
    /// Output only.
    /// The time at which this environment was created.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
}

impl RequestValue for Environment {}
impl ResponseResult for Environment {}


/// The environments in a project and location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations environments list projects](struct.ProjectLocationEnvironmentListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListEnvironmentsResponse {
    /// The page token used to query for the next page if one exists.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of environments returned by a ListEnvironmentsRequest.
    pub environments: Option<Vec<Environment>>,
}

impl ResponseResult for ListEnvironmentsResponse {}


/// The configuration information for the Kubernetes Engine nodes running
/// the Apache Airflow software.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Optional. The Compute Engine
    /// [machine type](/compute/docs/machine-types) used for cluster instances,
    /// specified as a
    /// [relative resource name](/apis/design/resource_names#relative_resource_name).
    /// For example:
    /// "projects/{projectId}/zones/{zoneId}/machineTypes/{machineTypeId}".
    /// 
    /// The `machineType` must belong to the enclosing environment's project and
    /// location. If both this field and `nodeConfig.location` are specified,
    /// this `machineType` must belong to the `nodeConfig.location`; if both are
    /// unspecified, the service will pick a zone in the Compute Engine region
    /// corresponding to the Cloud Composer location, and propagate that choice to
    /// both fields. If exactly one of this field and `nodeConfig.location` is
    /// specified, the location information from the specified field will be
    /// propagated to the unspecified field.
    /// 
    /// If this field is unspecified, the `machineTypeId` defaults
    /// to "n1-standard-1".
    #[serde(rename="machineType")]
    pub machine_type: Option<String>,
    /// Optional. The Compute Engine network to be used for machine
    /// communications, specified as a
    /// [relative resource name](/apis/design/resource_names#relative_resource_name).
    /// For example: "projects/{projectId}/global/networks/{networkId}".
    /// 
    /// [Shared VPC](/vpc/docs/shared-vpc) is not currently supported. The
    /// network must belong to the environment's project. If unspecified, the
    /// "default" network ID in the environment's project is used.  If a
    /// [Custom Subnet Network](/vpc/docs/vpc#vpc_networks_and_subnets)
    /// is provided, `nodeConfig.subnetwork` must also be provided.
    pub network: Option<String>,
    /// Optional. The list of instance tags applied to all node VMs. Tags are used
    /// to identify valid sources or targets for network firewalls. Each tag within
    /// the list must comply with [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
    /// Cannot be updated.
    pub tags: Option<Vec<String>>,
    /// Optional. The Google Cloud Platform Service Account to be used by the node
    /// VMs. If a service account is not specified, the "default" Compute Engine
    /// service account is used. Cannot be updated.
    #[serde(rename="serviceAccount")]
    pub service_account: Option<String>,
    /// Optional. The set of Google API scopes to be made available on all
    /// node VMs. If `oauth_scopes` is empty, defaults to
    /// ["https://www.googleapis.com/auth/cloud-platform"]. Cannot be updated.
    #[serde(rename="oauthScopes")]
    pub oauth_scopes: Option<Vec<String>>,
    /// Optional. The disk size in GB used for node VMs. Minimum size is 20GB.
    /// If unspecified, defaults to 100GB. Cannot be updated.
    #[serde(rename="diskSizeGb")]
    pub disk_size_gb: Option<i32>,
    /// Optional. The Compute Engine [zone](/compute/docs/regions-zones) in which
    /// to deploy the VMs used to run the Apache Airflow software, specified as a
    /// [relative resource name](/apis/design/resource_names#relative_resource_name).
    /// For example: "projects/{projectId}/zones/{zoneId}".
    /// 
    /// This `location` must belong to the enclosing environment's project and
    /// location. If both this field and `nodeConfig.machineType` are specified,
    /// `nodeConfig.machineType` must belong to this `location`; if both are
    /// unspecified, the service will pick a zone in the Compute Engine region
    /// corresponding to the Cloud Composer location, and propagate that choice to
    /// both fields. If only one field (`location` or `nodeConfig.machineType`) is
    /// specified, the location information from the specified field will be
    /// propagated to the unspecified field.
    pub location: Option<String>,
    /// Optional. The Compute Engine subnetwork to be used for machine
    /// communications, specified as a
    /// [relative resource name](/apis/design/resource_names#relative_resource_name).
    /// For example:
    /// "projects/{projectId}/regions/{regionId}/subnetworks/{subnetworkId}"
    /// 
    /// If a subnetwork is provided, `nodeConfig.network` must also be provided,
    /// and the subnetwork must belong to the enclosing environment's project and
    /// location.
    pub subnetwork: Option<String>,
}

impl Part for NodeConfig {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get projects](struct.ProjectLocationOperationGetCall.html) (response)
/// * [locations environments delete projects](struct.ProjectLocationEnvironmentDeleteCall.html) (response)
/// * [locations environments create projects](struct.ProjectLocationEnvironmentCreateCall.html) (response)
/// * [locations environments patch projects](struct.ProjectLocationEnvironmentPatchCall.html) (response)
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

impl ResponseResult for Operation {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations delete projects](struct.ProjectLocationOperationDeleteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `CloudComposer` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_composer1 as composer1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use composer1::CloudComposer;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_environments_create(...)`, `locations_environments_delete(...)`, `locations_environments_get(...)`, `locations_environments_list(...)`, `locations_environments_patch(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)` and `locations_operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
}

impl<'a, C, A> MethodsBuilder for ProjectMethods<'a, C, A> {}

impl<'a, C, A> ProjectMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the environment to update, in the form:
    ///            "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_patch(&self, request: Environment, name: &str) -> ProjectLocationEnvironmentPatchCall<'a, C, A> {
        ProjectLocationEnvironmentPatchCall {
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
    /// List environments.
    /// 
    /// # Arguments
    ///
    /// * `parent` - List environments in the given project and location, in the form:
    ///              "projects/{projectId}/locations/{locationId}"
    pub fn locations_environments_list(&self, parent: &str) -> ProjectLocationEnvironmentListCall<'a, C, A> {
        ProjectLocationEnvironmentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a new environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The parent must be of the form "projects/{projectId}/locations/{locationId}".
    pub fn locations_environments_create(&self, request: Environment, parent: &str) -> ProjectLocationEnvironmentCreateCall<'a, C, A> {
        ProjectLocationEnvironmentCreateCall {
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
    /// Delete an environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - The environment to delete, in the form:
    ///            "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_delete(&self, name: &str) -> ProjectLocationEnvironmentDeleteCall<'a, C, A> {
        ProjectLocationEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an existing environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the environment to get, in the form:
    ///            "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_get(&self, name: &str) -> ProjectLocationEnvironmentGetCall<'a, C, A> {
        ProjectLocationEnvironmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, C, A> {
        ProjectLocationOperationGetCall {
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
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, C, A> {
        ProjectLocationOperationListCall {
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
    /// Deletes a long-running operation. This method indicates that the client is
    /// no longer interested in the operation result. It does not cancel the
    /// operation. If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn locations_operations_delete(&self, name: &str) -> ProjectLocationOperationDeleteCall<'a, C, A> {
        ProjectLocationOperationDeleteCall {
            hub: self.hub,
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

/// Update an environment.
///
/// A builder for the *locations.environments.patch* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// use composer1::Environment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Environment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_environments_patch(req, "name")
///              .update_mask("accusam")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationEnvironmentPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _request: Environment,
    _name: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationEnvironmentPatchCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationEnvironmentPatchCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "composer.projects.locations.environments.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "name", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

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
                            return Box::new(futures::future::err(Error::MissingToken(err)));
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
            });
            return Box::new(final_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Environment) -> ProjectLocationEnvironmentPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The relative resource name of the environment to update, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationEnvironmentPatchCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// Required. A comma-separated list of paths, relative to `Environment`, of
    /// fields to update.
    /// For example, to set the version of scikit-learn to install in the
    /// environment to 0.19.0 and to remove an existing installation of
    /// numpy, the `updateMask` parameter would include the following two
    /// `paths` values: "config.softwareConfig.pypiPackages.scikit-learn" and
    /// "config.softwareConfig.pypiPackages.numpy". The included patch
    /// environment would specify the scikit-learn version as follows:
    /// 
    ///     {
    ///       "config":{
    ///         "softwareConfig":{
    ///           "pypiPackages":{
    ///             "scikit-learn":"==0.19.0"
    ///           }
    ///         }
    ///       }
    ///     }
    /// 
    /// Note that in the above example, any existing PyPI packages
    /// other than scikit-learn and numpy will be unaffected.
    /// 
    /// Only one update type may be included in a single request's `updateMask`.
    /// For example, one cannot update both the PyPI packages and
    /// labels in the same request. However, it is possible to update multiple
    /// members of a map field simultaneously in the same request. For example,
    /// to set the labels "label1" and "label2" while clearing "label3" (assuming
    /// it already exists), one can
    /// provide the paths "labels.label1", "labels.label2", and "labels.label3"
    /// and populate the patch environment as follows:
    /// 
    ///     {
    ///       "labels":{
    ///         "label1":"new-label1-value"
    ///         "label2":"new-label2-value"
    ///       }
    ///     }
    /// 
    /// Note that in the above example, any existing labels that are not
    /// included in the `updateMask` will be unaffected.
    /// 
    /// It is also possible to replace an entire map field by providing the
    /// map field's path in the `updateMask`. The new value of the field will
    /// be that which is provided in the patch environment. For example, to
    /// delete all pre-existing user-specified PyPI packages and
    /// install botocore at version 1.7.14, the `updateMask` would contain
    /// the path "config.softwareConfig.pypiPackages", and
    /// the patch environment would be the following:
    /// 
    ///     {
    ///       "config":{
    ///         "softwareConfig":{
    ///           "pypiPackages":{
    ///             "botocore":"==1.7.14"
    ///           }
    ///         }
    ///       }
    ///     }
    /// 
    /// **Note:** Only the following fields can be updated:
    /// 
    ///  <table>
    ///  <tbody>
    ///  <tr>
    ///  <td><strong>Mask</strong></td>
    ///  <td><strong>Purpose</strong></td>
    ///  </tr>
    ///  <tr>
    ///  <td>config.softwareConfig.pypiPackages
    ///  </td>
    ///  <td>Replace all custom custom PyPI packages. If a replacement
    ///  package map is not included in `environment`, all custom
    ///  PyPI packages are cleared. It is an error to provide both this mask and a
    ///  mask specifying an individual package.</td>
    ///  </tr>
    ///  <tr>
    ///  <td>config.softwareConfig.pypiPackages.<var>packagename</var></td>
    ///  <td>Update the custom PyPI package <var>packagename</var>,
    ///  preserving other packages. To delete the package, include it in
    ///  `updateMask`, and omit the mapping for it in
    ///  `environment.config.softwareConfig.pypiPackages`. It is an error
    ///  to provide both a mask of this form and the
    ///  "config.softwareConfig.pypiPackages" mask.</td>
    ///  </tr>
    ///  <tr>
    ///  <td>labels</td>
    ///  <td>Replace all environment labels. If a replacement labels map is not
    ///  included in `environment`, all labels are cleared. It is an error to
    ///  provide both this mask and a mask specifying one or more individual
    ///  labels.</td>
    ///  </tr>
    ///  <tr>
    ///  <td>labels.<var>labelName</var></td>
    ///  <td>Set the label named <var>labelName</var>, while preserving other
    ///  labels. To delete the label, include it in `updateMask` and omit its
    ///  mapping in `environment.labels`. It is an error to provide both a
    ///  mask of this form and the "labels" mask.</td>
    ///  </tr>
    ///  <tr>
    ///  <td>config.nodeCount</td>
    ///  <td>Horizontally scale the number of nodes in the environment. An integer
    ///  greater than or equal to 3 must be provided in the `config.nodeCount` field.
    ///  </td>
    ///  </tr>
    ///  <tr>
    ///  <td>config.softwareConfig.airflowConfigOverrides</td>
    ///  <td>Replace all Apache Airflow config overrides. If a replacement config
    ///  overrides map is not included in `environment`, all config overrides
    ///  are cleared.
    ///  It is an error to provide both this mask and a mask specifying one or
    ///  more individual config overrides.</td>
    ///  </tr>
    ///  <tr>
    ///  <td>config.softwareConfig.airflowConfigOverrides.<var>section</var>-<var>name
    ///  </var></td>
    ///  <td>Override the Apache Airflow config property <var>name</var> in the
    ///  section named <var>section</var>, preserving other properties. To delete
    ///  the property override, include it in `updateMask` and omit its mapping
    ///  in `environment.config.softwareConfig.airflowConfigOverrides`.
    ///  It is an error to provide both a mask of this form and the
    ///  "config.softwareConfig.airflowConfigOverrides" mask.</td>
    ///  </tr>
    ///  <tr>
    ///  <td>config.softwareConfig.envVariables</td>
    ///  <td>Replace all environment variables. If a replacement environment
    ///  variable map is not included in `environment`, all custom environment
    ///  variables  are cleared.
    ///  It is an error to provide both this mask and a mask specifying one or
    ///  more individual environment variables.</td>
    ///  </tr>
    ///  </tbody>
    ///  </table>
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> ProjectLocationEnvironmentPatchCall<'a, C, A> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationEnvironmentPatchCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationEnvironmentPatchCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationEnvironmentPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List environments.
///
/// A builder for the *locations.environments.list* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_environments_list("parent")
///              .page_token("justo")
///              .page_size(-1)
///              .doit();
/// # }
/// ```
pub struct ProjectLocationEnvironmentListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationEnvironmentListCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationEnvironmentListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, ListEnvironmentsResponse)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "composer.projects.locations.environments.list",
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
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/environments";
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
            });
            return Box::new(final_fut);
        }
    }


    /// List environments in the given project and location, in the form:
    /// "projects/{projectId}/locations/{locationId}"
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationEnvironmentListCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The next_page_token value returned from a previous List request, if any.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectLocationEnvironmentListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of environments to return.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectLocationEnvironmentListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationEnvironmentListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationEnvironmentListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationEnvironmentListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Create a new environment.
///
/// A builder for the *locations.environments.create* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// use composer1::Environment;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Environment::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_environments_create(req, "parent")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationEnvironmentCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _request: Environment,
    _parent: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationEnvironmentCreateCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationEnvironmentCreateCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "composer.projects.locations.environments.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/environments";
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
            });
            return Box::new(final_fut);
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Environment) -> ProjectLocationEnvironmentCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The parent must be of the form "projects/{projectId}/locations/{locationId}".
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectLocationEnvironmentCreateCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationEnvironmentCreateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationEnvironmentCreateCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationEnvironmentCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Delete an environment.
///
/// A builder for the *locations.environments.delete* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_environments_delete("name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationEnvironmentDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationEnvironmentDeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationEnvironmentDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "composer.projects.locations.environments.delete",
                               http_method: hyper::Method::DELETE });
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
                            return Box::new(futures::future::err(Error::MissingToken(err)));
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
            });
            return Box::new(final_fut);
        }
    }


    /// The environment to delete, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationEnvironmentDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationEnvironmentDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationEnvironmentDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationEnvironmentDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get an existing environment.
///
/// A builder for the *locations.environments.get* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_environments_get("name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationEnvironmentGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationEnvironmentGetCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationEnvironmentGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, Environment)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "composer.projects.locations.environments.get",
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
            });
            return Box::new(final_fut);
        }
    }


    /// The resource name of the environment to get, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationEnvironmentGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationEnvironmentGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationEnvironmentGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationEnvironmentGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the latest state of a long-running operation.  Clients can use this
/// method to poll the operation result at intervals as recommended by the API
/// service.
///
/// A builder for the *locations.operations.get* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_operations_get("name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationOperationGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationOperationGetCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationOperationGetCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


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
        dlg.begin(MethodInfo { id: "composer.projects.locations.operations.get",
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
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
    pub fn name(mut self, new_value: &str) -> ProjectLocationOperationGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationOperationGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationOperationGetCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationOperationGetCall<'a, C, A>
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
/// A builder for the *locations.operations.list* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_operations_list("name")
///              .page_token("gubergren")
///              .page_size(-95)
///              .filter("aliquyam")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationOperationListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationOperationListCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationOperationListCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, ListOperationsResponse)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "composer.projects.locations.operations.list",
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
                return Box::new(futures::future::err(Error::FieldClash(field)));

            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}/operations";
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
            });
            return Box::new(final_fut);
        }
    }


    /// The name of the operation's parent resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationOperationListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The standard list page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectLocationOperationListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The standard list page size.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectLocationOperationListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The standard list filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> ProjectLocationOperationListCall<'a, C, A> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationOperationListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationOperationListCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationOperationListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a long-running operation. This method indicates that the client is
/// no longer interested in the operation result. It does not cancel the
/// operation. If the server doesn't support this method, it returns
/// `google.rpc.Code.UNIMPLEMENTED`.
///
/// A builder for the *locations.operations.delete* method supported by a *project* resource.
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
/// # extern crate google_composer1 as composer1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use composer1::CloudComposer;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = CloudComposer::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().locations_operations_delete("name")
///              .doit();
/// # }
/// ```
pub struct ProjectLocationOperationDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a CloudComposer<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ProjectLocationOperationDeleteCall<'a, C, A> {}

impl<'a, C, A> ProjectLocationOperationDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Box<Future<Item = Result<(hyper::Response<hyper::Body>, Empty)>, Error = cmn::Error>> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "composer.projects.locations.operations.delete",
                               http_method: hyper::Method::DELETE });
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
                            return Box::new(futures::future::err(Error::MissingToken(err)));
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
                return Box::new(futures::future::ok(result_value))
/*
            }).map_err(|_err| {
                if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                    sleep(d);
                }
                dlg.finished(false);
                ()
*/
            });
            return Box::new(final_fut);
        }
    }


    /// The name of the operation resource to be deleted.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectLocationOperationDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ProjectLocationOperationDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectLocationOperationDeleteCall<'a, C, A>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectLocationOperationDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


