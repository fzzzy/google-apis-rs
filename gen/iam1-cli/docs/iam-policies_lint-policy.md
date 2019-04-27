Lints a Cloud IAM policy object or its sub fields. Currently supports
google.iam.v1.Policy, google.iam.v1.Binding and
google.iam.v1.Binding.condition.

Each lint operation consists of multiple lint validation units.
Validation units have the following properties:

- Each unit inspects the input object in regard to a particular
  linting aspect and issues a google.iam.admin.v1.LintResult
  disclosing the result.
- Domain of discourse of each unit can be either
  google.iam.v1.Policy, google.iam.v1.Binding, or
  google.iam.v1.Binding.condition depending on the purpose of the
  validation.
- A unit may require additional data (like the list of all possible
  enumerable values of a particular attribute used in the policy instance)
  which shall be provided by the caller. Refer to the comments of
  google.iam.admin.v1.LintPolicyRequest.context for more details.

The set of applicable validation units is determined by the Cloud IAM
server and is not configurable.

Regardless of any lint issues or their severities, successful calls to
`lintPolicy` return an HTTP 200 OK status code.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `iam1 --scope <scope> iam-policies lint-policy ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LintPolicyRequest:
  binding:
    condition:
      description: string
      expression: string
      location: string
      title: string
    members: [string]
    role: string
  condition:
    description: string
    expression: string
    location: string
    title: string
  full-resource-name: string
  policy:
    etag: string
    version: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .binding.condition    description=eirmod`
    - An optional description of the expression. This is a longer text which
        describes the expression, e.g. when hovered over it in a UI.
* `expression=sit`
    - Textual representation of an expression in
        Common Expression Language syntax.
        
        The application context of the containing message determines which
        well-known feature set of CEL is supported.
* `location=stet`
    - An optional string indicating the location of the expression for error
        reporting, e.g. a file name and a position in the file.
* `title=sed`
    - An optional title for the expression, i.e. a short string describing
        its purpose. This can be used e.g. in UIs which allow to enter the
        expression.

* `..    members=et`
    - Specifies the identities requesting access for a Cloud Platform resource.
        `members` can have the following values:
        
        * `allUsers`: A special identifier that represents anyone who is
           on the internet; with or without a Google account.
        
        * `allAuthenticatedUsers`: A special identifier that represents anyone
           who is authenticated with a Google account or a service account.
        
        * `user:{emailid}`: An email address that represents a specific Google
           account. For example, `alice@gmail.com` .
        
        
        * `serviceAccount:{emailid}`: An email address that represents a service
           account. For example, `my-other-app@appspot.gserviceaccount.com`.
        
        * `group:{emailid}`: An email address that represents a Google group.
           For example, `admins@example.com`.
        
        
        * `domain:{domain}`: A Google Apps domain name that represents all the
           users of that domain. For example, `google.com` or `example.com`.
        
        
    - Each invocation of this argument appends the given value to the array.
* `role=dolores`
    - Role that is assigned to `members`.
        For example, `roles/viewer`, `roles/editor`, or `roles/owner`.

* `..condition    description=kasd`
    - An optional description of the expression. This is a longer text which
        describes the expression, e.g. when hovered over it in a UI.
* `expression=accusam`
    - Textual representation of an expression in
        Common Expression Language syntax.
        
        The application context of the containing message determines which
        well-known feature set of CEL is supported.
* `location=takimata`
    - An optional string indicating the location of the expression for error
        reporting, e.g. a file name and a position in the file.
* `title=justo`
    - An optional title for the expression, i.e. a short string describing
        its purpose. This can be used e.g. in UIs which allow to enter the
        expression.

* `..    full-resource-name=amet.`
    - The full resource name of the policy this lint request is about.
        
        The name follows the Google Cloud Platform (GCP) resource format.
        For example, a GCP project with ID `my-project` will be named
        `//cloudresourcemanager.googleapis.com/projects/my-project`.
        
        The resource name is not used to read the policy instance from the Cloud
        IAM database. The candidate policy for lint has to be provided in the same
        request object.
* `policy    etag=erat`
    - `etag` is used for optimistic concurrency control as a way to help
        prevent simultaneous updates of a policy from overwriting each other.
        It is strongly suggested that systems make use of the `etag` in the
        read-modify-write cycle to perform policy updates in order to avoid race
        conditions: An `etag` is returned in the response to `getIamPolicy`, and
        systems are expected to put that etag in the request to `setIamPolicy` to
        ensure that their change will be applied to the same version of the policy.
        
        If no `etag` is provided in the call to `setIamPolicy`, then the existing
        policy is overwritten blindly.
* `version=66`
    - Deprecated.



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
