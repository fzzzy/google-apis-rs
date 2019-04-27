Watch for changes to Events resources.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/calendar*
* *https://www.googleapis.com/auth/calendar.events*
* *https://www.googleapis.com/auth/calendar.events.readonly*
* *https://www.googleapis.com/auth/calendar.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/calendar*.
You can set the scope for this method like this: `calendar3 --scope <scope> events watch ...`
# Required Scalar Argument
* **&lt;calendar-id&gt;** *(string)*
    - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the &#34;primary&#34; keyword.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Channel:
  address: string
  expiration: string
  id: string
  kind: string
  params: { string: string }
  payload: boolean
  resource-id: string
  resource-uri: string
  token: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    address=ipsum`
    - The address where notifications are delivered for this channel.
* `expiration=labore`
    - Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
* `id=lorem`
    - A UUID or similar unique string that identifies this channel.
* `kind=aliquyam`
    - Identifies this as a notification channel used to watch for changes to a resource. Value: the fixed string &#34;api#channel&#34;.
* `params=key=sanctus`
    - Additional parameters controlling delivery channel behavior. Optional.
    - the value will be associated with the given `key`
* `payload=true`
    - A Boolean value to indicate whether payload is wanted. Optional.
* `resource-id=consetetur`
    - An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
* `resource-uri=et`
    - A version-specific identifier for the watched resource.
* `token=justo`
    - An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
* `type=kasd`
    - The type of delivery mechanism used for this channel.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


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

* **-p max-results=integer**
    - Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.

* **-p show-hidden-invitations=boolean**
    - Whether to include hidden invitations in the result. Optional. The default is False.

* **-p page-token=string**
    - Token specifying which result page to return. Optional.

* **-p updated-min=string**
    - Lower bound for an event&#39;s last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time.

* **-p time-min=string**
    - Lower bound (inclusive) for an event&#39;s end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, e.g., 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but will be ignored. If timeMax is set, timeMin must be smaller than timeMax.

* **-p sync-token=string**
    - Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
        There are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.
        
        These are: 
        - iCalUID 
        - orderBy 
        - privateExtendedProperty 
        - q 
        - sharedExtendedProperty 
        - timeMin 
        - timeMax 
        - updatedMin If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
        Learn more about incremental synchronization.
        Optional. The default is to return all entries.

* **-p i-cal-uid=string**
    - Specifies event ID in the iCalendar format to be included in the response. Optional.

* **-p time-max=string**
    - Upper bound (exclusive) for an event&#39;s start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, e.g., 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but will be ignored. If timeMin is set, timeMax must be greater than timeMin.

* **-p private-extended-property=string**
    - Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints.

* **-p order-by=string**
    - The order of the events returned in the result. Optional. The default is an unspecified, stable order.

* **-p shared-extended-property=string**
    - Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints.

* **-p show-deleted=boolean**
    - Whether to include deleted events (with status equals &#34;cancelled&#34;) in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False.

* **-p single-events=boolean**
    - Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False.

* **-p q=string**
    - Free text search terms to find events that match these terms in any field, except for extended properties. Optional.

* **-p always-include-email=boolean**
    - Whether to always include a value in the email field for the organizer, creator and attendees, even if no real email is available (i.e. a generated, non-working value will be provided). The use of this option is discouraged and should only be used by clients which cannot handle the absence of an email address value in the mentioned places. Optional. The default is False.

* **-p time-zone=string**
    - Time zone used in the response. Optional. The default is the time zone of the calendar.

* **-p max-attendees=integer**
    - The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

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
