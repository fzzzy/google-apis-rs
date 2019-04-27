Creates a HealthCheck resource in the specified project using the data included in the request.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> health-checks insert ...`
# Required Scalar Argument
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
HealthCheck:
  check-interval-sec: integer
  creation-timestamp: string
  description: string
  healthy-threshold: integer
  http-health-check:
    host: string
    port: integer
    port-name: string
    proxy-header: string
    request-path: string
    response: string
  https-health-check:
    host: string
    port: integer
    port-name: string
    proxy-header: string
    request-path: string
    response: string
  id: string
  kind: string
  name: string
  self-link: string
  ssl-health-check:
    port: integer
    port-name: string
    proxy-header: string
    request: string
    response: string
  tcp-health-check:
    port: integer
    port-name: string
    proxy-header: string
    request: string
    response: string
  timeout-sec: integer
  type: string
  unhealthy-threshold: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    check-interval-sec=92`
    - How often (in seconds) to send a health check. The default value is 5 seconds.
* `creation-timestamp=vero`
    - [Output Only] Creation timestamp in 3339 text format.
* `description=dolores`
    - An optional description of this resource. Provide this property when you create the resource.
* `healthy-threshold=69`
    - A so-far unhealthy instance will be marked healthy after this many consecutive successes. The default value is 2.
* `http-health-check    host=ut`
    - The value of the host header in the HTTP health check request. If left empty (default value), the IP on behalf of which this health check is performed will be used.
* `port=54`
    - The TCP port number for the health check request. The default value is 80. Valid values are 1 through 65535.
* `port-name=sed`
    - Port name as defined in InstanceGroup#NamedPort#name. If both port and port_name are defined, port takes precedence.
* `proxy-header=sit`
    - Specifies the type of proxy header to append before sending data to the backend, either NONE or PROXY_V1. The default is NONE.
* `request-path=sit`
    - The request path of the HTTP health check request. The default value is /.
* `response=dolores`
    - The string to match anywhere in the first 1024 bytes of the response body. If left empty (the default value), the status code determines health. The response data can only be ASCII.

* `..https-health-check    host=et`
    - The value of the host header in the HTTPS health check request. If left empty (default value), the IP on behalf of which this health check is performed will be used.
* `port=94`
    - The TCP port number for the health check request. The default value is 443. Valid values are 1 through 65535.
* `port-name=takimata`
    - Port name as defined in InstanceGroup#NamedPort#name. If both port and port_name are defined, port takes precedence.
* `proxy-header=kasd`
    - Specifies the type of proxy header to append before sending data to the backend, either NONE or PROXY_V1. The default is NONE.
* `request-path=ut`
    - The request path of the HTTPS health check request. The default value is /.
* `response=sadipscing`
    - The string to match anywhere in the first 1024 bytes of the response body. If left empty (the default value), the status code determines health. The response data can only be ASCII.

* `..    id=et`
    - [Output Only] The unique identifier for the resource. This identifier is defined by the server.
* `kind=clita`
    - Type of the resource.
* `name=ipsum`
    - Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
* `self-link=dolor`
    - [Output Only] Server-defined URL for the resource.
* `ssl-health-check    port=7`
    - The TCP port number for the health check request. The default value is 443. Valid values are 1 through 65535.
* `port-name=magna`
    - Port name as defined in InstanceGroup#NamedPort#name. If both port and port_name are defined, port takes precedence.
* `proxy-header=aliquyam`
    - Specifies the type of proxy header to append before sending data to the backend, either NONE or PROXY_V1. The default is NONE.
* `request=kasd`
    - The application data to send once the SSL connection has been established (default value is empty). If both request and response are empty, the connection establishment alone will indicate health. The request data can only be ASCII.
* `response=duo`
    - The bytes to match against the beginning of the response data. If left empty (the default value), any response will indicate health. The response data can only be ASCII.

* `..tcp-health-check    port=80`
    - The TCP port number for the health check request. The default value is 80. Valid values are 1 through 65535.
* `port-name=sit`
    - Port name as defined in InstanceGroup#NamedPort#name. If both port and port_name are defined, port takes precedence.
* `proxy-header=eirmod`
    - Specifies the type of proxy header to append before sending data to the backend, either NONE or PROXY_V1. The default is NONE.
* `request=no`
    - The application data to send once the TCP connection has been established (default value is empty). If both request and response are empty, the connection establishment alone will indicate health. The request data can only be ASCII.
* `response=lorem`
    - The bytes to match against the beginning of the response data. If left empty (the default value), any response will indicate health. The response data can only be ASCII.

* `..    timeout-sec=72`
    - How long (in seconds) to wait before claiming failure. The default value is 5 seconds. It is invalid for timeoutSec to have greater value than checkIntervalSec.
* `type=tempor`
    - Specifies the type of the healthCheck, either TCP, SSL, HTTP or HTTPS. If not specified, the default is TCP. Exactly one of the protocol-specific health check field must be specified, which must match type field.
* `unhealthy-threshold=38`
    - A so-far healthy instance will be marked unhealthy after this many consecutive failures. The default value is 2.


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
