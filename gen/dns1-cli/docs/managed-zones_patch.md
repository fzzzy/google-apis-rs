Apply a partial update to an existing ManagedZone.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/ndev.clouddns.readwrite*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dns1 --scope <scope> managed-zones patch ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Identifies the project addressed by this request.
* **&lt;managed-zone&gt;** *(string)*
    - Identifies the managed zone addressed by this request. Can be the managed zone name or id.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ManagedZone:
  creation-time: string
  description: string
  dns-name: string
  dnssec-config:
    kind: string
    non-existence: string
    state: string
  id: string
  kind: string
  labels: { string: string }
  name: string
  name-server-set: string
  name-servers: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    creation-time=sadipscing`
    - The time that this resource was created on the server. This is in RFC3339 text format. Output only.
* `description=aliquyam`
    - A mutable string of at most 1024 characters associated with this resource for the user&#39;s convenience. Has no effect on the managed zone&#39;s function.
* `dns-name=ea`
    - The DNS name of this managed zone, for instance &#34;example.com.&#34;.
* `dnssec-config    kind=no`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dns#managedZoneDnsSecConfig&#34;.
* `non-existence=justo`
    - Specifies the mechanism used to provide authenticated denial-of-existence responses. Output only while state is not OFF.
* `state=justo`
    - Specifies whether DNSSEC is enabled, and what mode it is in.

* `..    id=et`
    - Unique identifier for the resource; defined by the server (output only)
* `kind=et`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dns#managedZone&#34;.
* `labels=key=diam`
    - User labels.
    - the value will be associated with the given `key`
* `name=ipsum`
    - User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes.
* `name-server-set=lorem`
    - Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users will leave this field unset.
* `name-servers=et`
    - Delegate your managed_zone to these virtual name servers; defined by the server (output only)
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

* **-p client-operation-id=string**
    - For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection.

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
