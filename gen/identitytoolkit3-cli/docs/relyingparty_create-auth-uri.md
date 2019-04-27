Creates the URI used by the IdP to authenticate the user.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `identitytoolkit3 --scope <scope> relyingparty create-auth-uri ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
IdentitytoolkitRelyingpartyCreateAuthUriRequest:
  app-id: string
  auth-flow-type: string
  client-id: string
  context: string
  continue-uri: string
  custom-parameter: { string: string }
  hosted-domain: string
  identifier: string
  oauth-consumer-key: string
  oauth-scope: string
  openid-realm: string
  ota-app: string
  provider-id: string
  session-id: string
  tenant-id: string
  tenant-project-number: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    app-id=eirmod`
    - The app ID of the mobile app, base64(CERT_SHA1):PACKAGE_NAME for Android, BUNDLE_ID for iOS.
* `auth-flow-type=sit`
    - Explicitly specify the auth flow type. Currently only support &#34;CODE_FLOW&#34; type. The field is only used for Google provider.
* `client-id=stet`
    - The relying party OAuth client ID.
* `context=sed`
    - The opaque value used by the client to maintain context info between the authentication request and the IDP callback.
* `continue-uri=et`
    - The URI to which the IDP redirects the user after the federated login flow.
* `custom-parameter=key=dolores`
    - The query parameter that client can customize by themselves in auth url. The following parameters are reserved for server so that they cannot be customized by clients: client_id, response_type, scope, redirect_uri, state, oauth_token.
    - the value will be associated with the given `key`
* `hosted-domain=kasd`
    - The hosted domain to restrict sign-in to accounts at that domain for Google Apps hosted accounts.
* `identifier=accusam`
    - The email or federated ID of the user.
* `oauth-consumer-key=takimata`
    - The developer&#39;s consumer key for OpenId OAuth Extension
* `oauth-scope=justo`
    - Additional oauth scopes, beyond the basid user profile, that the user would be prompted to grant
* `openid-realm=amet.`
    - Optional realm for OpenID protocol. The sub string &#34;scheme://domain:port&#34; of the param &#34;continueUri&#34; is used if this is not set.
* `ota-app=erat`
    - The native app package for OTA installation.
* `provider-id=labore`
    - The IdP ID. For white listed IdPs it&#39;s a short domain name e.g. google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it&#39;s the OP identifier.
* `session-id=sea`
    - The session_id passed by client.
* `tenant-id=nonumy`
    - For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from.
* `tenant-project-number=dolores`
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
