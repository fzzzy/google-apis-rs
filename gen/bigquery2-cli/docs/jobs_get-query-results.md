Retrieves the results of a query job.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/bigquery*
* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud-platform.read-only*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/bigquery*.
You can set the scope for this method like this: `bigquery2 --scope <scope> jobs get-query-results ...`
# Required Scalar Arguments
* **&lt;project-id&gt;** *(string)*
    - [Required] Project ID of the query job
* **&lt;job-id&gt;** *(string)*
    - [Required] Job ID of the query job

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

* **-p location=string**
    - [Experimental] The geographic location where the job should run. Required except for US and EU. See details at https://cloud.google.com/bigquery/docs/dataset-locations#specifying_your_location.

* **-p start-index=string**
    - Zero-based index of the starting row

* **-p timeout-ms=integer**
    - How long to wait for the query to complete, in milliseconds, before returning. Default is 10 seconds. If the timeout passes before the job completes, the &#39;jobComplete&#39; field in the response will be false

* **-p page-token=string**
    - Page token, returned by a previous call, to request the next page of results

* **-p max-results=integer**
    - Maximum number of results to read

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
