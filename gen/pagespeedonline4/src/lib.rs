// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *pagespeedonline* crate version *1.0.8+20180822*, where *20180822* is the exact revision of the *pagespeedonline:v4* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.8*.
//! 
//! Everything else about the *pagespeedonline* *v4* API can be found at the
//! [official documentation site](https://developers.google.com/speed/docs/insights/v4/getting-started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/pagespeedonline4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Pagespeedonline.html) ... 
//! 
//! * pagespeedapi
//!  * [*runpagespeed*](struct.PagespeedapiRunpagespeedCall.html)
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
//! * **[Hub](struct.Pagespeedonline.html)**
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
//! let r = hub.pagespeedapi().runpagespeed(...).doit()
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
//! google-pagespeedonline4 = "*"
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
//! extern crate google_pagespeedonline4 as pagespeedonline4;
//! use pagespeedonline4::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use pagespeedonline4::Pagespeedonline;
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
//! let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.pagespeedapi().runpagespeed("url")
//!              .utm_source("amet.")
//!              .utm_campaign("erat")
//!              .strategy("labore")
//!              .snapshots(true)
//!              .screenshot(false)
//!              .add_rule("dolores")
//!              .locale("gubergren")
//!              .filter_third_party_resources(false)
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




// ########
// HUB ###
// ######

/// Central instance to access all Pagespeedonline related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_pagespeedonline4 as pagespeedonline4;
/// use pagespeedonline4::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use pagespeedonline4::Pagespeedonline;
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
/// let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("ea")
///              .utm_campaign("no")
///              .strategy("justo")
///              .snapshots(true)
///              .screenshot(true)
///              .add_rule("et")
///              .locale("diam")
///              .filter_third_party_resources(false)
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
pub struct Pagespeedonline<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Pagespeedonline<C, A> {}

impl<'a, C, A> Pagespeedonline<C, A>
    where  C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Pagespeedonline<C, A> {
        Pagespeedonline {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.8".to_string(),
            _base_url: "https://www.googleapis.com/pagespeedonline/v4/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn pagespeedapi(&'a self) -> PagespeedapiMethods<'a, C, A> {
        PagespeedapiMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.8`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/pagespeedonline/v4/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
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
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4 {
    /// List of arguments for the format string.
    pub args: Option<Vec<PagespeedApiFormatStringV4Args>>,
    /// A localized format string with {{FOO}} placeholders, where 'FOO' is the key of the argument whose value should be substituted. For HYPERLINK arguments, the format string will instead contain {{BEGIN_FOO}} and {{END_FOO}} for the argument with key 'FOO'.
    pub format: Option<String>,
}

impl Part for PagespeedApiFormatStringV4 {}


/// Metrics of end users' page loading experience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4LoadingExperience {
    /// no description provided
    pub metrics: Option<HashMap<String, PagespeedApiPagespeedResponseV4LoadingExperienceMetrics>>,
    /// The url, pattern or origin which the metrics are on.
    pub id: Option<String>,
    /// no description provided
    pub overall_category: Option<String>,
    /// no description provided
    pub initial_url: Option<String>,
}

impl NestedType for PagespeedApiPagespeedResponseV4LoadingExperience {}
impl Part for PagespeedApiPagespeedResponseV4LoadingExperience {}


/// List of arguments for the format string.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4Args {
    /// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
    pub rects: Option<Vec<PagespeedApiFormatStringV4ArgsRects>>,
    /// The placeholder key for this arg, as a string.
    pub key: Option<String>,
    /// Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, DURATION, VERBATIM_STRING, PERCENTAGE, HYPERLINK, or SNAPSHOT_RECT.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
    pub secondary_rects: Option<Vec<PagespeedApiFormatStringV4ArgsSecondaryRects>>,
    /// Argument value, as a localized string.
    pub value: Option<String>,
}

impl NestedType for PagespeedApiFormatStringV4Args {}
impl Part for PagespeedApiFormatStringV4Args {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [runpagespeed pagespeedapi](struct.PagespeedapiRunpagespeedCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4 {
    /// Kind of result.
    pub kind: Option<String>,
    /// The captcha verify result
    #[serde(rename="captchaResult")]
    pub captcha_result: Option<String>,
    /// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename="formattedResults")]
    pub formatted_results: Option<PagespeedApiPagespeedResponseV4FormattedResults>,
    /// Base64-encoded screenshot of the page that was analyzed.
    pub screenshot: Option<PagespeedApiImageV4>,
    /// Title of the page, as displayed in the browser's title bar.
    pub title: Option<String>,
    /// A map with one entry for each rule group in these results.
    #[serde(rename="ruleGroups")]
    pub rule_groups: Option<HashMap<String, PagespeedApiPagespeedResponseV4RuleGroups>>,
    /// Additional base64-encoded screenshots of the page, in various partial render states.
    pub snapshots: Option<Vec<PagespeedApiImageV4>>,
    /// Metrics of end users' page loading experience.
    #[serde(rename="loadingExperience")]
    pub loading_experience: Option<PagespeedApiPagespeedResponseV4LoadingExperience>,
    /// The version of PageSpeed used to generate these results.
    pub version: Option<PagespeedApiPagespeedResponseV4Version>,
    /// Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error.
    #[serde(rename="responseCode")]
    pub response_code: Option<i32>,
    /// List of rules that were specified in the request, but which the server did not know how to instantiate.
    #[serde(rename="invalidRules")]
    pub invalid_rules: Option<Vec<String>>,
    /// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
    #[serde(rename="pageStats")]
    pub page_stats: Option<PagespeedApiPagespeedResponseV4PageStats>,
    /// Canonicalized and final URL for the document, after following page redirects (if any).
    pub id: Option<String>,
}

impl ResponseResult for PagespeedApiPagespeedResponseV4 {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions {
    /// no description provided
    pub max: Option<i32>,
    /// no description provided
    pub proportion: Option<f64>,
    /// no description provided
    pub min: Option<i32>,
}

impl NestedType for PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions {}
impl Part for PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions {}


/// The enum-like identifier for this rule. For instance "EnableKeepAlive" or "AvoidCssImport". Not localized.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResults {
    /// Whether this rule is in 'beta'. Rules in beta are new rules that are being tested, which do not impact the overall score.
    pub beta: Option<bool>,
    /// List of rule groups that this rule belongs to. Each entry in the list is one of "SPEED", "USABILITY", or "SECURITY".
    pub groups: Option<Vec<String>>,
    /// The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal.
    #[serde(rename="ruleImpact")]
    pub rule_impact: Option<f64>,
    /// Localized name of the rule, intended for presentation to a user.
    #[serde(rename="localizedRuleName")]
    pub localized_rule_name: Option<String>,
    /// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
    #[serde(rename="urlBlocks")]
    pub url_blocks: Option<Vec<PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks>>,
    /// A brief summary description for the rule, indicating at a high level what should be done to follow the rule and what benefit can be gained by doing so.
    pub summary: Option<PagespeedApiFormatStringV4>,
}

impl NestedType for PagespeedApiPagespeedResponseV4FormattedResultsRuleResults {}
impl Part for PagespeedApiPagespeedResponseV4FormattedResultsRuleResults {}


/// Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResults {
    /// The locale of the formattedResults, e.g. "en_US".
    pub locale: Option<String>,
    /// Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server.
    #[serde(rename="ruleResults")]
    pub rule_results: Option<HashMap<String, PagespeedApiPagespeedResponseV4FormattedResultsRuleResults>>,
}

impl NestedType for PagespeedApiPagespeedResponseV4FormattedResults {}
impl Part for PagespeedApiPagespeedResponseV4FormattedResults {}


/// List of entries that provide information about URLs in the url block. Optional.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls {
    /// List of entries that provide additional details about a single URL. Optional.
    pub details: Option<Vec<PagespeedApiFormatStringV4>>,
    /// A format string that gives information about the URL, and a list of arguments for that format string.
    pub result: Option<PagespeedApiFormatStringV4>,
}

impl NestedType for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls {}
impl Part for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls {}


/// The version of PageSpeed used to generate these results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4Version {
    /// The major version number of PageSpeed used to generate these results.
    pub major: Option<i32>,
    /// The minor version number of PageSpeed used to generate these results.
    pub minor: Option<i32>,
}

impl NestedType for PagespeedApiPagespeedResponseV4Version {}
impl Part for PagespeedApiPagespeedResponseV4Version {}


/// The name of this rule group: one of "SPEED", "USABILITY", or "SECURITY".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4RuleGroups {
    /// The score (0-100) for this rule group, which indicates how much better a page could be in that category (e.g. how much faster, or how much more usable, or how much more secure). A high score indicates little room for improvement, while a lower score indicates more room for improvement.
    pub score: Option<i32>,
    /// no description provided
    pub pass: Option<bool>,
}

impl NestedType for PagespeedApiPagespeedResponseV4RuleGroups {}
impl Part for PagespeedApiPagespeedResponseV4RuleGroups {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiImageV4 {
    /// Width of screenshot in pixels.
    pub width: Option<i32>,
    /// Unique string key, if any, identifying this image.
    pub key: Option<String>,
    /// no description provided
    pub page_rect: Option<PagespeedApiImageV4PageRect>,
    /// Image data base64 encoded.
    pub data: Option<String>,
    /// Mime type of image data (e.g. "image/jpeg").
    pub mime_type: Option<String>,
    /// Height of screenshot in pixels.
    pub height: Option<i32>,
}

impl Part for PagespeedApiImageV4 {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiImageV4PageRect {
    /// no description provided
    pub width: Option<i32>,
    /// no description provided
    pub top: Option<i32>,
    /// no description provided
    pub height: Option<i32>,
    /// no description provided
    pub left: Option<i32>,
}

impl NestedType for PagespeedApiImageV4PageRect {}
impl Part for PagespeedApiImageV4PageRect {}


/// The screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments. If this is absent for a SNAPSHOT_RECT argument, it means that that argument refers to the entire snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4ArgsRects {
    /// no description provided
    pub width: Option<i32>,
    /// no description provided
    pub top: Option<i32>,
    /// no description provided
    pub height: Option<i32>,
    /// no description provided
    pub left: Option<i32>,
}

impl NestedType for PagespeedApiFormatStringV4ArgsRects {}
impl Part for PagespeedApiFormatStringV4ArgsRects {}


/// Secondary screen rectangles being referred to, with dimensions measured in CSS pixels. This is only ever used for SNAPSHOT_RECT arguments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiFormatStringV4ArgsSecondaryRects {
    /// no description provided
    pub width: Option<i32>,
    /// no description provided
    pub top: Option<i32>,
    /// no description provided
    pub height: Option<i32>,
    /// no description provided
    pub left: Option<i32>,
}

impl NestedType for PagespeedApiFormatStringV4ArgsSecondaryRects {}
impl Part for PagespeedApiFormatStringV4ArgsSecondaryRects {}


/// The type of the metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4LoadingExperienceMetrics {
    /// no description provided
    pub category: Option<String>,
    /// no description provided
    pub distributions: Option<Vec<PagespeedApiPagespeedResponseV4LoadingExperienceMetricsDistributions>>,
    /// no description provided
    pub median: Option<i32>,
}

impl NestedType for PagespeedApiPagespeedResponseV4LoadingExperienceMetrics {}
impl Part for PagespeedApiPagespeedResponseV4LoadingExperienceMetrics {}


/// Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4PageStats {
    /// Total size of all request bytes sent by the page.
    #[serde(rename="totalRequestBytes")]
    pub total_request_bytes: Option<String>,
    /// Number of CSS resources referenced by the page.
    #[serde(rename="numberCssResources")]
    pub number_css_resources: Option<i32>,
    /// The needed round trips to load render blocking resources
    #[serde(rename="numRenderBlockingRoundTrips")]
    pub num_render_blocking_round_trips: Option<i32>,
    /// Number of HTTP resources loaded by the page.
    #[serde(rename="numberResources")]
    pub number_resources: Option<i32>,
    /// Number of response bytes for image resources on the page.
    #[serde(rename="imageResponseBytes")]
    pub image_response_bytes: Option<String>,
    /// Number of uncompressed response bytes for JS resources on the page.
    #[serde(rename="javascriptResponseBytes")]
    pub javascript_response_bytes: Option<String>,
    /// List of transient fetch failure urls.
    #[serde(rename="transientFetchFailureUrls")]
    pub transient_fetch_failure_urls: Option<Vec<String>>,
    /// Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page.
    #[serde(rename="textResponseBytes")]
    pub text_response_bytes: Option<String>,
    /// Number of over-the-wire bytes, uses the default gzip compression strategy as an estimation.
    #[serde(rename="overTheWireResponseBytes")]
    pub over_the_wire_response_bytes: Option<String>,
    /// Number of response bytes for flash resources on the page.
    #[serde(rename="flashResponseBytes")]
    pub flash_response_bytes: Option<String>,
    /// The needed round trips to load the full page
    #[serde(rename="numTotalRoundTrips")]
    pub num_total_round_trips: Option<i32>,
    /// Content management system (CMS) used for the page.
    pub cms: Option<String>,
    /// Number of transient-failed resources.
    #[serde(rename="numberTransientFetchFailureResources")]
    pub number_transient_fetch_failure_resources: Option<i32>,
    /// Number of uncompressed response bytes for CSS resources on the page.
    #[serde(rename="cssResponseBytes")]
    pub css_response_bytes: Option<String>,
    /// Number of response bytes for other resources on the page.
    #[serde(rename="otherResponseBytes")]
    pub other_response_bytes: Option<String>,
    /// List of roboted urls.
    #[serde(rename="robotedUrls")]
    pub roboted_urls: Option<Vec<String>>,
    /// Number of unique hosts referenced by the page.
    #[serde(rename="numberHosts")]
    pub number_hosts: Option<i32>,
    /// Number of static (i.e. cacheable) resources on the page.
    #[serde(rename="numberStaticResources")]
    pub number_static_resources: Option<i32>,
    /// Number of uncompressed response bytes for the main HTML document and all iframes on the page.
    #[serde(rename="htmlResponseBytes")]
    pub html_response_bytes: Option<String>,
    /// Number of roboted resources.
    #[serde(rename="numberRobotedResources")]
    pub number_roboted_resources: Option<i32>,
    /// Number of JavaScript resources referenced by the page.
    #[serde(rename="numberJsResources")]
    pub number_js_resources: Option<i32>,
}

impl NestedType for PagespeedApiPagespeedResponseV4PageStats {}
impl Part for PagespeedApiPagespeedResponseV4PageStats {}


/// List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks {
    /// Heading to be displayed with the list of URLs.
    pub header: Option<PagespeedApiFormatStringV4>,
    /// List of entries that provide information about URLs in the url block. Optional.
    pub urls: Option<Vec<PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocksUrls>>,
}

impl NestedType for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks {}
impl Part for PagespeedApiPagespeedResponseV4FormattedResultsRuleResultsUrlBlocks {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *pagespeedapi* resources.
/// It is not used directly, but through the `Pagespeedonline` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_pagespeedonline4 as pagespeedonline4;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use pagespeedonline4::Pagespeedonline;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `runpagespeed(...)`
/// // to build up your call.
/// let rb = hub.pagespeedapi();
/// # }
/// ```
pub struct PagespeedapiMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Pagespeedonline<C, A>,
}

impl<'a, C, A> MethodsBuilder for PagespeedapiMethods<'a, C, A> {}

impl<'a, C, A> PagespeedapiMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
    /// 
    /// # Arguments
    ///
    /// * `url` - The URL to fetch and analyze
    pub fn runpagespeed(&self, url: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        PagespeedapiRunpagespeedCall {
            hub: self.hub,
            _url: url.to_string(),
            _utm_source: Default::default(),
            _utm_campaign: Default::default(),
            _strategy: Default::default(),
            _snapshots: Default::default(),
            _screenshot: Default::default(),
            _rule: Default::default(),
            _locale: Default::default(),
            _filter_third_party_resources: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
///
/// A builder for the *runpagespeed* method supported by a *pagespeedapi* resource.
/// It is not used directly, but through a `PagespeedapiMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_pagespeedonline4 as pagespeedonline4;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use pagespeedonline4::Pagespeedonline;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Pagespeedonline::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("et")
///              .utm_campaign("duo")
///              .strategy("aliquyam")
///              .snapshots(true)
///              .screenshot(false)
///              .add_rule("eos")
///              .locale("erat")
///              .filter_third_party_resources(false)
///              .doit();
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Pagespeedonline<C, A>,
    _url: String,
    _utm_source: Option<String>,
    _utm_campaign: Option<String>,
    _strategy: Option<String>,
    _snapshots: Option<bool>,
    _screenshot: Option<bool>,
    _rule: Vec<String>,
    _locale: Option<String>,
    _filter_third_party_resources: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for PagespeedapiRunpagespeedCall<'a, C, A> {}

impl<'a, C, A> PagespeedapiRunpagespeedCall<'a, C, A> where C: BorrowMut<hyper::Client<hyper::client::HttpConnector, hyper::Body>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::Response<hyper::Body>, PagespeedApiPagespeedResponseV4)> {
        use std::io::{Read, Seek};
        use hyper::header::{HeaderMap, HeaderValue, CONTENT_RANGE, CONTENT_TYPE, CONTENT_LENGTH, USER_AGENT, AUTHORIZATION};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("url", self._url.to_string()));
        if let Some(value) = self._utm_source {
            params.push(("utm_source", value.to_string()));
        }
        if let Some(value) = self._utm_campaign {
            params.push(("utm_campaign", value.to_string()));
        }
        if let Some(value) = self._strategy {
            params.push(("strategy", value.to_string()));
        }
        if let Some(value) = self._snapshots {
            params.push(("snapshots", value.to_string()));
        }
        if let Some(value) = self._screenshot {
            params.push(("screenshot", value.to_string()));
        }
        if self._rule.len() > 0 {
            for f in self._rule.iter() {
                params.push(("rule", f.to_string()));
            }
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._filter_third_party_resources {
            params.push(("filter_third_party_resources", value.to_string()));
        }
        for &field in ["alt", "url", "utm_source", "utm_campaign", "strategy", "snapshots", "screenshot", "rule", "locale", "filter_third_party_resources"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "runPagespeed";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        use http::Uri;
        let url = url.parse::<Uri>().unwrap();



        loop {
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


    /// The URL to fetch and analyze
    ///
    /// Sets the *url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._url = new_value.to_string();
        self
    }
    /// Campaign source for analytics.
    ///
    /// Sets the *utm_source* query property to the given value.
    pub fn utm_source(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._utm_source = Some(new_value.to_string());
        self
    }
    /// Campaign name for analytics.
    ///
    /// Sets the *utm_campaign* query property to the given value.
    pub fn utm_campaign(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._utm_campaign = Some(new_value.to_string());
        self
    }
    /// The analysis strategy (desktop or mobile) to use, and desktop is the default
    ///
    /// Sets the *strategy* query property to the given value.
    pub fn strategy(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._strategy = Some(new_value.to_string());
        self
    }
    /// Indicates if binary data containing snapshot images should be included
    ///
    /// Sets the *snapshots* query property to the given value.
    pub fn snapshots(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._snapshots = Some(new_value);
        self
    }
    /// Indicates if binary data containing a screenshot should be included
    ///
    /// Sets the *screenshot* query property to the given value.
    pub fn screenshot(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._screenshot = Some(new_value);
        self
    }
    /// A PageSpeed rule to run; if none are given, all rules are run
    ///
    /// Append the given value to the *rule* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_rule(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._rule.push(new_value.to_string());
        self
    }
    /// The locale used to localize formatted results
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Indicates if third party resources should be filtered out before PageSpeed analysis.
    ///
    /// Sets the *filter_third_party_resources* query property to the given value.
    pub fn filter_third_party_resources(mut self, new_value: bool) -> PagespeedapiRunpagespeedCall<'a, C, A> {
        self._filter_third_party_resources = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PagespeedapiRunpagespeedCall<'a, C, A> {
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
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


