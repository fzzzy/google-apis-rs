Updates an existing web property. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.edit* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.edit*.
You can set the scope for this method like this: `analytics3 --scope <scope> management webproperties-patch ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account ID to which the web property belongs
* **&lt;web-property-id&gt;** *(string)*
    - Web property ID
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Webproperty:
  account-id: string
  child-link:
    href: string
    type: string
  created: string
  data-retention-reset-on-new-activity: boolean
  data-retention-ttl: string
  default-profile-id: string
  id: string
  industry-vertical: string
  internal-web-property-id: string
  kind: string
  level: string
  name: string
  parent-link:
    href: string
    type: string
  permissions:
    effective: [string]
  profile-count: integer
  self-link: string
  starred: boolean
  updated: string
  website-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=lorem`
    - Account ID to which this web property belongs.
* `child-link    href=dolores`
    - Link to the list of views (profiles) for this web property.
* `type=voluptua.`
    - Type of the parent link. Its value is &#34;analytics#profiles&#34;.

* `..    created=duo`
    - Time this web property was created.
* `data-retention-reset-on-new-activity=true`
    - Set to true to reset the retention period of the user identifier with each new event from that user (thus setting the expiration date to current time plus retention period).
        Set to false to delete data associated with the user identifer automatically after the rentention period.
        This property cannot be set on insert.
* `data-retention-ttl=clita`
    - The length of time for which user and event data is retained.
        This property cannot be set on insert.
* `default-profile-id=sit`
    - Default view (profile) ID.
* `id=invidunt`
    - Web property ID of the form UA-XXXXX-YY.
* `industry-vertical=vero`
    - The industry vertical/category selected for this web property.
* `internal-web-property-id=sed`
    - Internal ID for this web property.
* `kind=ut`
    - Resource type for Analytics WebProperty.
* `level=rebum.`
    - Level for this web property. Possible values are STANDARD or PREMIUM.
* `name=stet`
    - Name of this web property.
* `parent-link    href=sanctus`
    - Link to the account for this web property.
* `type=no`
    - Type of the parent link. Its value is &#34;analytics#account&#34;.

* `..permissions    effective=rebum.`
    - All the permissions that the user has for this web property. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent account.
    - Each invocation of this argument appends the given value to the array.

* `..    profile-count=90`
    - View (Profile) count for this web property.
* `self-link=sed`
    - Link for this web property.
* `starred=true`
    - Indicates whether this web property is starred or not.
* `updated=gubergren`
    - Time this web property was last modified.
* `website-url=invidunt`
    - Website url for this web property.


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