CreateOfflineProfile creates a new profile resource in the offline mode.
The client provides the profile to create along with the profile bytes, the
server records it.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/monitoring*
* *https://www.googleapis.com/auth/monitoring.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudprofiler2 --scope <scope> projects profiles-create-offline ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Parent project to create the profile in.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Profile:
  deployment:
    labels: { string: string }
    project-id: string
    target: string
  duration: string
  labels: { string: string }
  name: string
  profile-bytes: string
  profile-type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .deployment    labels=key=et`
    - Labels identify the deployment within the user universe and same target.
        Validation regex for label names: `^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`.
        Value for an individual label must be &lt;= 512 bytes, the total
        size of all label names and values must be &lt;= 1024 bytes.
        
        Label named &#34;language&#34; can be used to record the programming language of
        the profiled deployment. The standard choices for the value include &#34;java&#34;,
        &#34;go&#34;, &#34;python&#34;, &#34;ruby&#34;, &#34;nodejs&#34;, &#34;php&#34;, &#34;dotnet&#34;.
        
        For deployments running on Google Cloud Platform, &#34;zone&#34; or &#34;region&#34; label
        should be present describing the deployment location. An example of a zone
        is &#34;us-central1-a&#34;, an example of a region is &#34;us-central1&#34; or
        &#34;us-central&#34;.
    - the value will be associated with the given `key`
* `project-id=dolores`
    - Project ID is the ID of a cloud project.
        Validation regex: `^a-z{4,61}[a-z0-9]$`.
* `target=kasd`
    - Target is the service name used to group related deployments:
        * Service name for GAE Flex / Standard.
        * Cluster and container name for GKE.
        * User-specified string for direct GCE profiling (e.g. Java).
        * Job name for Dataflow.
        Validation regex: `^[a-z]([-a-z0-9_.]{0,253}[a-z0-9])?$`.

* `..    duration=accusam`
    - Duration of the profiling session.
        Input (for the offline mode) or output (for the online mode).
        The field represents requested profiling duration. It may slightly differ
        from the effective profiling duration, which is recorded in the profile
        data, in case the profiling can&#39;t be stopped immediately (e.g. in case
        stopping the profiling is handled asynchronously).
* `labels=key=takimata`
    - Input only. Labels associated to this specific profile. These labels will
        get merged with the deployment labels for the final data set.  See
        documentation on deployment labels for validation rules and limits.
    - the value will be associated with the given `key`
* `name=justo`
    - Output only. Opaque, server-assigned, unique ID for this profile.
* `profile-bytes=amet.`
    - Input only. Profile bytes, as a gzip compressed serialized proto, the
        format is https://github.com/google/pprof/blob/master/proto/profile.proto.
* `profile-type=erat`
    - Type of profile.
        For offline mode, this must be specified when creating the profile. For
        online mode it is assigned and returned by the server.


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
