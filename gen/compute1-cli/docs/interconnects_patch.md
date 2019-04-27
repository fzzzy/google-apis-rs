Updates the specified interconnect with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> interconnects patch ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
* **&lt;interconnect&gt;** *(string)*
    - Name of the interconnect to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Interconnect:
  admin-enabled: boolean
  creation-timestamp: string
  customer-name: string
  description: string
  google-ip-address: string
  google-reference-id: string
  id: string
  interconnect-attachments: [string]
  interconnect-type: string
  kind: string
  link-type: string
  location: string
  name: string
  noc-contact-email: string
  operational-status: string
  peer-ip-address: string
  provisioned-link-count: integer
  requested-link-count: integer
  self-link: string
  state: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    admin-enabled=true`
    - Administrative status of the interconnect. When this is set to true, the Interconnect is functional and can carry traffic. When set to false, no packets can be carried over the interconnect and no BGP routes are exchanged over it. By default, the status is set to true.
* `creation-timestamp=gubergren`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `customer-name=magna`
    - Customer name, to put in the Letter of Authorization as the party authorized to request a crossconnect.
* `description=at`
    - An optional description of this resource. Provide this property when you create the resource.
* `google-ip-address=duo`
    - [Output Only] IP address configured on the Google side of the Interconnect link. This can be used only for ping tests.
* `google-reference-id=sed`
    - [Output Only] Google reference ID; to be used when raising support tickets with Google or otherwise to debug backend connectivity issues.
* `id=consetetur`
    - [Output Only] The unique identifier for the resource. This identifier is defined by the server.
* `interconnect-attachments=et`
    - [Output Only] A list of the URLs of all InterconnectAttachments configured to use this Interconnect.
    - Each invocation of this argument appends the given value to the array.
* `interconnect-type=duo`
    - Type of interconnect. Note that &#34;IT_PRIVATE&#34; has been deprecated in favor of &#34;DEDICATED&#34;
* `kind=erat`
    - [Output Only] Type of the resource. Always compute#interconnect for interconnects.
* `link-type=sanctus`
    - Type of link requested. This field indicates speed of each of the links in the bundle, not the entire bundle. Only 10G per link is allowed for a dedicated interconnect. Options: Ethernet_10G_LR
* `location=kasd`
    - URL of the InterconnectLocation object that represents where this connection is to be provisioned.
* `name=takimata`
    - Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
* `noc-contact-email=dolor`
    - Email address to contact the customer NOC for operations and maintenance notifications regarding this Interconnect. If specified, this will be used for notifications in addition to all other forms described, such as Stackdriver logs alerting and Cloud Notifications.
* `operational-status=nonumy`
    - [Output Only] The current status of whether or not this Interconnect is functional.
* `peer-ip-address=sanctus`
    - [Output Only] IP address configured on the customer side of the Interconnect link. The customer should configure this IP address during turnup when prompted by Google NOC. This can be used only for ping tests.
* `provisioned-link-count=32`
    - [Output Only] Number of links actually provisioned in this interconnect.
* `requested-link-count=9`
    - Target number of physical links in the link bundle, as requested by the customer.
* `self-link=justo`
    - [Output Only] Server-defined URL for the resource.
* `state=accusam`
    - [Output Only] The current state of whether or not this Interconnect is functional.


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
