Adds a new user to the given account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.manage.users* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.manage.users*.
You can set the scope for this method like this: `analytics3 --scope <scope> management account-user-links-insert ...`
# Required Scalar Argument
* **&lt;account-id&gt;** *(string)*
    - Account ID to create the user link for.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
EntityUserLink:
  entity:
    account-ref:
      href: string
      id: string
      kind: string
      name: string
    profile-ref:
      account-id: string
      href: string
      id: string
      internal-web-property-id: string
      kind: string
      name: string
      web-property-id: string
    web-property-ref:
      account-id: string
      href: string
      id: string
      internal-web-property-id: string
      kind: string
      name: string
  id: string
  kind: string
  permissions:
    effective: [string]
    local: [string]
  self-link: string
  user-ref:
    email: string
    id: string
    kind: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .entity.account-ref    href=eirmod`
    - Link for this account.
* `id=sit`
    - Account ID.
* `kind=stet`
    - Analytics account reference.
* `name=sed`
    - Account name.

* `..profile-ref    account-id=et`
    - Account ID to which this view (profile) belongs.
* `href=dolores`
    - Link for this view (profile).
* `id=kasd`
    - View (Profile) ID.
* `internal-web-property-id=accusam`
    - Internal ID for the web property to which this view (profile) belongs.
* `kind=takimata`
    - Analytics view (profile) reference.
* `name=justo`
    - Name of this view (profile).
* `web-property-id=amet.`
    - Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs.

* `..web-property-ref    account-id=erat`
    - Account ID to which this web property belongs.
* `href=labore`
    - Link for this web property.
* `id=sea`
    - Web property ID of the form UA-XXXXX-YY.
* `internal-web-property-id=nonumy`
    - Internal ID for this web property.
* `kind=dolores`
    - Analytics web property reference.
* `name=gubergren`
    - Name of this web property.


* `...    id=sadipscing`
    - Entity user link ID
* `kind=aliquyam`
    - Resource type for entity user link.
* `permissions    effective=ea`
    - Effective permissions represent all the permissions that a user has for this entity. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent entity. Effective permissions are read-only.
    - Each invocation of this argument appends the given value to the array.
* `local=no`
    - Permissions that a user has been assigned at this very level. Does not include any implied or inherited permissions. Local permissions are modifiable.
    - Each invocation of this argument appends the given value to the array.

* `..    self-link=justo`
    - Self link for this resource.
* `user-ref    email=justo`
    - Email ID of this user.
* `id=et`
    - User ID.
* `kind=et`
    - No description provided.



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
