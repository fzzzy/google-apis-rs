Creates a new uptime check configuration.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/monitoring*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `monitoring3 --scope <scope> projects uptime-check-configs-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The project in which to create the uptime check. The format  is projects/[PROJECT_ID].
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
UptimeCheckConfig:
  display-name: string
  http-check:
    auth-info:
      password: string
      username: string
    headers: { string: string }
    mask-headers: boolean
    path: string
    port: integer
    use-ssl: boolean
  is-internal: boolean
  monitored-resource:
    labels: { string: string }
    type: string
  name: string
  period: string
  resource-group:
    group-id: string
    resource-type: string
  selected-regions: [string]
  tcp-check:
    port: integer
  timeout: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    display-name=consetetur`
    - A human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced. Required.
* `http-check.auth-info    password=amet.`
    - The password to authenticate.
* `username=voluptua.`
    - The username to authenticate.

* `..    headers=key=lorem`
    - The list of headers to send as part of the uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described at https://www.w3.org/Protocols/rfc2616/rfc2616.txt (page 31). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100.
    - the value will be associated with the given `key`
* `mask-headers=true`
    - Boolean specifiying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if mask_headers is set to True then the headers will be obscured with ******.
* `path=justo`
    - The path to the page to run the check against. Will be combined with the host (specified within the MonitoredResource) and port to construct the full URL. Optional (defaults to &#34;/&#34;).
* `port=49`
    - The port to the page to run the check against. Will be combined with host (specified within the MonitoredResource) and path to construct the full URL. Optional (defaults to 80 without SSL, or 443 with SSL).
* `use-ssl=true`
    - If true, use HTTPS instead of HTTP to run the check.

* `..    is-internal=true`
    - Denotes whether this is a check that egresses from InternalCheckers.
* `monitored-resource    labels=key=rebum.`
    - Required. Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels &#34;project_id&#34;, &#34;instance_id&#34;, and &#34;zone&#34;.
    - the value will be associated with the given `key`
* `type=consetetur`
    - Required. The monitored resource type. This field must match the type field of a MonitoredResourceDescriptor object. For example, the type of a Compute Engine VM instance is gce_instance. For a list of types, see Monitoring resource types and Logging resource types.

* `..    name=sadipscing`
    - A unique resource name for this UptimeCheckConfig. The format is:projects/[PROJECT_ID]/uptimeCheckConfigs/[UPTIME_CHECK_ID].This field should be omitted when creating the uptime check configuration; on create, the resource name is assigned by the server and included in the response.
* `period=vero`
    - How often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s.
* `resource-group    group-id=sadipscing`
    - The group of resources being monitored. Should be only the group_id, not projects/&lt;project_id&gt;/groups/&lt;group_id&gt;.
* `resource-type=invidunt`
    - The resource type of the group members.

* `..    selected-regions=consetetur`
    - The list of regions from which the check will be run. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions.
    - Each invocation of this argument appends the given value to the array.
* `tcp-check    port=17`
    - The port to the page to run the check against. Will be combined with host (specified within the MonitoredResource) to construct the full URL. Required.

* `..    timeout=duo`
    - The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). Required.


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
# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
