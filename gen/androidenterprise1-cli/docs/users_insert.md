Creates a new EMM-managed user.

The Users resource passed in the body of the request should include an accountIdentifier and an accountType.
If a corresponding user already exists with the same account identifier, the user will be updated with the resource. In this case only the displayName field can be changed.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidenterprise* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidenterprise*.
You can set the scope for this method like this: `androidenterprise1 --scope <scope> users insert ...`
# Required Scalar Argument
* **&lt;enterprise-id&gt;** *(string)*
    - The ID of the enterprise.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
User:
  account-identifier: string
  account-type: string
  display-name: string
  id: string
  kind: string
  management-type: string
  primary-email: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-identifier=labore`
    - A unique identifier you create for this user, such as &#34;user342&#34; or &#34;asset#44418&#34;. Do not use personally identifiable information (PII) for this property. Must always be set for EMM-managed users. Not set for Google-managed users.
* `account-type=invidunt`
    - The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount.
* `display-name=ea`
    - The name that will appear in user interfaces. Setting this property is optional when creating EMM-managed users. If you do set this property, use something generic about the organization (such as &#34;Example, Inc.&#34;) or your name (as EMM). Not used for Google-managed user accounts.
* `id=sadipscing`
    - The unique ID for the user.
* `kind=rebum.`
    - Identifies what kind of resource this is. Value: the fixed string &#34;androidenterprise#user&#34;.
* `management-type=dolore`
    - The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge.
* `primary-email=nonumy`
    - The user&#39;s primary email address, for example, &#34;jsmith@example.com&#34;. Will always be set for Google managed users and not set for EMM managed users.


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