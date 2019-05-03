Updates a permission.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive2 --scope <scope> permissions update ...`
# Required Scalar Arguments
* **&lt;file-id&gt;** *(string)*
    - The ID for the file or Team Drive.
* **&lt;permission-id&gt;** *(string)*
    - The ID for the permission.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Permission:
  additional-roles: [string]
  auth-key: string
  deleted: boolean
  domain: string
  email-address: string
  etag: string
  expiration-date: string
  id: string
  kind: string
  name: string
  photo-link: string
  role: string
  self-link: string
  type: string
  value: string
  with-link: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    additional-roles=diam`
    - Additional roles for this user. Only commenter is currently allowed, though more may be supported in the future.
    - Each invocation of this argument appends the given value to the array.
* `auth-key=sea`
    - Deprecated.
* `deleted=true`
    - Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions.
* `domain=sed`
    - The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is user, group or domain.
* `email-address=amet.`
    - The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is user or group.
* `etag=duo`
    - The ETag of the permission.
* `expiration-date=sit`
    - The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions:  
        - They can only be set on user and group permissions 
        - The date must be in the future 
        - The date cannot be more than a year in the future 
        - The date can only be set on drive.permissions.update or drive.permissions.patch requests
* `id=at`
    - The ID of the user this permission refers to, and identical to the permissionId in the About and Files resources. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored.
* `kind=voluptua.`
    - This is always drive#permission.
* `name=sed`
    - The name for this permission.
* `photo-link=sea`
    - A link to the profile photo, if available.
* `role=dolore`
    - The primary role for this user. While new values may be supported in the future, the following are currently allowed:  
        - owner 
        - organizer 
        - fileOrganizer 
        - writer 
        - reader
* `self-link=kasd`
    - A link back to this permission.
* `type=at`
    - The account type. Allowed values are:  
        - user 
        - group 
        - domain 
        - anyone
* `value=diam`
    - The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored.
* `with-link=true`
    - Whether the link is required for this permission.


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
    - Whether changing a role to &#39;owner&#39; downgrades the current owners to writers. Does nothing if the specified role is not &#39;owner&#39;.

* **-p remove-expiration=boolean**
    - Whether to remove the expiration date.

* **-p supports-team-drives=boolean**
    - Whether the requesting application supports Team Drives.

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
