Set account info for a user.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `identitytoolkit3 --scope <scope> relyingparty set-account-info ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
IdentitytoolkitRelyingpartySetAccountInfoRequest:
  captcha-challenge: string
  captcha-response: string
  created-at: string
  custom-attributes: string
  delegated-project-number: string
  delete-attribute: [string]
  delete-provider: [string]
  disable-user: boolean
  display-name: string
  email: string
  email-verified: boolean
  id-token: string
  instance-id: string
  last-login-at: string
  local-id: string
  oob-code: string
  password: string
  phone-number: string
  photo-url: string
  provider: [string]
  return-secure-token: boolean
  upgrade-to-federated-login: boolean
  valid-since: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    captcha-challenge=et`
    - The captcha challenge.
* `captcha-response=amet`
    - Response to the captcha.
* `created-at=et`
    - The timestamp when the account is created.
* `custom-attributes=consetetur`
    - The custom attributes to be set in the user&#39;s id token.
* `delegated-project-number=ut`
    - GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
* `delete-attribute=ea`
    - The attributes users request to delete.
    - Each invocation of this argument appends the given value to the array.
* `delete-provider=sed`
    - The IDPs the user request to delete.
    - Each invocation of this argument appends the given value to the array.
* `disable-user=true`
    - Whether to disable the user.
* `display-name=dolor`
    - The name of the user.
* `email=dolor`
    - The email of the user.
* `email-verified=true`
    - Mark the email as verified or not.
* `id-token=consetetur`
    - The GITKit token of the authenticated user.
* `instance-id=amet.`
    - Instance id token of the app.
* `last-login-at=voluptua.`
    - Last login timestamp.
* `local-id=lorem`
    - The local ID of the user.
* `oob-code=gubergren`
    - The out-of-band code of the change email request.
* `password=justo`
    - The new password of the user.
* `phone-number=sit`
    - Privileged caller can update user with specified phone number.
* `photo-url=vero`
    - The photo url of the user.
* `provider=diam`
    - The associated IDPs of the user.
    - Each invocation of this argument appends the given value to the array.
* `return-secure-token=false`
    - Whether return sts id token and refresh token instead of gitkit token.
* `upgrade-to-federated-login=true`
    - Mark the user to upgrade to federated login.
* `valid-since=sadipscing`
    - Timestamp in seconds for valid login token.


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
