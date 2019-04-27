Inserts an existing calendar into the user&#39;s calendar list.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/calendar* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/calendar*.
You can set the scope for this method like this: `calendar3 --scope <scope> calendar-list insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CalendarListEntry:
  access-role: string
  background-color: string
  color-id: string
  conference-properties:
    allowed-conference-solution-types: [string]
  deleted: boolean
  description: string
  etag: string
  foreground-color: string
  hidden: boolean
  id: string
  kind: string
  location: string
  primary: boolean
  selected: boolean
  summary: string
  summary-override: string
  time-zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    access-role=et`
    - The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
        - &#34;freeBusyReader&#34; - Provides read access to free/busy information. 
        - &#34;reader&#34; - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
        - &#34;writer&#34; - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
        - &#34;owner&#34; - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
* `background-color=duo`
    - The main color of the calendar in the hexadecimal format &#34;#0088aa&#34;. This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.
* `color-id=aliquyam`
    - The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional.
* `conference-properties    allowed-conference-solution-types=sea`
    - The types of conference solutions that are supported for this calendar.
        The possible values are:  
        - &#34;eventHangout&#34; 
        - &#34;eventNamedHangout&#34; 
        - &#34;hangoutsMeet&#34;  Optional.
    - Each invocation of this argument appends the given value to the array.

* `..    deleted=false`
    - Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
* `description=eos`
    - Description of the calendar. Optional. Read-only.
* `etag=erat`
    - ETag of the resource.
* `foreground-color=sadipscing`
    - The foreground color of the calendar in the hexadecimal format &#34;#ffffff&#34;. This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.
* `hidden=true`
    - Whether the calendar has been hidden from the list. Optional. The default is False.
* `id=eirmod`
    - Identifier of the calendar.
* `kind=elitr`
    - Type of the resource (&#34;calendar#calendarListEntry&#34;).
* `location=amet`
    - Geographic location of the calendar as free-form text. Optional. Read-only.
* `primary=false`
    - Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False.
* `selected=true`
    - Whether the calendar content shows up in the calendar UI. Optional. The default is False.
* `summary=eirmod`
    - Title of the calendar. Read-only.
* `summary-override=dolore`
    - The summary that the authenticated user has set for this calendar. Optional.
* `time-zone=invidunt`
    - The time zone of the calendar. Optional. Read-only.


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

* **-p color-rgb-format=boolean**
    - Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.

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
