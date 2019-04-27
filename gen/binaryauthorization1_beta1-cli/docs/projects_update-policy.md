Creates or updates a project&#39;s policy, and returns a copy of the
new policy. A policy is always updated as a whole, to avoid race
conditions with concurrent policy enforcement (or management!)
requests. Returns NOT_FOUND if the project does not exist, INVALID_ARGUMENT
if the request is malformed.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `binaryauthorization1-beta1 --scope <scope> projects update-policy ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Output only. The resource name, in the format `projects/*/policy`. There is
        at most one policy per project.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Policy:
  default-admission-rule:
    enforcement-mode: string
    evaluation-mode: string
    require-attestations-by: [string]
  description: string
  name: string
  update-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .default-admission-rule    enforcement-mode=gubergren`
    - Required. The action when a pod creation is denied by the admission rule.
* `evaluation-mode=sadipscing`
    - Required. How this admission rule will be evaluated.
* `require-attestations-by=aliquyam`
    - Optional. The resource names of the attestors that must attest to
        a container image, in the format `projects/*/attestors/*`. Each
        attestor must exist before a policy can reference it.  To add an attestor
        to a policy the principal issuing the policy change request must be able
        to read the attestor resource.
        
        Note: this field must be non-empty when the evaluation_mode field specifies
        REQUIRE_ATTESTATION, otherwise it must be empty.
    - Each invocation of this argument appends the given value to the array.

* `..    description=ea`
    - Optional. A descriptive comment.
* `name=no`
    - Output only. The resource name, in the format `projects/*/policy`. There is
        at most one policy per project.
* `update-time=justo`
    - Output only. Time when the policy was last updated.


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
