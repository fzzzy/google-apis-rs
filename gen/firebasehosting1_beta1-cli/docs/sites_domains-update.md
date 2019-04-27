Updates the specified domain mapping, creating the mapping as if it does
not exist.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/firebase*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `firebasehosting1-beta1 --scope <scope> sites domains-update ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Required. The name of the domain association to update or create, if an
        association doesn&#39;t already exist.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Domain:
  domain-name: string
  domain-redirect:
    domain-name: string
    type: string
  provisioning:
    cert-challenge-discovered-txt: [string]
    cert-challenge-dns:
      domain-name: string
      token: string
    cert-challenge-http:
      path: string
      token: string
    cert-status: string
    discovered-ips: [string]
    dns-fetch-time: string
    dns-status: string
    expected-ips: [string]
  site: string
  status: string
  update-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    domain-name=gubergren`
    - Required. The domain name of the association.
* `domain-redirect    domain-name=sadipscing`
    - Required. The domain name to redirect to.
* `type=aliquyam`
    - Required. The redirect status code.

* `..provisioning    cert-challenge-discovered-txt=ea`
    - The TXT records (for the certificate challenge) that were found at the last
        DNS fetch.
    - Each invocation of this argument appends the given value to the array.
* `cert-challenge-dns    domain-name=no`
    - The domain name upon which the DNS challenge must be satisfied.
* `token=justo`
    - The value that must be present as a TXT record on the domain name to
        satisfy the challenge.

* `..cert-challenge-http    path=justo`
    - The URL path on which to serve the specified token to satisfy the
        certificate challenge.
* `token=et`
    - The token to serve at the specified URL path to satisfy the certificate
        challenge.

* `..    cert-status=et`
    - The certificate provisioning status; updated when Firebase Hosting
        provisions an SSL certificate for the domain.
* `discovered-ips=diam`
    - The IPs found at the last DNS fetch.
    - Each invocation of this argument appends the given value to the array.
* `dns-fetch-time=ipsum`
    - The time at which the last DNS fetch occurred.
* `dns-status=lorem`
    - The DNS record match status as of the last DNS fetch.
* `expected-ips=et`
    - The list of IPs to which the domain is expected to resolve.
    - Each invocation of this argument appends the given value to the array.

* `..    site=duo`
    - Required. The site name of the association.
* `status=aliquyam`
    - Output only. Additional status of the domain association.
* `update-time=sea`
    - Output only. The time at which the domain was last updated.


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
