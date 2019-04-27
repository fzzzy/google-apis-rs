Updates an instance, and begins allocating or releasing resources
as requested. The returned long-running
operation can be used to track the
progress of updating the instance. If the named instance does not
exist, returns `NOT_FOUND`.

Immediately upon completion of this request:

  * For resource types for which a decrease in the instance&#39;s allocation
    has been requested, billing is based on the newly-requested level.

Until completion of the returned operation:

  * Cancelling the operation sets its metadata&#39;s
    cancel_time, and begins
    restoring resources to their pre-request values. The operation
    is guaranteed to succeed at undoing all resource changes,
    after which point it terminates with a `CANCELLED` status.
  * All other attempts to modify the instance are rejected.
  * Reading the instance via the API continues to give the pre-request
    resource levels.

Upon completion of the returned operation:

  * Billing begins for all successfully-allocated resources (some types
    may have lower than the requested levels).
  * All newly-reserved resources are available for serving the instance&#39;s
    tables.
  * The instance&#39;s new resource levels are readable via the API.

The returned long-running operation will
have a name of the format `&lt;instance_name&gt;/operations/&lt;operation_id&gt;` and
can be used to track the instance modification.  The
metadata field type is
UpdateInstanceMetadata.
The response field type is
Instance, if successful.

Authorization requires `spanner.instances.update` permission on
resource name.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/spanner.admin*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `spanner1 --scope <scope> projects instances-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Required. A unique identifier for the instance, which cannot be changed
        after the instance is created. Values are of the form
        `projects/&lt;project&gt;/instances/a-z*[a-z0-9]`. The final
        segment of the name must be between 6 and 30 characters in length.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
UpdateInstanceRequest:
  field-mask: string
  instance:
    config: string
    display-name: string
    labels: { string: string }
    name: string
    node-count: integer
    state: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    field-mask=kasd`
    - Required. A mask specifying which fields in [][google.spanner.admin.instance.v1.UpdateInstanceRequest.instance] should be updated.
        The field mask must always be specified; this prevents any future fields in
        [][google.spanner.admin.instance.v1.Instance] from being erased accidentally by clients that do not know
        about them.
* `instance    config=invidunt`
    - Required. The name of the instance&#39;s configuration. Values are of the form
        `projects/&lt;project&gt;/instanceConfigs/&lt;configuration&gt;`. See
        also InstanceConfig and
        ListInstanceConfigs.
* `display-name=rebum.`
    - Required. The descriptive name for this instance as it appears in UIs.
        Must be unique per project and between 4 and 30 characters in length.
* `labels=key=lorem`
    - Cloud Labels are a flexible and lightweight mechanism for organizing cloud
        resources into groups that reflect a customer&#39;s organizational needs and
        deployment strategies. Cloud Labels can be used to filter collections of
        resources. They can be used to control how resource metrics are aggregated.
        And they can be used as arguments to policy management rules (e.g. route,
        firewall, load balancing, etc.).
        
         * Label keys must be between 1 and 63 characters long and must conform to
           the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`.
         * Label values must be between 0 and 63 characters long and must conform
           to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`.
         * No more than 64 labels can be associated with a given resource.
        
        See https://goo.gl/xmQnxf for more information on and examples of labels.
        
        If you plan to use labels in your own code, please note that additional
        characters may be allowed in the future. And so you are advised to use an
        internal label representation, such as JSON, which doesn&#39;t rely upon
        specific characters being disallowed.  For example, representing labels
        as the string:  name + &#34;_&#34; + value  would prove problematic if we were to
        allow &#34;_&#34; in a future release.
    - the value will be associated with the given `key`
* `name=clita`
    - Required. A unique identifier for the instance, which cannot be changed
        after the instance is created. Values are of the form
        `projects/&lt;project&gt;/instances/a-z*[a-z0-9]`. The final
        segment of the name must be between 6 and 30 characters in length.
* `node-count=64`
    - Required. The number of nodes allocated to this instance. This may be zero
        in API responses for instances that are not yet in state `READY`.
        
        See [the documentation](https://cloud.google.com/spanner/docs/instances#node_count)
        for more information about nodes.
* `state=eirmod`
    - Output only. The current instance state. For
        CreateInstance, the state must be
        either omitted or set to `CREATING`. For
        UpdateInstance, the state must be
        either omitted or set to `READY`.



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
