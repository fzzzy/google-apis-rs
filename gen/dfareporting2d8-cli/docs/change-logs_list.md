Retrieves a list of change logs. This method supports paging.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting2d8 --scope <scope> change-logs list ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.

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

* **-p search-string=string**
    - Select only change logs whose object ID, user name, old or new values match the search string.

* **-p min-change-time=string**
    - Select only change logs whose change time is before the specified minChangeTime.The time should be formatted as an RFC3339 date/time string. For example, for 10:54 PM on July 18th, 2015, in the America/New York time zone, the format is &#34;2015-07-18T22:54:00-04:00&#34;. In other words, the year, month, day, the letter T, the hour (24-hour clock system), minute, second, and then the time zone offset.

* **-p user-profile-ids=string**
    - Select only change logs with these user profile IDs.

* **-p max-change-time=string**
    - Select only change logs whose change time is before the specified maxChangeTime.The time should be formatted as an RFC3339 date/time string. For example, for 10:54 PM on July 18th, 2015, in the America/New York time zone, the format is &#34;2015-07-18T22:54:00-04:00&#34;. In other words, the year, month, day, the letter T, the hour (24-hour clock system), minute, second, and then the time zone offset.

* **-p object-type=string**
    - Select only change logs with the specified object type.

* **-p page-token=string**
    - Value of the nextPageToken from the previous result page.

* **-p action=string**
    - Select only change logs with the specified action.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p object-ids=string**
    - Select only change logs with these object IDs.

* **-p ids=string**
    - Select only change logs with these IDs.

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
