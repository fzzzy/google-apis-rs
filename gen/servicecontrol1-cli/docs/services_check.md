Checks whether an operation on a service should be allowed to proceed
based on the configuration of the service and related policies. It must be
called before the operation is executed.

If feasible, the client should cache the check results and reuse them for
60 seconds. In case of any server errors, the client should rely on the
cached results for much longer time to avoid outage.
WARNING: There is general 60s delay for the configuration and policy
propagation, therefore callers MUST NOT depend on the `Check` method having
the latest policy information.

NOTE: the CheckRequest has the size limit of 64KB.

This method requires the `servicemanagement.services.check` permission
on the specified service. For more information, see
[Cloud IAM](https://cloud.google.com/iam).
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/servicecontrol*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `servicecontrol1 --scope <scope> services check ...`
# Required Scalar Argument
* **&lt;service-name&gt;** *(string)*
    - The service name as specified in its service configuration. For example,
        `&#34;pubsub.googleapis.com&#34;`.
        
        See
        [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
        for the definition of a service name.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CheckRequest:
  operation:
    consumer-id: string
    end-time: string
    importance: string
    labels: { string: string }
    operation-id: string
    operation-name: string
    quota-properties:
      quota-mode: string
    resource-container: string
    start-time: string
    user-labels: { string: string }
  request-project-settings: boolean
  service-config-id: string
  skip-activation-check: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .operation    consumer-id=kasd`
    - Identity of the consumer who is using the service.
        This field should be filled in for the operations initiated by a
        consumer, but not for service-initiated operations that are
        not related to a specific consumer.
        
        This can be in one of the following formats:
          project:&lt;project_id&gt;,
          project_number:&lt;project_number&gt;,
          api_key:&lt;api_key&gt;.
* `end-time=accusam`
    - End time of the operation.
        Required when the operation is used in ServiceController.Report,
        but optional when the operation is used in ServiceController.Check.
* `importance=takimata`
    - DO NOT USE. This is an experimental field.
* `labels=key=justo`
    - Labels describing the operation. Only the following labels are allowed:
        
        - Labels describing monitored resources as defined in
          the service configuration.
        - Default labels of metric values. When specified, labels defined in the
          metric value override these default.
        - The following labels defined by Google Cloud Platform:
            - `cloud.googleapis.com/location` describing the location where the
               operation happened,
            - `servicecontrol.googleapis.com/user_agent` describing the user agent
               of the API request,
            - `servicecontrol.googleapis.com/service_agent` describing the service
               used to handle the API request (e.g. ESP),
            - `servicecontrol.googleapis.com/platform` describing the platform
               where the API is served, such as App Engine, Compute Engine, or
               Kubernetes Engine.
    - the value will be associated with the given `key`
* `operation-id=amet.`
    - Identity of the operation. This must be unique within the scope of the
        service that generated the operation. If the service calls
        Check() and Report() on the same operation, the two calls should carry
        the same id.
        
        UUID version 4 is recommended, though not required.
        In scenarios where an operation is computed from existing information
        and an idempotent id is desirable for deduplication purpose, UUID version 5
        is recommended. See RFC 4122 for details.
* `operation-name=erat`
    - Fully qualified name of the operation. Reserved for future use.
* `quota-properties    quota-mode=labore`
    - Quota mode for this operation.

* `..    resource-container=sea`
    - DO NOT USE. This field is deprecated, use &#34;resources&#34; field instead.
        The resource name of the parent of a resource in the resource hierarchy.
        
        This can be in one of the following formats:
            - “projects/&lt;project-id or project-number&gt;”
            - “folders/&lt;folder-id&gt;”
            - “organizations/&lt;organization-id&gt;”
* `start-time=nonumy`
    - Required. Start time of the operation.
* `user-labels=key=dolores`
    - User defined labels for the resource that this operation is associated
        with. Only a combination of 1000 user labels per consumer project are
        allowed.
    - the value will be associated with the given `key`

* `..    request-project-settings=false`
    - Requests the project settings to be returned as part of the check response.
* `service-config-id=sadipscing`
    - Specifies which version of service configuration should be used to process
        the request.
        
        If unspecified or no matching version can be found, the
        latest one will be used.
* `skip-activation-check=true`
    - Indicates if service activation check should be skipped for this request.
        Default behavior is to perform the check and apply relevant quota.


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
