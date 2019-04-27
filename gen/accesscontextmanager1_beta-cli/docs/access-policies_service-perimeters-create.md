Create an Service Perimeter. The
longrunning operation from this RPC will have a successful status once the
Service Perimeter has
propagated to long-lasting storage. Service Perimeters containing
errors will result in an error response for the first error encountered.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `accesscontextmanager1-beta --scope <scope> access-policies service-perimeters-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. Resource name for the access policy which owns this Service
        Perimeter.
        
        Format: `accessPolicies/{policy_id}`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ServicePerimeter:
  create-time: string
  description: string
  name: string
  perimeter-type: string
  status:
    access-levels: [string]
    resources: [string]
    restricted-services: [string]
    unrestricted-services: [string]
  title: string
  update-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=justo`
    - Output only. Time the `ServicePerimeter` was created in UTC.
* `description=et`
    - Description of the `ServicePerimeter` and its use. Does not affect
        behavior.
* `name=et`
    - Required. Resource name for the ServicePerimeter.  The `short_name`
        component must begin with a letter and only include alphanumeric and &#39;_&#39;.
        Format: `accessPolicies/{policy_id}/servicePerimeters/{short_name}`
* `perimeter-type=diam`
    - Perimeter type indicator. A single project is
        allowed to be a member of single regular perimeter, but multiple service
        perimeter bridges. A project cannot be a included in a perimeter bridge
        without being included in regular perimeter. For perimeter bridges,
        restricted/unrestricted service lists as well as access lists must be
        empty.
* `status    access-levels=ipsum`
    - A list of `AccessLevel` resource names that allow resources within the
        `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed
        must be in the same policy as this `ServicePerimeter`. Referencing a
        nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are
        listed, resources within the perimeter can only be accessed via GCP calls with
        request origins within the perimeter. Example:
        `&#34;accessPolicies/MY_POLICY/accessLevels/MY_LEVEL&#34;`.
        For Service Perimeter Bridge, must be empty.
    - Each invocation of this argument appends the given value to the array.
* `resources=lorem`
    - A list of GCP resources that are inside of the service perimeter.
        Currently only projects are allowed. Format: `projects/{project_number}`
    - Each invocation of this argument appends the given value to the array.
* `restricted-services=et`
    - GCP services that are subject to the Service Perimeter restrictions. May
        contain a list of services or a single wildcard &#34;*&#34;. For example, if
        `storage.googleapis.com` is specified, access to the storage buckets
        inside the perimeter must meet the perimeter&#39;s access restrictions.
        
        Wildcard means that unless explicitly specified by &#34;unrestricted_services&#34;
        list, any service is treated as restricted. One of the fields
        &#34;restricted_services&#34;, &#34;unrestricted_services&#34; must contain a wildcard &#34;*&#34;,
        otherwise the Service Perimeter specification is invalid. It also means
        that both field being empty is invalid as well. &#34;restricted_services&#34; can
        be empty if and only if &#34;unrestricted_services&#34; list contains a &#34;*&#34;
        wildcard.
    - Each invocation of this argument appends the given value to the array.
* `unrestricted-services=duo`
    - GCP services that are not subject to the Service Perimeter restrictions.
        May contain a list of services or a single wildcard &#34;*&#34;. For example, if
        `logging.googleapis.com` is unrestricted, users can access logs inside the
        perimeter as if the perimeter doesn&#39;t exist, and it also means VMs inside the perimeter
        can access logs outside the perimeter.
        
        The wildcard means that unless explicitly specified by
        &#34;restricted_services&#34; list, any service is treated as unrestricted. One of
        the fields &#34;restricted_services&#34;, &#34;unrestricted_services&#34; must contain a
        wildcard &#34;*&#34;, otherwise the Service Perimeter specification is invalid. It
        also means that both field being empty is invalid as well.
        &#34;unrestricted_services&#34; can be empty if and only if &#34;restricted_services&#34;
        list contains a &#34;*&#34; wildcard.
    - Each invocation of this argument appends the given value to the array.

* `..    title=aliquyam`
    - Human readable title. Must be unique within the Policy.
* `update-time=sea`
    - Output only. Time the `ServicePerimeter` was updated in UTC.


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
