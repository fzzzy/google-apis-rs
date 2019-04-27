Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate. This method supports patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/devstorage.full_control*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `storage1 --scope <scope> buckets patch ...`
# Required Scalar Argument
* **&lt;bucket&gt;** *(string)*
    - Name of a bucket.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Bucket:
  billing:
    requester-pays: boolean
  default-event-based-hold: boolean
  encryption:
    default-kms-key-name: string
  etag: string
  id: string
  kind: string
  labels: { string: string }
  location: string
  logging:
    log-bucket: string
    log-object-prefix: string
  metageneration: string
  name: string
  owner:
    entity: string
    entity-id: string
  project-number: string
  retention-policy:
    effective-time: string
    is-locked: boolean
    retention-period: string
  self-link: string
  storage-class: string
  time-created: string
  updated: string
  versioning:
    enabled: boolean
  website:
    main-page-suffix: string
    not-found-page: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .billing    requester-pays=true`
    - When set to true, Requester Pays is enabled for this bucket.

* `..    default-event-based-hold=true`
    - The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold&#39;s release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed.
* `encryption    default-kms-key-name=dolor`
    - A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified.

* `..    etag=et`
    - HTTP 1.1 Entity tag for the bucket.
* `id=consetetur`
    - The ID of the bucket. For buckets, the id and name properties are the same.
* `kind=amet.`
    - The kind of item this is. For buckets, this is always storage#bucket.
* `labels=key=voluptua.`
    - User-provided labels, in key/value pairs.
    - the value will be associated with the given `key`
* `location=lorem`
    - The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the developer&#39;s guide for the authoritative list.
* `logging    log-bucket=gubergren`
    - The destination bucket where the current bucket&#39;s logs should be placed.
* `log-object-prefix=justo`
    - A prefix for log object names.

* `..    metageneration=sit`
    - The metadata generation of this bucket.
* `name=vero`
    - The name of the bucket.
* `owner    entity=diam`
    - The entity, in the form project-owner-projectId.
* `entity-id=rebum.`
    - The ID for the entity.

* `..    project-number=consetetur`
    - The project number of the project the bucket belongs to.
* `retention-policy    effective-time=sadipscing`
    - Server-determined value that indicates the time from which policy was enforced and effective. This value is in RFC 3339 format.
* `is-locked=false`
    - Once locked, an object retention policy cannot be modified.
* `retention-period=sadipscing`
    - The duration in seconds that objects need to be retained. Retention duration must be greater than zero and less than 100 years. Note that enforcement of retention periods less than a day is not guaranteed. Such periods should only be used for testing purposes.

* `..    self-link=invidunt`
    - The URI of this bucket.
* `storage-class=consetetur`
    - The bucket&#39;s default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see storage classes.
* `time-created=dolore`
    - The creation time of the bucket in RFC 3339 format.
* `updated=duo`
    - The modification time of the bucket in RFC 3339 format.
* `versioning    enabled=false`
    - While set to true, versioning is fully enabled for this bucket.

* `..website    main-page-suffix=lorem`
    - If the requested object path is missing, the service will ensure the path has a trailing &#39;/&#39;, append this suffix, and attempt to retrieve the resulting object. This allows the creation of index.html objects to represent directory pages.
* `not-found-page=et`
    - If the requested object path is missing, and any mainPageSuffix object is missing, if applicable, the service will return the named object from this bucket as the content for a 404 Not Found result.



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
    - Apply a predefined set of access controls to this bucket.

* **-p if-metageneration-not-match=string**
    - Makes the return of the bucket metadata conditional on whether the bucket&#39;s current metageneration does not match the given value.

* **-p projection=string**
    - Set of properties to return. Defaults to full.

* **-p if-metageneration-match=string**
    - Makes the return of the bucket metadata conditional on whether the bucket&#39;s current metageneration matches the given value.

* **-p user-project=string**
    - The project to be billed for this request. Required for Requester Pays buckets.

* **-p predefined-default-object-acl=string**
    - Apply a predefined set of default object access controls to this bucket.

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
