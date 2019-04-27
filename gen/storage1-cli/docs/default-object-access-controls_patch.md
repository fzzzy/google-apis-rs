Patches a default object ACL entry on the specified bucket.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/devstorage.full_control*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `storage1 --scope <scope> default-object-access-controls patch ...`
# Required Scalar Arguments
* **&lt;bucket&gt;** *(string)*
    - Name of a bucket.
* **&lt;entity&gt;** *(string)*
    - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ObjectAccessControl:
  bucket: string
  domain: string
  email: string
  entity: string
  entity-id: string
  etag: string
  generation: string
  id: string
  kind: string
  object: string
  project-team:
    project-number: string
    team: string
  role: string
  self-link: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    bucket=ea`
    - The name of the bucket.
* `domain=ea`
    - The domain associated with the entity, if any.
* `email=et`
    - The email address associated with the entity, if any.
* `entity=dolor`
    - The entity holding the permission, in one of the following forms: 
        - user-userId 
        - user-email 
        - group-groupId 
        - group-email 
        - domain-domain 
        - project-team-projectId 
        - allUsers 
        - allAuthenticatedUsers Examples: 
        - The user liz@example.com would be user-liz@example.com. 
        - The group example@googlegroups.com would be group-example@googlegroups.com. 
        - To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
* `entity-id=diam`
    - The ID for the entity, if any.
* `etag=kasd`
    - HTTP 1.1 Entity tag for the access-control entry.
* `generation=invidunt`
    - The content generation of the object, if applied to an object.
* `id=rebum.`
    - The ID of the access-control entry.
* `kind=lorem`
    - The kind of item this is. For object access control entries, this is always storage#objectAccessControl.
* `object=clita`
    - The name of the object, if applied to an object.
* `project-team    project-number=invidunt`
    - The project number.
* `team=eirmod`
    - The team.

* `..    role=at`
    - The access permission for the entity.
* `self-link=consetetur`
    - The link to this access-control entry.


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

* **-p user-project=string**
    - The project to be billed for this request. Required for Requester Pays buckets.

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
