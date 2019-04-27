Creates a permission for a file or Team Drive.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive3 --scope <scope> permissions create ...`
# Required Scalar Argument
* **&lt;file-id&gt;** *(string)*
    - The ID of the file or Team Drive.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Permission:
  allow-file-discovery: boolean
  deleted: boolean
  display-name: string
  domain: string
  email-address: string
  expiration-time: string
  id: string
  kind: string
  photo-link: string
  role: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    allow-file-discovery=true`
    - Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone.
* `deleted=false`
    - Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions.
* `display-name=lorem`
    - A displayable name for users, groups or domains.
* `domain=eirmod`
    - The domain to which this permission refers.
* `email-address=elitr`
    - The email address of the user or group to which this permission refers.
* `expiration-time=clita`
    - The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:  
        - They can only be set on user and group permissions 
        - The time must be in the future 
        - The time cannot be more than a year in the future
* `id=et`
    - The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId.
* `kind=eirmod`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#permission&#34;.
* `photo-link=ea`
    - A link to the user&#39;s profile photo, if available.
* `role=et`
    - The role granted by this permission. While new values may be supported in the future, the following are currently allowed:  
        - owner 
        - organizer 
        - fileOrganizer 
        - writer 
        - commenter 
        - reader
* `type=sed`
    - The type of the grantee. Valid values are:  
        - user 
        - group 
        - domain 
        - anyone


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

* **-p use-domain-admin-access=boolean**
    - Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the item belongs.

* **-p transfer-ownership=boolean**
    - Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect.

* **-p supports-team-drives=boolean**
    - Whether the requesting application supports Team Drives.

* **-p email-message=string**
    - A plain text custom message to include in the notification email.

* **-p send-notification-email=boolean**
    - Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers.

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
