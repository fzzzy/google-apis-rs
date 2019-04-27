update user
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.directory.user* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.user*.
You can set the scope for this method like this: `admin1-directory --scope <scope> users update ...`
# Required Scalar Argument
* **&lt;user-key&gt;** *(string)*
    - Email or immutable ID of the user. If ID, it should match with id of user object
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
User:
  agreed-to-terms: boolean
  aliases: [string]
  archived: boolean
  change-password-at-next-login: boolean
  creation-time: string
  customer-id: string
  deletion-time: string
  etag: string
  hash-function: string
  id: string
  include-in-global-address-list: boolean
  ip-whitelisted: boolean
  is-admin: boolean
  is-delegated-admin: boolean
  is-enforced-in2-sv: boolean
  is-enrolled-in2-sv: boolean
  is-mailbox-setup: boolean
  kind: string
  last-login-time: string
  name:
    family-name: string
    full-name: string
    given-name: string
  non-editable-aliases: [string]
  org-unit-path: string
  password: string
  primary-email: string
  suspended: boolean
  suspension-reason: string
  thumbnail-photo-etag: string
  thumbnail-photo-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    agreed-to-terms=false`
    - Indicates if user has agreed to terms (Read-only)
* `aliases=erat`
    - List of aliases (Read-only)
    - Each invocation of this argument appends the given value to the array.
* `archived=true`
    - Indicates if user is archived.
* `change-password-at-next-login=false`
    - Boolean indicating if the user should change password in next login
* `creation-time=vero`
    - User&#39;s G Suite account creation time. (Read-only)
* `customer-id=sed`
    - CustomerId of User (Read-only)
* `deletion-time=vero`
    - No description provided.
* `etag=dolore`
    - ETag of the resource.
* `hash-function=lorem`
    - Hash function name for password. Supported are MD5, SHA-1 and crypt
* `id=invidunt`
    - Unique identifier of User (Read-only)
* `include-in-global-address-list=false`
    - Boolean indicating if user is included in Global Address List
* `ip-whitelisted=false`
    - Boolean indicating if ip is whitelisted
* `is-admin=false`
    - Boolean indicating if the user is admin (Read-only)
* `is-delegated-admin=false`
    - Boolean indicating if the user is delegated admin (Read-only)
* `is-enforced-in2-sv=false`
    - Is 2-step verification enforced (Read-only)
* `is-enrolled-in2-sv=false`
    - Is enrolled in 2-step verification (Read-only)
* `is-mailbox-setup=false`
    - Is mailbox setup (Read-only)
* `kind=ipsum`
    - Kind of resource this is.
* `last-login-time=voluptua.`
    - User&#39;s last login time. (Read-only)
* `name    family-name=eirmod`
    - Last Name
* `full-name=sed`
    - Full Name
* `given-name=accusam`
    - First Name

* `..    non-editable-aliases=sanctus`
    - List of non editable aliases (Read-only)
    - Each invocation of this argument appends the given value to the array.
* `org-unit-path=dolor`
    - OrgUnit of User
* `password=dolor`
    - User&#39;s password
* `primary-email=dolore`
    - username of User
* `suspended=false`
    - Indicates if user is suspended.
* `suspension-reason=ipsum`
    - Suspension reason if user is suspended (Read-only)
* `thumbnail-photo-etag=diam`
    - ETag of the user&#39;s photo (Read-only)
* `thumbnail-photo-url=sed`
    - Photo Url of the user (Read-only)


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
