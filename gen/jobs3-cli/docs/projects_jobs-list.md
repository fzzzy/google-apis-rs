Lists jobs by filter.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/jobs*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `jobs3 --scope <scope> projects jobs-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        The resource name of the project under which the job is created.
        
        The format is &#34;projects/{project_id}&#34;, for example,
        &#34;projects/api-test-project&#34;.

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
    - Optional.
        
        The starting point of a query result.

* **-p filter=string**
    - Required.
        
        The filter string specifies the jobs to be enumerated.
        
        Supported operator: =, AND
        
        The fields eligible for filtering are:
        
        * `companyName` (Required)
        * `requisitionId` (Optional)
        
        Sample Query:
        
        * companyName = &#34;projects/api-test-project/companies/123&#34;
        * companyName = &#34;projects/api-test-project/companies/123&#34; AND requisitionId
        = &#34;req-1&#34;

* **-p page-size=integer**
    - Optional.
        
        The maximum number of jobs to be returned per page of results.
        
        If job_view is set to JobView.JOB_VIEW_ID_ONLY, the maximum allowed
        page size is 1000. Otherwise, the maximum allowed page size is 100.
        
        Default is 100 if empty or a number &lt; 1 is specified.

* **-p job-view=string**
    - Optional.
        
        The desired job attributes returned for jobs in the
        search response. Defaults to JobView.JOB_VIEW_FULL if no value is
        specified.

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
