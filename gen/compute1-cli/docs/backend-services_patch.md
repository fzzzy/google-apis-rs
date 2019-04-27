Patches the specified BackendService resource with the data included in the request. There are several restrictions and guidelines to keep in mind when updating a backend service. Read  Restrictions and Guidelines for more information. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> backend-services patch ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
* **&lt;backend-service&gt;** *(string)*
    - Name of the BackendService resource to patch.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
BackendService:
  affinity-cookie-ttl-sec: integer
  cdn-policy:
    cache-key-policy:
      include-host: boolean
      include-protocol: boolean
      include-query-string: boolean
      query-string-blacklist: [string]
      query-string-whitelist: [string]
    signed-url-cache-max-age-sec: string
    signed-url-key-names: [string]
  connection-draining:
    draining-timeout-sec: integer
  creation-timestamp: string
  description: string
  enable-cdn: boolean
  fingerprint: string
  health-checks: [string]
  iap:
    enabled: boolean
    oauth2-client-id: string
    oauth2-client-secret: string
    oauth2-client-secret-sha256: string
  id: string
  kind: string
  load-balancing-scheme: string
  name: string
  port: integer
  port-name: string
  protocol: string
  region: string
  security-policy: string
  self-link: string
  session-affinity: string
  timeout-sec: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    affinity-cookie-ttl-sec=78`
    - Lifetime of cookies in seconds if session_affinity is GENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts only until the end of the browser session (or equivalent). The maximum allowed value for TTL is one day.
        
        When the load balancing scheme is INTERNAL, this field is not used.
* `cdn-policy.cache-key-policy    include-host=true`
    - If true, requests to different hosts will be cached separately.
* `include-protocol=false`
    - If true, http and https requests will be cached separately.
* `include-query-string=false`
    - If true, include query string parameters in the cache key according to query_string_whitelist and query_string_blacklist. If neither is set, the entire query string will be included. If false, the query string will be excluded from the cache key entirely.
* `query-string-blacklist=est`
    - Names of query string parameters to exclude in cache keys. All other parameters will be included. Either specify query_string_whitelist or query_string_blacklist, not both. &#39;&amp;&#39; and &#39;=&#39; will be percent encoded and not treated as delimiters.
    - Each invocation of this argument appends the given value to the array.
* `query-string-whitelist=clita`
    - Names of query string parameters to include in cache keys. All other parameters will be excluded. Either specify query_string_whitelist or query_string_blacklist, not both. &#39;&amp;&#39; and &#39;=&#39; will be percent encoded and not treated as delimiters.
    - Each invocation of this argument appends the given value to the array.

* `..    signed-url-cache-max-age-sec=invidunt`
    - Maximum number of seconds the response to a signed URL request will be considered fresh. After this time period, the response will be revalidated before being served. Defaults to 1hr (3600s). When serving responses to signed URL requests, Cloud CDN will internally behave as though all responses from this backend had a ?Cache-Control: public, max-age=[TTL]? header, regardless of any existing Cache-Control header. The actual headers served in responses will not be altered.
* `signed-url-key-names=ut`
    - [Output Only] Names of the keys for signing request URLs.
    - Each invocation of this argument appends the given value to the array.

* `..connection-draining    draining-timeout-sec=82`
    - Time for which instance will be drained (not accept new connections, but still work to finish started).

* `..    creation-timestamp=eos`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `description=voluptua.`
    - An optional description of this resource. Provide this property when you create the resource.
* `enable-cdn=true`
    - If true, enable Cloud CDN for this BackendService.
        
        When the load balancing scheme is INTERNAL, this field is not used.
* `fingerprint=sed`
    - Fingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking. This field will be ignored when inserting a BackendService. An up-to-date fingerprint must be provided in order to update the BackendService.
        
        To see the latest fingerprint, make a get() request to retrieve a BackendService.
* `health-checks=aliquyam`
    - The list of URLs to the HttpHealthCheck or HttpsHealthCheck resource for health checking this BackendService. Currently at most one health check can be specified, and a health check is required for Compute Engine backend services. A health check must not be specified for App Engine backend and Cloud Function backend.
        
        For internal load balancing, a URL to a HealthCheck resource must be specified instead.
    - Each invocation of this argument appends the given value to the array.
* `iap    enabled=false`
    - No description provided.
* `oauth2-client-id=ea`
    - No description provided.
* `oauth2-client-secret=et`
    - No description provided.
* `oauth2-client-secret-sha256=dolor`
    - [Output Only] SHA256 hash value for the field oauth2_client_secret above.

* `..    id=diam`
    - [Output Only] The unique identifier for the resource. This identifier is defined by the server.
* `kind=kasd`
    - [Output Only] Type of resource. Always compute#backendService for backend services.
* `load-balancing-scheme=invidunt`
    - Indicates whether the backend service will be used with internal or external load balancing. A backend service created for one type of load balancing cannot be used with the other. Possible values are INTERNAL and EXTERNAL.
* `name=rebum.`
    - Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
* `port=50`
    - Deprecated in favor of portName. The TCP port to connect on the backend. The default value is 80.
        
        This cannot be used for internal load balancing.
* `port-name=clita`
    - Name of backend port. The same name should appear in the instance groups referenced by this service. Required when the load balancing scheme is EXTERNAL.
        
        When the load balancing scheme is INTERNAL, this field is not used.
* `protocol=invidunt`
    - The protocol this BackendService uses to communicate with backends.
        
        Possible values are HTTP, HTTPS, TCP, and SSL. The default is HTTP.
        
        For internal load balancing, the possible values are TCP and UDP, and the default is TCP.
* `region=eirmod`
    - [Output Only] URL of the region where the regional backend service resides. This field is not applicable to global backend services. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
* `security-policy=at`
    - [Output Only] The resource URL for the security policy associated with this backend service.
* `self-link=consetetur`
    - [Output Only] Server-defined URL for the resource.
* `session-affinity=et`
    - Type of session affinity to use. The default is NONE.
        
        When the load balancing scheme is EXTERNAL, can be NONE, CLIENT_IP, or GENERATED_COOKIE.
        
        When the load balancing scheme is INTERNAL, can be NONE, CLIENT_IP, CLIENT_IP_PROTO, or CLIENT_IP_PORT_PROTO.
        
        When the protocol is UDP, this field is not used.
* `timeout-sec=21`
    - How many seconds to wait for the backend before considering it a failed request. Default is 30 seconds.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p request-id=string**
    - An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed.
        
        For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments.
        
        The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
