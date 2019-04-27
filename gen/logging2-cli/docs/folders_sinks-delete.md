Deletes a sink. If the sink has a unique writer_identity, then that service account is also deleted.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/logging.admin*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `logging2 --scope <scope> folders sinks-delete ...`
# Required Scalar Argument
* **&lt;sink-name&gt;** *(string)*
    - Required. The full resource name of the sink to delete, including the parent resource and the sink identifier:
        &#34;projects/[PROJECT_ID]/sinks/[SINK_ID]&#34;
        &#34;organizations/[ORGANIZATION_ID]/sinks/[SINK_ID]&#34;
        &#34;billingAccounts/[BILLING_ACCOUNT_ID]/sinks/[SINK_ID]&#34;
        &#34;folders/[FOLDER_ID]/sinks/[SINK_ID]&#34;
        Example: &#34;projects/my-project-id/sinks/my-sink-id&#34;.

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
