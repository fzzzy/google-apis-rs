Sets the IAM access control policy for the specified Project. Overwrites
any existing policy.

The following constraints apply when using `setIamPolicy()`:

+ Project does not support `allUsers` and `allAuthenticatedUsers` as
`members` in a `Binding` of a `Policy`.

+ The owner role can be granted only to `user` and `serviceAccount`.

+ Service accounts can be made owners of a project directly
without any restrictions. However, to be added as an owner, a user must be
invited via Cloud Platform console and must accept the invitation.

+ A user cannot be granted the owner role using `setIamPolicy()`. The user
must be granted the owner role using the Cloud Platform Console and must
explicitly accept the invitation.

+ Invitations to grant the owner role cannot be sent using
`setIamPolicy()`; they must be sent only using the Cloud Platform Console.

+ Membership changes that leave the project without any owners that have
accepted the Terms of Service (ToS) will be rejected.

+ If the project is not part of an organization, there must be at least
one owner who has accepted the Terms of Service (ToS) agreement in the
policy. Calling `setIamPolicy()` to remove the last ToS-accepted owner
from the policy will fail. This restriction also applies to legacy
projects that no longer have owners who have accepted the ToS. Edits to
IAM policies will be rejected until the lack of a ToS-accepting owner is
rectified.

+ This method will replace the existing policy, and cannot be used to
append additional IAM settings.

Note: Removing service accounts from policies or changing their roles
can render services completely inoperable. It is important to understand
how the service account is being used before removing or updating its
roles.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudresourcemanager1-beta1 --scope <scope> projects set-iam-policy ...`
# Required Scalar Argument
* **&lt;resource&gt;** *(string)*
    - REQUIRED: The resource for which the policy is being specified.
        See the operation documentation for the appropriate value for this field.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SetIamPolicyRequest:
  policy:
    etag: string
    version: integer
  update-mask: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .policy    etag=aliquyam`
    - `etag` is used for optimistic concurrency control as a way to help
        prevent simultaneous updates of a policy from overwriting each other.
        It is strongly suggested that systems make use of the `etag` in the
        read-modify-write cycle to perform policy updates in order to avoid race
        conditions: An `etag` is returned in the response to `getIamPolicy`, and
        systems are expected to put that etag in the request to `setIamPolicy` to
        ensure that their change will be applied to the same version of the policy.
        
        If no `etag` is provided in the call to `setIamPolicy`, then the existing
        policy is overwritten blindly.
* `version=35`
    - Deprecated.

* `..    update-mask=no`
    - OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only
        the fields in the mask will be modified. If no mask is provided, the
        following default mask is used:
        paths: &#34;bindings, etag&#34;
        This field is only used by Cloud IAM.


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