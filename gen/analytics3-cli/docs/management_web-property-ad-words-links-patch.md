Updates an existing webProperty-AdWords link. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.edit* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.edit*.
You can set the scope for this method like this: `analytics3 --scope <scope> management web-property-ad-words-links-patch ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - ID of the account which the given web property belongs to.
* **&lt;web-property-id&gt;** *(string)*
    - Web property ID to retrieve the AdWords link for.
* **&lt;web-property-ad-words-link-id&gt;** *(string)*
    - Web property-AdWords link ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
EntityAdWordsLink:
  entity:
    web-property-ref:
      account-id: string
      href: string
      id: string
      internal-web-property-id: string
      kind: string
      name: string
  id: string
  kind: string
  name: string
  profile-ids: [string]
  self-link: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .entity.web-property-ref    account-id=diam`
    - Account ID to which this web property belongs.
* `href=magna`
    - Link for this web property.
* `id=dolores`
    - Web property ID of the form UA-XXXXX-YY.
* `internal-web-property-id=eirmod`
    - Internal ID for this web property.
* `kind=diam`
    - Analytics web property reference.
* `name=sea`
    - Name of this web property.


* `...    id=labore`
    - Entity AdWords link ID
* `kind=sed`
    - Resource type for entity AdWords link.
* `name=amet.`
    - Name of the link. This field is required when creating an AdWords link.
* `profile-ids=duo`
    - IDs of linked Views (Profiles) represented as strings.
    - Each invocation of this argument appends the given value to the array.
* `self-link=sit`
    - URL link for this Google Analytics - Google AdWords link.


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
