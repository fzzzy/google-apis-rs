Updates an access control rule.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/calendar* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/calendar*.
You can set the scope for this method like this: `calendar3 --scope <scope> acl update ...`
# Required Scalar Arguments
* **&lt;calendar-id&gt;** *(string)*
    - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the &#34;primary&#34; keyword.
* **&lt;rule-id&gt;** *(string)*
    - ACL rule identifier.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AclRule:
  etag: string
  id: string
  kind: string
  role: string
  scope:
    type: string
    value: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    etag=labore`
    - ETag of the resource.
* `id=sea`
    - Identifier of the ACL rule.
* `kind=nonumy`
    - Type of the resource (&#34;calendar#aclRule&#34;).
* `role=dolores`
    - The role assigned to the scope. Possible values are:  
        - &#34;none&#34; - Provides no access. 
        - &#34;freeBusyReader&#34; - Provides read access to free/busy information. 
        - &#34;reader&#34; - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
        - &#34;writer&#34; - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
        - &#34;owner&#34; - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.
* `scope    type=gubergren`
    - The type of the scope. Possible values are:  
        - &#34;default&#34; - The public scope. This is the default value. 
        - &#34;user&#34; - Limits the scope to a single user. 
        - &#34;group&#34; - Limits the scope to a group. 
        - &#34;domain&#34; - Limits the scope to a domain.  Note: The permissions granted to the &#34;default&#34;, or public, scope apply to any user, authenticated or not.
* `value=sadipscing`
    - The email address of a user or group, or the name of a domain, depending on the scope type. Omitted for type &#34;default&#34;.



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

* **-p send-notifications=boolean**
    - Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True.

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
