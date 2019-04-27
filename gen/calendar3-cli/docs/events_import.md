Imports an event. This operation is used to add a private copy of an existing event to a calendar.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/calendar*
* *https://www.googleapis.com/auth/calendar.events*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/calendar*.
You can set the scope for this method like this: `calendar3 --scope <scope> events import ...`
# Required Scalar Argument
* **&lt;calendar-id&gt;** *(string)*
    - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the &#34;primary&#34; keyword.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Event:
  anyone-can-add-self: boolean
  attendees-omitted: boolean
  color-id: string
  conference-data:
    conference-id: string
    conference-solution:
      icon-uri: string
      key:
        type: string
      name: string
    create-request:
      conference-solution-key:
        type: string
      request-id: string
      status:
        status-code: string
    notes: string
    parameters:
      add-on-parameters:
        parameters: { string: string }
    signature: string
  created: string
  creator:
    display-name: string
    email: string
    id: string
    self: boolean
  description: string
  end:
    date: string
    date-time: string
    time-zone: string
  end-time-unspecified: boolean
  etag: string
  extended-properties:
    private: { string: string }
    shared: { string: string }
  gadget:
    display: string
    height: integer
    icon-link: string
    link: string
    preferences: { string: string }
    title: string
    type: string
    width: integer
  guests-can-invite-others: boolean
  guests-can-modify: boolean
  guests-can-see-other-guests: boolean
  hangout-link: string
  html-link: string
  i-cal-uid: string
  id: string
  kind: string
  location: string
  locked: boolean
  organizer:
    display-name: string
    email: string
    id: string
    self: boolean
  original-start-time:
    date: string
    date-time: string
    time-zone: string
  private-copy: boolean
  recurrence: [string]
  recurring-event-id: string
  reminders:
    use-default: boolean
  sequence: integer
  source:
    title: string
    url: string
  start:
    date: string
    date-time: string
    time-zone: string
  status: string
  summary: string
  transparency: string
  updated: string
  visibility: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    anyone-can-add-self=true`
    - Whether anyone can invite themselves to the event (currently works for Google+ events only). Optional. The default is False.
* `attendees-omitted=true`
    - Whether attendees may have been omitted from the event&#39;s representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant&#39;s response. Optional. The default is False.
* `color-id=clita`
    - The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional.
* `conference-data    conference-id=diam`
    - The ID of the conference.
        Can be used by developers to keep track of conferences, should not be displayed to users.
        Values for solution types:  
        - &#34;eventHangout&#34;: unset.
        - &#34;eventNamedHangout&#34;: the name of the Hangout.
        - &#34;hangoutsMeet&#34;: the 10-letter meeting code, for example &#34;aaa-bbbb-ccc&#34;.  Optional.
* `conference-solution    icon-uri=justo`
    - The user-visible icon for this solution.
* `key    type=est`
    - The conference solution type.
        If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
        The possible values are:  
        - &#34;eventHangout&#34; for Hangouts for consumers (http://hangouts.google.com)
        - &#34;eventNamedHangout&#34; for classic Hangouts for G Suite users (http://hangouts.google.com)
        - &#34;hangoutsMeet&#34; for Hangouts Meet (http://meet.google.com)

* `..    name=clita`
    - The user-visible name of this solution. Not localized.

* `..create-request.conference-solution-key    type=invidunt`
    - The conference solution type.
        If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
        The possible values are:  
        - &#34;eventHangout&#34; for Hangouts for consumers (http://hangouts.google.com)
        - &#34;eventNamedHangout&#34; for classic Hangouts for G Suite users (http://hangouts.google.com)
        - &#34;hangoutsMeet&#34; for Hangouts Meet (http://meet.google.com)

* `..    request-id=ut`
    - The client-generated unique ID for this request.
        Clients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored.
* `status    status-code=dolores`
    - The current status of the conference create request. Read-only.
        The possible values are:  
        - &#34;pending&#34;: the conference create request is still being processed.
        - &#34;success&#34;: the conference create request succeeded, the entry points are populated.
        - &#34;failure&#34;: the conference create request failed, there are no entry points.


* `...    notes=eos`
    - Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional.
* `parameters.add-on-parameters    parameters=key=voluptua.`
    - No description provided.
    - the value will be associated with the given `key`


* `...    signature=duo`
    - The signature of the conference data.
        Genereated on server side. Must be preserved while copying the conference data between events, otherwise the conference data will not be copied.
        Unset for a conference with a failed create request.
        Optional for a conference with a pending create request.

* `..    created=sed`
    - Creation time of the event (as a RFC3339 timestamp). Read-only.
* `creator    display-name=aliquyam`
    - The creator&#39;s name, if available.
* `email=ea`
    - The creator&#39;s email address, if available.
* `id=ea`
    - The creator&#39;s Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API
* `self=false`
    - Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

* `..    description=dolor`
    - Description of the event. Optional.
* `end    date=diam`
    - The date, in the format &#34;yyyy-mm-dd&#34;, if this is an all-day event.
* `date-time=kasd`
    - The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.
* `time-zone=invidunt`
    - The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. &#34;Europe/Zurich&#34;.) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

* `..    end-time-unspecified=true`
    - Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.
* `etag=lorem`
    - ETag of the resource.
* `extended-properties    private=key=clita`
    - Properties that are private to the copy of the event that appears on this calendar.
    - the value will be associated with the given `key`
* `shared=key=invidunt`
    - Properties that are shared between copies of the event on other attendees&#39; calendars.
    - the value will be associated with the given `key`

* `..gadget    display=eirmod`
    - The gadget&#39;s display mode. Optional. Possible values are:  
        - &#34;icon&#34; - The gadget displays next to the event&#39;s title in the calendar view. 
        - &#34;chip&#34; - The gadget displays when the event is clicked.
* `height=24`
    - The gadget&#39;s height in pixels. The height must be an integer greater than 0. Optional.
* `icon-link=consetetur`
    - The gadget&#39;s icon URL. The URL scheme must be HTTPS.
* `link=et`
    - The gadget&#39;s URL. The URL scheme must be HTTPS.
* `preferences=key=sed`
    - Preferences.
    - the value will be associated with the given `key`
* `title=sit`
    - The gadget&#39;s title.
* `type=takimata`
    - The gadget&#39;s type.
* `width=58`
    - The gadget&#39;s width in pixels. The width must be an integer greater than 0. Optional.

* `..    guests-can-invite-others=false`
    - Whether attendees other than the organizer can invite others to the event. Optional. The default is True.
* `guests-can-modify=true`
    - Whether attendees other than the organizer can modify the event. Optional. The default is False.
* `guests-can-see-other-guests=true`
    - Whether attendees other than the organizer can see who the event&#39;s attendees are. Optional. The default is True.
* `hangout-link=lorem`
    - An absolute link to the Google+ hangout associated with this event. Read-only.
* `html-link=diam`
    - An absolute link to this event in the Google Calendar Web UI. Read-only.
* `i-cal-uid=ut`
    - Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
        Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.
* `id=ut`
    - Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
        - characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
        - the length of the ID must be between 5 and 1024 characters 
        - the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
        If you do not specify an ID, it will be automatically generated by the server.
        Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.
* `kind=amet.`
    - Type of the resource (&#34;calendar#event&#34;).
* `location=ipsum`
    - Geographic location of the event as free-form text. Optional.
* `locked=false`
    - Whether this is a locked event copy where no changes can be made to the main event fields &#34;summary&#34;, &#34;description&#34;, &#34;location&#34;, &#34;start&#34;, &#34;end&#34; or &#34;recurrence&#34;. The default is False. Read-Only.
* `organizer    display-name=dolor`
    - The organizer&#39;s name, if available.
* `email=sea`
    - The organizer&#39;s email address, if available. It must be a valid email address as per RFC5322.
* `id=ut`
    - The organizer&#39;s Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API
* `self=true`
    - Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

* `..original-start-time    date=sanctus`
    - The date, in the format &#34;yyyy-mm-dd&#34;, if this is an all-day event.
* `date-time=voluptua.`
    - The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.
* `time-zone=dolor`
    - The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. &#34;Europe/Zurich&#34;.) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

* `..    private-copy=true`
    - Whether this is a private event copy where changes are not shared with other copies on other calendars. Optional. Immutable. The default is False.
* `recurrence=et`
    - List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.
    - Each invocation of this argument appends the given value to the array.
* `recurring-event-id=vero`
    - For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable.
* `reminders    use-default=true`
    - Whether the default reminders of the calendar apply to the event.

* `..    sequence=71`
    - Sequence number as per iCalendar.
* `source    title=et`
    - Title of the source; for example a title of a web page or an email subject.
* `url=ipsum`
    - URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS.

* `..start    date=justo`
    - The date, in the format &#34;yyyy-mm-dd&#34;, if this is an all-day event.
* `date-time=dolore`
    - The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.
* `time-zone=vero`
    - The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. &#34;Europe/Zurich&#34;.) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

* `..    status=dolor`
    - Status of the event. Optional. Possible values are:  
        - &#34;confirmed&#34; - The event is confirmed. This is the default status. 
        - &#34;tentative&#34; - The event is tentatively confirmed. 
        - &#34;cancelled&#34; - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
        A cancelled status represents two different states depending on the event type:  
        - Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
        Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
        - All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
        Deleted events are only guaranteed to have the id field populated.   On the organizer&#39;s calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
        If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated.
* `summary=takimata`
    - Title of the event.
* `transparency=et`
    - Whether the event blocks time on the calendar. Optional. Possible values are:  
        - &#34;opaque&#34; - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
        - &#34;transparent&#34; - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.
* `updated=nonumy`
    - Last modification time of the event (as a RFC3339 timestamp). Read-only.
* `visibility=et`
    - Visibility of the event. Optional. Possible values are:  
        - &#34;default&#34; - Uses the default visibility for events on the calendar. This is the default value. 
        - &#34;public&#34; - The event is public and event details are visible to all readers of the calendar. 
        - &#34;private&#34; - The event is private and only event attendees may view event details. 
        - &#34;confidential&#34; - The event is private. This value is provided for compatibility reasons.


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

* **-p supports-attachments=boolean**
    - Whether API client performing operation supports event attachments. Optional. The default is False.

* **-p conference-data-version=integer**
    - Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event&#39;s body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.

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
