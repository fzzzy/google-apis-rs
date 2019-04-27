Patches an object&#39;s metadata.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/devstorage.full_control*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `storage1 --scope <scope> objects patch ...`
# Required Scalar Arguments
* **&lt;bucket&gt;** *(string)*
    - Name of the bucket in which the object resides.
* **&lt;object&gt;** *(string)*
    - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Object:
  bucket: string
  cache-control: string
  component-count: integer
  content-disposition: string
  content-encoding: string
  content-language: string
  content-type: string
  crc32c: string
  customer-encryption:
    encryption-algorithm: string
    key-sha256: string
  etag: string
  event-based-hold: boolean
  generation: string
  id: string
  kind: string
  kms-key-name: string
  md5-hash: string
  media-link: string
  metadata: { string: string }
  metageneration: string
  name: string
  owner:
    entity: string
    entity-id: string
  retention-expiration-time: string
  self-link: string
  size: string
  storage-class: string
  temporary-hold: boolean
  time-created: string
  time-deleted: string
  time-storage-class-updated: string
  updated: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    bucket=erat`
    - The name of the bucket containing this object.
* `cache-control=gubergren`
    - Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600.
* `component-count=71`
    - Number of underlying components that make up this object. Components are accumulated by compose operations.
* `content-disposition=et`
    - Content-Disposition of the object data.
* `content-encoding=amet`
    - Content-Encoding of the object data.
* `content-language=lorem`
    - Content-Language of the object data.
* `content-type=voluptua.`
    - Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream.
* `crc32c=rebum.`
    - CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best Practices.
* `customer-encryption    encryption-algorithm=justo`
    - The encryption algorithm.
* `key-sha256=labore`
    - SHA256 hash value of the encryption key.

* `..    etag=voluptua.`
    - HTTP 1.1 Entity tag for the object.
* `event-based-hold=false`
    - Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold&#39;s release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false.
* `generation=dolor`
    - The content generation of this object. Used for object versioning.
* `id=takimata`
    - The ID of the object, including the bucket name, object name, and generation number.
* `kind=voluptua.`
    - The kind of item this is. For objects, this is always storage#object.
* `kms-key-name=no`
    - Cloud KMS Key used to encrypt this object, if the object is encrypted by such a key.
* `md5-hash=aliquyam`
    - MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see Hashes and ETags: Best Practices.
* `media-link=magna`
    - Media download link.
* `metadata=key=et`
    - User-provided metadata, in key/value pairs.
    - the value will be associated with the given `key`
* `metageneration=sed`
    - The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object.
* `name=est`
    - The name of the object. Required if not specified by URL parameter.
* `owner    entity=sit`
    - The entity, in the form user-userId.
* `entity-id=et`
    - The ID for the entity.

* `..    retention-expiration-time=consetetur`
    - A server-determined value that specifies the earliest time that the object&#39;s retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold).
* `self-link=sea`
    - The link to this object.
* `size=nonumy`
    - Content-Length of the data in bytes.
* `storage-class=consetetur`
    - Storage class of the object.
* `temporary-hold=false`
    - Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object.
* `time-created=clita`
    - The creation time of the object in RFC 3339 format.
* `time-deleted=sea`
    - The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted.
* `time-storage-class-updated=vero`
    - The time at which the object&#39;s storage class was last changed. When the object is initially created, it will be set to timeCreated.
* `updated=dolores`
    - The modification time of the object metadata in RFC 3339 format.


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

* **-p predefined-acl=string**
    - Apply a predefined set of access controls to this object.

* **-p if-generation-not-match=string**
    - Makes the operation conditional on whether the object&#39;s current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.

* **-p generation=string**
    - If present, selects a specific revision of this object (as opposed to the latest version, the default).

* **-p if-metageneration-not-match=string**
    - Makes the operation conditional on whether the object&#39;s current metageneration does not match the given value.

* **-p if-generation-match=string**
    - Makes the operation conditional on whether the object&#39;s current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.

* **-p projection=string**
    - Set of properties to return. Defaults to full.

* **-p user-project=string**
    - The project to be billed for this request, for Requester Pays buckets.

* **-p if-metageneration-match=string**
    - Makes the operation conditional on whether the object&#39;s current metageneration matches the given value.

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
