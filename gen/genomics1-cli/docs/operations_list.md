Lists operations that match the specified filter in the request.
Authorization requires the following [Google IAM](https://cloud.google.com/iam) permission&amp;#58;

* `genomics.operations.list`
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/genomics*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `genomics1 --scope <scope> operations list ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the operation&#39;s parent resource.

# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p page-token=string**
    - The standard list page token.

* **-p page-size=integer**
    - The maximum number of results to return. If unspecified, defaults to
        256. The maximum value is 2048.

* **-p filter=string**
    - A string for filtering Operations.
        In v2alpha1, the following filter fields are supported&amp;#58;
        
        * createTime&amp;#58; The time this job was created
        * events&amp;#58; The set of event (names) that have occurred while running
          the pipeline.  The &amp;#58; operator can be used to determine if a
          particular event has occurred.
        * error&amp;#58; If the pipeline is running, this value is NULL.  Once the
          pipeline finishes, the value is the standard Google error code.
        * labels.key or labels.&#34;key with space&#34; where key is a label key.
        * done&amp;#58; If the pipeline is running, this value is false. Once the
          pipeline finishes, the value is true.
        
        In v1 and v1alpha2, the following filter fields are supported&amp;#58;
        
        * projectId&amp;#58; Required. Corresponds to
          OperationMetadata.projectId.
        * createTime&amp;#58; The time this job was created, in seconds from the
          [epoch](http://en.wikipedia.org/wiki/Unix_time). Can use `&gt;=` and/or `&lt;=`
          operators.
        * status&amp;#58; Can be `RUNNING`, `SUCCESS`, `FAILURE`, or `CANCELED`. Only
          one status may be specified.
        * labels.key where key is a label key.
        
        Examples&amp;#58;
        
        * `projectId = my-project AND createTime &gt;= 1432140000`
        * `projectId = my-project AND createTime &gt;= 1432140000 AND createTime &lt;= 1432150000 AND status = RUNNING`
        * `projectId = my-project AND labels.color = *`
        * `projectId = my-project AND labels.color = red`

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
