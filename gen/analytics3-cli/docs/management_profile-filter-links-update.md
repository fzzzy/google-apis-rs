Update an existing profile filter link.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.edit* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.edit*.
You can set the scope for this method like this: `analytics3 --scope <scope> management profile-filter-links-update ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account ID to which profile filter link belongs.
* **&lt;web-property-id&gt;** *(string)*
    - Web property Id to which profile filter link belongs
* **&lt;profile-id&gt;** *(string)*
    - Profile ID to which filter link belongs
* **&lt;link-id&gt;** *(string)*
    - ID of the profile filter link to be updated.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ProfileFilterLink:
  filter-ref:
    account-id: string
    href: string
    id: string
    kind: string
    name: string
  id: string
  kind: string
  profile-ref:
    account-id: string
    href: string
    id: string
    internal-web-property-id: string
    kind: string
    name: string
    web-property-id: string
  rank: integer
  self-link: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .filter-ref    account-id=duo`
    - Account ID to which this filter belongs.
* `href=ipsum`
    - Link for this filter.
* `id=diam`
    - Filter ID.
* `kind=sed`
    - Kind value for filter reference.
* `name=et`
    - Name of this filter.

* `..    id=tempor`
    - Profile filter link ID.
* `kind=justo`
    - Resource type for Analytics filter.
* `profile-ref    account-id=takimata`
    - Account ID to which this view (profile) belongs.
* `href=invidunt`
    - Link for this view (profile).
* `id=amet`
    - View (Profile) ID.
* `internal-web-property-id=dolor`
    - Internal ID for the web property to which this view (profile) belongs.
* `kind=voluptua.`
    - Analytics view (profile) reference.
* `name=et`
    - Name of this view (profile).
* `web-property-id=elitr`
    - Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs.

* `..    rank=89`
    - The rank of this profile filter link relative to the other filters linked to the same profile.
        For readonly (i.e., list and get) operations, the rank always starts at 1.
        For write (i.e., create, update, or delete) operations, you may specify a value between 0 and 255 inclusively, [0, 255]. In order to insert a link at the end of the list, either don&#39;t specify a rank or set a rank to a number greater than the largest rank in the list. In order to insert a link to the beginning of the list specify a rank that is less than or equal to 1. The new link will move all existing filters with the same or lower rank down the list. After the link is inserted/updated/deleted all profile filter links will be renumbered starting at 1.
* `self-link=sit`
    - Link for this profile filter link.


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
