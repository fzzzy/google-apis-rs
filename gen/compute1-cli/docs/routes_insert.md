Creates a Route resource in the specified project using the data included in the request.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> routes insert ...`
# Required Scalar Argument
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Route:
  creation-timestamp: string
  description: string
  dest-range: string
  id: string
  kind: string
  name: string
  network: string
  next-hop-gateway: string
  next-hop-instance: string
  next-hop-ip: string
  next-hop-network: string
  next-hop-peering: string
  next-hop-vpn-tunnel: string
  priority: integer
  self-link: string
  tags: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    creation-timestamp=at`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `description=amet`
    - An optional description of this resource. Provide this property when you create the resource.
* `dest-range=sed`
    - The destination range of outgoing packets that this route applies to. Only IPv4 is supported.
* `id=sit`
    - [Output Only] The unique identifier for the resource. This identifier is defined by the server.
* `kind=sanctus`
    - [Output Only] Type of this resource. Always compute#routes for Route resources.
* `name=dolores`
    - Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
* `network=eos`
    - Fully-qualified URL of the network that this route applies to.
* `next-hop-gateway=invidunt`
    - The URL to a gateway that should handle matching packets. You can only specify the internet gateway using a full or partial valid URL:  projects/&lt;project-id&gt;/global/gateways/default-internet-gateway
* `next-hop-instance=amet`
    - The URL to an instance that should handle matching packets. You can specify this as a full or partial URL. For example:
        https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/
* `next-hop-ip=eirmod`
    - The network IP address of an instance that should handle matching packets. Only IPv4 is supported.
* `next-hop-network=consetetur`
    - The URL of the local network if it should handle matching packets.
* `next-hop-peering=sea`
    - [Output Only] The network peering name that should handle matching packets, which should conform to RFC1035.
* `next-hop-vpn-tunnel=erat`
    - The URL to a VpnTunnel that should handle matching packets.
* `priority=69`
    - The priority of this route. Priority is used to break ties in cases where there is more than one matching route of equal prefix length. In the case of two routes with equal prefix length, the one with the lowest-numbered priority value wins. Default value is 1000. Valid range is 0 through 65535.
* `self-link=justo`
    - [Output Only] Server-defined fully-qualified URL for this resource.
* `tags=ea`
    - A list of instance tags to which this route applies.
    - Each invocation of this argument appends the given value to the array.


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
