Lists the tasks in a queue.

By default, only the BASIC view is retrieved
due to performance considerations;
response_view controls the
subset of information which is returned.

The tasks may be returned in any order. The ordering may change at any
time.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudtasks2-beta2 --scope <scope> projects locations-queues-tasks-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        The queue name. For example:
        `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`

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

* **-p response-view=string**
    - The response_view specifies which subset of the Task will be
        returned.
        
        By default response_view is BASIC; not all
        information is retrieved by default because some data, such as
        payloads, might be desirable to return only when needed because
        of its large size or because of the sensitivity of data that it
        contains.
        
        Authorization for FULL requires
        `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/)
        permission on the Task resource.

* **-p page-token=string**
    - A token identifying the page of results to return.
        
        To request the first page results, page_token must be empty. To
        request the next page of results, page_token must be the value of
        next_page_token returned
        from the previous call to ListTasks
        method.
        
        The page token is valid for only 2 hours.

* **-p page-size=integer**
    - Requested page size. Fewer tasks than requested might be returned.
        
        The maximum page size is 1000. If unspecified, the page size will
        be the maximum. Fewer tasks than requested might be returned,
        even if more tasks exist; use
        next_page_token in the
        response to determine if more tasks exist.

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
