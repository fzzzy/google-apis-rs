Returns a list of activities visible to the current logged in user. Visible activities are determined by the visiblity settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/activity* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/activity*.
You can set the scope for this method like this: `appsactivity1 --scope <scope> activities list ...`

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
    - The maximum number of events to return on a page. The response includes a continuation token if there are more events.

* **-p grouping-strategy=string**
    - Indicates the strategy to use when grouping singleEvents items in the associated combinedEvent object.

* **-p source=string**
    - The Google service from which to return activities. Possible values of source are: 
        - drive.google.com

* **-p user-id=string**
    - Indicates the user to return activity for. Use the special value me to indicate the currently authenticated user.

* **-p drive-ancestor-id=string**
    - Identifies the Drive folder containing the items for which to return activities.

* **-p page-token=string**
    - A token to retrieve a specific page of results.

* **-p drive-file-id=string**
    - Identifies the Drive item to return activities for.

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
