Verifies the user entered password.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `identitytoolkit3 --scope <scope> relyingparty verify-password ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
IdentitytoolkitRelyingpartyVerifyPasswordRequest:
  captcha-challenge: string
  captcha-response: string
  delegated-project-number: string
  email: string
  id-token: string
  instance-id: string
  password: string
  pending-id-token: string
  return-secure-token: boolean
  tenant-id: string
  tenant-project-number: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    captcha-challenge=sed`
    - The captcha challenge.
* `captcha-response=sit`
    - Response to the captcha.
* `delegated-project-number=takimata`
    - GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
* `email=elitr`
    - The email of the user.
* `id-token=nonumy`
    - The GITKit token of the authenticated user.
* `instance-id=rebum.`
    - Instance id token of the app.
* `password=lorem`
    - The password inputed by the user.
* `pending-id-token=lorem`
    - The GITKit token for the non-trusted IDP, which is to be confirmed by the user.
* `return-secure-token=true`
    - Whether return sts id token and refresh token instead of gitkit token.
* `tenant-id=ut`
    - For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from.
* `tenant-project-number=ut`
    - Tenant project number to be used for idp discovery.


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