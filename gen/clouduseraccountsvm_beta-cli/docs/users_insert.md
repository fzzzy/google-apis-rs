Creates a User resource in the specified project using the data included in the request.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud.useraccounts*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `clouduseraccountsvm-beta --scope <scope> users insert ...`
# Required Scalar Argument
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
User:
  creation-timestamp: string
  description: string
  groups: [string]
  id: string
  kind: string
  name: string
  owner: string
  self-link: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    creation-timestamp=nonumy`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `description=dolores`
    - An optional textual description of the resource; provided by the client when the resource is created.
* `groups=gubergren`
    - [Output Only] A list of URLs to Group resources who contain the user. Users are only members of groups in the same project.
    - Each invocation of this argument appends the given value to the array.
* `id=sadipscing`
    - [Output Only] Unique identifier for the resource; defined by the server.
* `kind=aliquyam`
    - [Output Only] Type of the resource. Always clouduseraccounts#user for users.
* `name=ea`
    - Name of the resource; provided by the client when the resource is created.
* `owner=no`
    - Email address of account&#39;s owner. This account will be validated to make sure it exists. The email can belong to any domain, but it must be tied to a Google account.
* `self-link=justo`
    - [Output Only] Server defined URL for the resource.


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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.