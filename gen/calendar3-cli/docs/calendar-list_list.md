Returns the calendars on the user&#39;s calendar list.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/calendar*
* *https://www.googleapis.com/auth/calendar.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/calendar.readonly*.
You can set the scope for this method like this: `calendar3 --scope <scope> calendar-list list ...`

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

* **-p show-deleted=boolean**
    - Whether to include deleted calendar list entries in the result. Optional. The default is False.

* **-p sync-token=string**
    - Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won&#39;t be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.
        To ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.
        If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
        Learn more about incremental synchronization.
        Optional. The default is to return all entries.

* **-p page-token=string**
    - Token specifying which result page to return. Optional.

* **-p show-hidden=boolean**
    - Whether to show hidden entries. Optional. The default is False.

* **-p min-access-role=string**
    - The minimum access role for the user in the returned entries. Optional. The default is no restriction.

* **-p max-results=integer**
    - Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.

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
