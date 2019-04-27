Adds a new user to the given view (profile).
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.manage.users* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.manage.users*.
You can set the scope for this method like this: `analytics3 --scope <scope> management profile-user-links-insert ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account ID to create the user link for.
* **&lt;web-property-id&gt;** *(string)*
    - Web Property ID to create the user link for.
* **&lt;profile-id&gt;** *(string)*
    - View (Profile) ID to create the user link for.
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

* `-r .entity.account-ref    href=justo`
    - Link for this account.
* `id=gubergren`
    - Account ID.
* `kind=ipsum`
    - Analytics account reference.
* `name=amet.`
    - Account name.

* `..profile-ref    account-id=dolor`
    - Account ID to which this view (profile) belongs.
* `href=sanctus`
    - Link for this view (profile).
* `id=ut`
    - View (Profile) ID.
* `internal-web-property-id=magna`
    - Internal ID for the web property to which this view (profile) belongs.
* `kind=amet.`
    - Analytics view (profile) reference.
* `name=eirmod`
    - Name of this view (profile).
* `web-property-id=ea`
    - Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs.

* `..web-property-ref    account-id=ea`
    - Account ID to which this web property belongs.
* `href=et`
    - Link for this web property.
* `id=sanctus`
    - Web property ID of the form UA-XXXXX-YY.
* `internal-web-property-id=diam`
    - Internal ID for this web property.
* `kind=eos`
    - Analytics web property reference.
* `name=tempor`
    - Name of this web property.


* `...    id=sea`
    - Entity user link ID
* `kind=ea`
    - Resource type for entity user link.
* `permissions    effective=nonumy`
    - Effective permissions represent all the permissions that a user has for this entity. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent entity. Effective permissions are read-only.
    - Each invocation of this argument appends the given value to the array.
* `local=duo`
    - Permissions that a user has been assigned at this very level. Does not include any implied or inherited permissions. Local permissions are modifiable.
    - Each invocation of this argument appends the given value to the array.

* `..    self-link=tempor`
    - Self link for this resource.
* `user-ref    email=consetetur`
    - Email ID of this user.
* `id=eirmod`
    - User ID.
* `kind=aliquyam`
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
