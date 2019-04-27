
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The resource name of the service account for which the credentials
        are requested, in the following format:
        `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GenerateIdentityBindingAccessTokenRequest:
  jwt: string
  scope: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    jwt=kasd`
    - Required. Input token.
        Must be in JWT format according to
        RFC7523 (https://tools.ietf.org/html/rfc7523)
        and must have &#39;kid&#39; field in the header.
        Supported signing algorithms: RS256 (RS512, ES256, ES512 coming soon).
        Mandatory payload fields (along the lines of RFC 7523, section 3):
        - iss: issuer of the token. Must provide a discovery document at
               $iss/.well-known/openid-configuration . The document needs to be
               formatted according to section 4.2 of the OpenID Connect Discovery
               1.0 specification.
        - iat: Issue time in seconds since epoch. Must be in the past.
        - exp: Expiration time in seconds since epoch. Must be less than 48 hours
               after iat. We recommend to create tokens that last shorter than 6
               hours to improve security unless business reasons mandate longer
               expiration times. Shorter token lifetimes are generally more secure
               since tokens that have been exfiltrated by attackers can be used for
               a shorter time. you can configure the maximum lifetime of the
               incoming token in the configuration of the mapper.
               The resulting Google token will expire within an hour or at &#34;exp&#34;,
               whichever is earlier.
        - sub: JWT subject, identity asserted in the JWT.
        - aud: Configured in the mapper policy. By default the service account
               email.
        
        Claims from the incoming token can be transferred into the output token
        accoding to the mapper configuration. The outgoing claim size is limited.
        Outgoing claims size must be less than 4kB serialized as JSON without
        whitespace.
        
        Example header:
        {
          &#34;alg&#34;: &#34;RS256&#34;,
          &#34;kid&#34;: &#34;92a4265e14ab04d4d228a48d10d4ca31610936f8&#34;
        }
        Example payload:
        {
          &#34;iss&#34;: &#34;https://accounts.google.com&#34;,
          &#34;iat&#34;: 1517963104,
          &#34;exp&#34;: 1517966704,
          &#34;aud&#34;: &#34;https://iamcredentials.googleapis.com/google.iam.credentials.v1.CloudGaia&#34;,
          &#34;sub&#34;: &#34;113475438248934895348&#34;,
          &#34;my_claims&#34;: {
            &#34;additional_claim&#34;: &#34;value&#34;
          }
        }
* `scope=accusam`
    - Code to identify the scopes to be included in the OAuth 2.0 access token.
        See https://developers.google.com/identity/protocols/googlescopes for more
        information.
        At least one value required.
    - Each invocation of this argument appends the given value to the array.


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

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
