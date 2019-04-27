Returns an event.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/calendar*
* *https://www.googleapis.com/auth/calendar.events*
* *https://www.googleapis.com/auth/calendar.events.readonly*
* *https://www.googleapis.com/auth/calendar.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/calendar.events.readonly*.
You can set the scope for this method like this: `calendar3 --scope <scope> events get ...`
# Required Scalar Arguments
* **&lt;calendar-id&gt;** *(string)*
    - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the &#34;primary&#34; keyword.
* **&lt;event-id&gt;** *(string)*
    - Event identifier.

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

* **-p always-include-email=boolean**
    - Whether to always include a value in the email field for the organizer, creator and attendees, even if no real email is available (i.e. a generated, non-working value will be provided). The use of this option is discouraged and should only be used by clients which cannot handle the absence of an email address value in the mentioned places. Optional. The default is False.

* **-p max-attendees=integer**
    - The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

* **-p time-zone=string**
    - Time zone used in the response. Optional. The default is the time zone of the calendar.

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
