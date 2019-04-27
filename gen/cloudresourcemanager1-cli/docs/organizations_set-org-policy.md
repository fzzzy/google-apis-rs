Updates the specified `Policy` on the resource. Creates a new `Policy` for
that `Constraint` on the resource if one does not exist.

Not supplying an `etag` on the request `Policy` results in an unconditional
write of the `Policy`.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudresourcemanager1 --scope <scope> organizations set-org-policy ...`
# Required Scalar Argument
* **&lt;resource&gt;** *(string)*
    - Resource name of the resource to attach the `Policy`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SetOrgPolicyRequest:
  policy:
    boolean-policy:
      enforced: boolean
    constraint: string
    etag: string
    list-policy:
      all-values: string
      allowed-values: [string]
      denied-values: [string]
      inherit-from-parent: boolean
      suggested-value: string
    update-time: string
    version: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .policy.boolean-policy    enforced=true`
    - If `true`, then the `Policy` is enforced. If `false`, then any
        configuration is acceptable.
        
        Suppose you have a `Constraint` `constraints/compute.disableSerialPortAccess`
        with `constraint_default` set to `ALLOW`. A `Policy` for that
        `Constraint` exhibits the following behavior:
          - If the `Policy` at this resource has enforced set to `false`, serial
            port connection attempts will be allowed.
          - If the `Policy` at this resource has enforced set to `true`, serial
            port connection attempts will be refused.
          - If the `Policy` at this resource is `RestoreDefault`, serial port
            connection attempts will be allowed.
          - If no `Policy` is set at this resource or anywhere higher in the
            resource hierarchy, serial port connection attempts will be allowed.
          - If no `Policy` is set at this resource, but one exists higher in the
            resource hierarchy, the behavior is as if the`Policy` were set at
            this resource.
        
        The following examples demonstrate the different possible layerings:
        
        Example 1 (nearest `Constraint` wins):
          `organizations/foo` has a `Policy` with:
            {enforced: false}
          `projects/bar` has no `Policy` set.
        The constraint at `projects/bar` and `organizations/foo` will not be
        enforced.
        
        Example 2 (enforcement gets replaced):
          `organizations/foo` has a `Policy` with:
            {enforced: false}
          `projects/bar` has a `Policy` with:
            {enforced: true}
        The constraint at `organizations/foo` is not enforced.
        The constraint at `projects/bar` is enforced.
        
        Example 3 (RestoreDefault):
          `organizations/foo` has a `Policy` with:
            {enforced: true}
          `projects/bar` has a `Policy` with:
            {RestoreDefault: {}}
        The constraint at `organizations/foo` is enforced.
        The constraint at `projects/bar` is not enforced, because
        `constraint_default` for the `Constraint` is `ALLOW`.

* `..    constraint=amet`
    - The name of the `Constraint` the `Policy` is configuring, for example,
        `constraints/serviceuser.services`.
        
        Immutable after creation.
* `etag=no`
    - An opaque tag indicating the current version of the `Policy`, used for
        concurrency control.
        
        When the `Policy` is returned from either a `GetPolicy` or a
        `ListOrgPolicy` request, this `etag` indicates the version of the current
        `Policy` to use when executing a read-modify-write loop.
        
        When the `Policy` is returned from a `GetEffectivePolicy` request, the
        `etag` will be unset.
        
        When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value
        that was returned from a `GetOrgPolicy` request as part of a
        read-modify-write loop for concurrency control. Not setting the `etag`in a
        `SetOrgPolicy` request will result in an unconditional write of the
        `Policy`.
* `list-policy    all-values=labore`
    - The policy all_values state.
* `allowed-values=eirmod`
    - List of values allowed  at this resource. Can only be set if `all_values`
        is set to `ALL_VALUES_UNSPECIFIED`.
    - Each invocation of this argument appends the given value to the array.
* `denied-values=dolore`
    - List of values denied at this resource. Can only be set if `all_values`
        is set to `ALL_VALUES_UNSPECIFIED`.
    - Each invocation of this argument appends the given value to the array.
* `inherit-from-parent=true`
    - Determines the inheritance behavior for this `Policy`.
        
        By default, a `ListPolicy` set at a resource supercedes any `Policy` set
        anywhere up the resource hierarchy. However, if `inherit_from_parent` is
        set to `true`, then the values from the effective `Policy` of the parent
        resource are inherited, meaning the values set in this `Policy` are
        added to the values inherited up the hierarchy.
        
        Setting `Policy` hierarchies that inherit both allowed values and denied
        values isn&#39;t recommended in most circumstances to keep the configuration
        simple and understandable. However, it is possible to set a `Policy` with
        `allowed_values` set that inherits a `Policy` with `denied_values` set.
        In this case, the values that are allowed must be in `allowed_values` and
        not present in `denied_values`.
        
        For example, suppose you have a `Constraint`
        `constraints/serviceuser.services`, which has a `constraint_type` of
        `list_constraint`, and with `constraint_default` set to `ALLOW`.
        Suppose that at the Organization level, a `Policy` is applied that
        restricts the allowed API activations to {`E1`, `E2`}. Then, if a
        `Policy` is applied to a project below the Organization that has
        `inherit_from_parent` set to `false` and field all_values set to DENY,
        then an attempt to activate any API will be denied.
        
        The following examples demonstrate different possible layerings for
        `projects/bar` parented by `organizations/foo`:
        
        Example 1 (no inherited values):
          `organizations/foo` has a `Policy` with values:
            {allowed_values: “E1” allowed_values:”E2”}
          `projects/bar` has `inherit_from_parent` `false` and values:
            {allowed_values: &#34;E3&#34; allowed_values: &#34;E4&#34;}
        The accepted values at `organizations/foo` are `E1`, `E2`.
        The accepted values at `projects/bar` are `E3`, and `E4`.
        
        Example 2 (inherited values):
          `organizations/foo` has a `Policy` with values:
            {allowed_values: “E1” allowed_values:”E2”}
          `projects/bar` has a `Policy` with values:
            {value: “E3” value: ”E4” inherit_from_parent: true}
        The accepted values at `organizations/foo` are `E1`, `E2`.
        The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`.
        
        Example 3 (inheriting both allowed and denied values):
          `organizations/foo` has a `Policy` with values:
            {allowed_values: &#34;E1&#34; allowed_values: &#34;E2&#34;}
          `projects/bar` has a `Policy` with:
            {denied_values: &#34;E1&#34;}
        The accepted values at `organizations/foo` are `E1`, `E2`.
        The value accepted at `projects/bar` is `E2`.
        
        Example 4 (RestoreDefault):
          `organizations/foo` has a `Policy` with values:
            {allowed_values: “E1” allowed_values:”E2”}
          `projects/bar` has a `Policy` with values:
            {RestoreDefault: {}}
        The accepted values at `organizations/foo` are `E1`, `E2`.
        The accepted values at `projects/bar` are either all or none depending on
        the value of `constraint_default` (if `ALLOW`, all; if
        `DENY`, none).
        
        Example 5 (no policy inherits parent policy):
          `organizations/foo` has no `Policy` set.
          `projects/bar` has no `Policy` set.
        The accepted values at both levels are either all or none depending on
        the value of `constraint_default` (if `ALLOW`, all; if
        `DENY`, none).
        
        Example 6 (ListConstraint allowing all):
          `organizations/foo` has a `Policy` with values:
            {allowed_values: “E1” allowed_values: ”E2”}
          `projects/bar` has a `Policy` with:
            {all: ALLOW}
        The accepted values at `organizations/foo` are `E1`, E2`.
        Any value is accepted at `projects/bar`.
        
        Example 7 (ListConstraint allowing none):
          `organizations/foo` has a `Policy` with values:
            {allowed_values: “E1” allowed_values: ”E2”}
          `projects/bar` has a `Policy` with:
            {all: DENY}
        The accepted values at `organizations/foo` are `E1`, E2`.
        No value is accepted at `projects/bar`.
        
        Example 10 (allowed and denied subtrees of Resource Manager hierarchy):
        Given the following resource hierarchy
          O1-&gt;{F1, F2}; F1-&gt;{P1}; F2-&gt;{P2, P3},
          `organizations/foo` has a `Policy` with values:
            {allowed_values: &#34;under:organizations/O1&#34;}
          `projects/bar` has a `Policy` with:
            {allowed_values: &#34;under:projects/P3&#34;}
            {denied_values: &#34;under:folders/F2&#34;}
        The accepted values at `organizations/foo` are `organizations/O1`,
          `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`,
          `projects/P3`.
        The accepted values at `projects/bar` are `organizations/O1`,
          `folders/F1`, `projects/P1`.
* `suggested-value=aliquyam`
    - Optional. The Google Cloud Console will try to default to a configuration
        that matches the value specified in this `Policy`. If `suggested_value`
        is not set, it will inherit the value specified higher in the hierarchy,
        unless `inherit_from_parent` is `false`.

* `..    update-time=accusam`
    - The time stamp the `Policy` was previously updated. This is set by the
        server, not specified by the caller, and represents the last time a call to
        `SetOrgPolicy` was made for that `Policy`. Any value set by the client will
        be ignored.
* `version=45`
    - Version of the `Policy`. Default version is 0;



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
