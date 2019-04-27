Lists documents.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/datastore*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `firestore1-beta1 --scope <scope> projects databases-documents-list ...`
# Required Scalar Arguments
* **&lt;parent&gt;** *(string)*
    - The parent resource name. In the format:
        `projects/{project_id}/databases/{database_id}/documents` or
        `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        For example:
        `projects/my-project/databases/my-database/documents` or
        `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
* **&lt;collection-id&gt;** *(string)*
    - The collection ID, relative to `parent`, to list. For example: `chatrooms`
        or `messages`.

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

* **-p page-size=integer**
    - The maximum number of documents to return.

* **-p read-time=string**
    - Reads documents as they were at the given time.
        This may not be older than 60 seconds.

* **-p mask-field-paths=string**
    - The list of field paths in the mask. See Document.fields for a field
        path syntax reference.

* **-p show-missing=boolean**
    - If the list should show missing documents. A missing document is a
        document that does not exist but has sub-documents. These documents will
        be returned with a key but will not have fields, Document.create_time,
        or Document.update_time set.
        
        Requests with `show_missing` may not specify `where` or
        `order_by`.

* **-p order-by=string**
    - The order to sort results by. For example: `priority desc, name`.

* **-p transaction=string**
    - Reads documents in a transaction.

* **-p page-token=string**
    - The `next_page_token` value returned from a previous List request, if any.

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
