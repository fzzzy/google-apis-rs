Create a new contact and return the person resource for that contact.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/contacts* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/contacts*.
You can set the scope for this method like this: `people1 --scope <scope> people create-contact ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Person:
  age-range: string
  etag: string
  metadata:
    deleted: boolean
    linked-people-resource-names: [string]
    object-type: string
    previous-resource-names: [string]
  resource-name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    age-range=no`
    - **DEPRECATED** (Please use `person.ageRanges` instead)**
        
        The person&#39;s read-only age range.
* `etag=justo`
    - The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the
        resource. Used for web cache validation.
* `metadata    deleted=true`
    - True if the person resource has been deleted. Populated only for
        [`connections.list`](/people/api/rest/v1/people.connections/list) requests
        that include a sync token.
* `linked-people-resource-names=et`
    - Resource names of people linked to this resource.
    - Each invocation of this argument appends the given value to the array.
* `object-type=et`
    - **DEPRECATED** (Please use
        `person.metadata.sources.profileMetadata.objectType` instead)
        
        The type of the person object.
* `previous-resource-names=diam`
    - Any former resource names this person has had. Populated only for
        [`connections.list`](/people/api/rest/v1/people.connections/list) requests
        that include a sync token.
        
        The resource name may change when adding or removing fields that link a
        contact and profile such as a verified email, verified phone number, or
        profile URL.
    - Each invocation of this argument appends the given value to the array.

* `..    resource-name=ipsum`
    - The resource name for the person, assigned by the server. An ASCII string
        with a max length of 27 characters, in the form of
        `people/`&lt;var&gt;person_id&lt;/var&gt;.


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

* **-p parent=string**
    - The resource name of the owning person resource.

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
