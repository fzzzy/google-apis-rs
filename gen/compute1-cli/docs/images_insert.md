Creates an image in the specified project using the data included in the request.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*
* *https://www.googleapis.com/auth/devstorage.full_control*
* *https://www.googleapis.com/auth/devstorage.read_only*
* *https://www.googleapis.com/auth/devstorage.read_write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> images insert ...`
# Required Scalar Argument
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Image:
  archive-size-bytes: string
  creation-timestamp: string
  deprecated:
    deleted: string
    deprecated: string
    obsolete: string
    replacement: string
    state: string
  description: string
  disk-size-gb: string
  family: string
  id: string
  image-encryption-key:
    kms-key-name: string
    raw-key: string
    sha256: string
  kind: string
  label-fingerprint: string
  labels: { string: string }
  license-codes: [string]
  licenses: [string]
  name: string
  raw-disk:
    container-type: string
    sha1-checksum: string
    source: string
  self-link: string
  source-disk: string
  source-disk-encryption-key:
    kms-key-name: string
    raw-key: string
    sha256: string
  source-disk-id: string
  source-image: string
  source-image-encryption-key:
    kms-key-name: string
    raw-key: string
    sha256: string
  source-image-id: string
  source-snapshot: string
  source-snapshot-encryption-key:
    kms-key-name: string
    raw-key: string
    sha256: string
  source-snapshot-id: string
  source-type: string
  status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    archive-size-bytes=dolor`
    - Size of the image tar.gz archive stored in Google Cloud Storage (in bytes).
* `creation-timestamp=et`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `deprecated    deleted=amet`
    - An optional RFC3339 timestamp on or after which the state of this resource is intended to change to DELETED. This is only informational and the status will not change unless the client explicitly changes it.
* `deprecated=sit`
    - An optional RFC3339 timestamp on or after which the state of this resource is intended to change to DEPRECATED. This is only informational and the status will not change unless the client explicitly changes it.
* `obsolete=vero`
    - An optional RFC3339 timestamp on or after which the state of this resource is intended to change to OBSOLETE. This is only informational and the status will not change unless the client explicitly changes it.
* `replacement=nonumy`
    - The URL of the suggested replacement for a deprecated resource. The suggested replacement resource must be the same kind of resource as the deprecated resource.
* `state=accusam`
    - The deprecation state of this resource. This can be DEPRECATED, OBSOLETE, or DELETED. Operations which create a new resource using a DEPRECATED resource will return successfully, but with a warning indicating the deprecated resource and recommending its replacement. Operations which use OBSOLETE or DELETED resources will be rejected and result in an error.

* `..    description=est`
    - An optional description of this resource. Provide this property when you create the resource.
* `disk-size-gb=sit`
    - Size of the image when restored onto a persistent disk (in GB).
* `family=erat`
    - The name of the image family to which this image belongs. You can create disks by specifying an image family instead of a specific image name. The image family always returns its latest image that is not deprecated. The name of the image family must comply with RFC1035.
* `id=vero`
    - [Output Only] The unique identifier for the resource. This identifier is defined by the server.
* `image-encryption-key    kms-key-name=accusam`
    - The name of the encryption key that is stored in Google Cloud KMS.
* `raw-key=et`
    - Specifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource.
* `sha256=clita`
    - [Output only] The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource.

* `..    kind=tempor`
    - [Output Only] Type of the resource. Always compute#image for images.
* `label-fingerprint=ut`
    - A fingerprint for the labels being applied to this image, which is essentially a hash of the labels used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash in order to update or change labels.
        
        To see the latest fingerprint, make a get() request to retrieve an image.
* `labels=key=kasd`
    - Labels to apply to this image. These can be later modified by the setLabels method.
    - the value will be associated with the given `key`
* `license-codes=diam`
    - Integer license codes indicating which licenses are attached to this image.
    - Each invocation of this argument appends the given value to the array.
* `licenses=ut`
    - Any applicable license URI.
    - Each invocation of this argument appends the given value to the array.
* `name=diam`
    - Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
* `raw-disk    container-type=eos`
    - The format used to encode and transmit the block device, which should be TAR. This is just a container and transmission format and not a runtime format. Provided by the client when the disk image is created.
* `sha1-checksum=elitr`
    - An optional SHA1 checksum of the disk image before unpackaging; provided by the client when the disk image is created.
* `source=tempor`
    - The full Google Cloud Storage URL where the disk image is stored. You must provide either this property or the sourceDisk property but not both.

* `..    self-link=ipsum`
    - [Output Only] Server-defined URL for the resource.
* `source-disk=no`
    - URL of the source disk used to create this image. This can be a full or valid partial URL. You must provide either this property or the rawDisk.source property but not both to create an image. For example, the following are valid values:  
        - https://www.googleapis.com/compute/v1/projects/project/zones/zone/disks/disk 
        - projects/project/zones/zone/disks/disk 
        - zones/zone/disks/disk
* `source-disk-encryption-key    kms-key-name=takimata`
    - The name of the encryption key that is stored in Google Cloud KMS.
* `raw-key=takimata`
    - Specifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource.
* `sha256=gubergren`
    - [Output only] The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource.

* `..    source-disk-id=clita`
    - [Output Only] The ID value of the disk used to create this image. This value may be used to determine whether the image was taken from the current or a previous instance of a given disk name.
* `source-image=et`
    - URL of the source image used to create this image. This can be a full or valid partial URL. You must provide exactly one of:  
        - this property, or  
        - the rawDisk.source property, or  
        - the sourceDisk property   in order to create an image.
* `source-image-encryption-key    kms-key-name=justo`
    - The name of the encryption key that is stored in Google Cloud KMS.
* `raw-key=clita`
    - Specifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource.
* `sha256=eos`
    - [Output only] The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource.

* `..    source-image-id=sea`
    - [Output Only] The ID value of the image used to create this image. This value may be used to determine whether the image was taken from the current or a previous instance of a given image name.
* `source-snapshot=clita`
    - URL of the source snapshot used to create this image. This can be a full or valid partial URL. You must provide exactly one of:  
        - this property, or  
        - the sourceImage property, or  
        - the rawDisk.source property, or  
        - the sourceDisk property   in order to create an image.
* `source-snapshot-encryption-key    kms-key-name=tempor`
    - The name of the encryption key that is stored in Google Cloud KMS.
* `raw-key=stet`
    - Specifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource.
* `sha256=dolor`
    - [Output only] The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource.

* `..    source-snapshot-id=stet`
    - [Output Only] The ID value of the snapshot used to create this image. This value may be used to determine whether the snapshot was taken from the current or a previous instance of a given snapshot name.
* `source-type=magna`
    - The type of the image used to create this disk. The default and only value is RAW
* `status=lorem`
    - [Output Only] The status of the image. An image can be used to create other resources, such as instances, only after the image has been successfully created and the status is set to READY. Possible values are FAILED, PENDING, or READY.


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

* **-p request-id=string**
    - An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed.
        
        For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments.
        
        The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).

* **-p force-create=boolean**
    - Force image creation if true.

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
