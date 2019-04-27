Lists DlpJobs that match the specified filter in the request.
See https://cloud.google.com/dlp/docs/inspecting-storage and
https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dlp2 --scope <scope> projects dlp-jobs-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The parent resource name, for example projects/my-project-id.

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

* **-p order-by=string**
    - Optional comma separated list of fields to order by,
        followed by `asc` or `desc` postfix. This list is case-insensitive,
        default sorting order is ascending, redundant space characters are
        insignificant.
        
        Example: `name asc, end_time asc, create_time desc`
        
        Supported fields are:
        
        - `create_time`: corresponds to time the job was created.
        - `end_time`: corresponds to time the job ended.
        - `name`: corresponds to job&#39;s name.
        - `state`: corresponds to `state`

* **-p filter=string**
    - Optional. Allows filtering.
        
        Supported syntax:
        
        * Filter expressions are made up of one or more restrictions.
        * Restrictions can be combined by `AND` or `OR` logical operators. A
        sequence of restrictions implicitly uses `AND`.
        * A restriction has the form of `&lt;field&gt; &lt;operator&gt; &lt;value&gt;`.
        * Supported fields/values for inspect jobs:
            - `state` - PENDING|RUNNING|CANCELED|FINISHED|FAILED
            - `inspected_storage` - DATASTORE|CLOUD_STORAGE|BIGQUERY
            - `trigger_name` - The resource name of the trigger that created job.
        * Supported fields for risk analysis jobs:
            - `state` - RUNNING|CANCELED|FINISHED|FAILED
        * The operator must be `=` or `!=`.
        
        Examples:
        
        * inspected_storage = cloud_storage AND state = done
        * inspected_storage = cloud_storage OR inspected_storage = bigquery
        * inspected_storage = cloud_storage AND (state = done OR state = canceled)
        
        The length of this field should be no more than 500 characters.

* **-p page-size=integer**
    - The standard list page size.

* **-p page-token=string**
    - The standard list page token.

* **-p type=string**
    - The type of job. Defaults to `DlpJobType.INSPECT`

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
