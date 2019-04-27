Attaches an existing Disk resource to an instance. You must first create the disk before you can attach it. It is not possible to create and attach a disk at the same time. For more information, read Adding a persistent disk to your instance.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> instances attach-disk ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
* **&lt;zone&gt;** *(string)*
    - The name of the zone for this request.
* **&lt;instance&gt;** *(string)*
    - The instance name for this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AttachedDisk:
  auto-delete: boolean
  boot: boolean
  device-name: string
  disk-encryption-key:
    kms-key-name: string
    raw-key: string
    sha256: string
  index: integer
  initialize-params:
    description: string
    disk-name: string
    disk-size-gb: string
    disk-type: string
    labels: { string: string }
    source-image: string
    source-image-encryption-key:
      kms-key-name: string
      raw-key: string
      sha256: string
  interface: string
  kind: string
  licenses: [string]
  mode: string
  source: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    auto-delete=true`
    - Specifies whether the disk will be auto-deleted when the instance is deleted (but not when the disk is detached from the instance).
* `boot=false`
    - Indicates that this is a boot disk. The virtual machine will use the first partition of the disk for its root filesystem.
* `device-name=amet.`
    - Specifies a unique device name of your choice that is reflected into the /dev/disk/by-id/google-* tree of a Linux operating system running within the instance. This name can be used to reference the device for mounting, resizing, and so on, from within the instance.
        
        If not specified, the server chooses a default device name to apply to this disk, in the form persistent-disks-x, where x is a number assigned by Google Compute Engine. This field is only applicable for persistent disks.
* `disk-encryption-key    kms-key-name=clita`
    - The name of the encryption key that is stored in Google Cloud KMS.
* `raw-key=stet`
    - Specifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource.
* `sha256=sed`
    - [Output only] The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource.

* `..    index=11`
    - [Output Only] A zero-based index to this disk, where 0 is reserved for the boot disk. If you have many disks attached to an instance, each disk would have a unique index number.
* `initialize-params    description=elitr`
    - An optional description. Provide this property when creating the disk.
* `disk-name=gubergren`
    - Specifies the disk name. If not specified, the default is to use the name of the instance. If the disk with the instance name exists already in the given zone/region, a new name will be automatically generated.
* `disk-size-gb=sit`
    - Specifies the size of the disk in base-2 GB.
* `disk-type=et`
    - Specifies the disk type to use to create the instance. If not specified, the default is pd-standard, specified using the full URL. For example:
        https://www.googleapis.com/compute/v1/projects/project/zones/zone/diskTypes/pd-standard
        
        
        Other values include pd-ssd and local-ssd. If you define this field, you can provide either the full or partial URL. For example, the following are valid values:  
        - https://www.googleapis.com/compute/v1/projects/project/zones/zone/diskTypes/diskType 
        - projects/project/zones/zone/diskTypes/diskType 
        - zones/zone/diskTypes/diskType  Note that for InstanceTemplate, this is the name of the disk type, not URL.
* `labels=key=sea`
    - Labels to apply to this disk. These can be later modified by the disks.setLabels method. This field is only applicable for persistent disks.
    - the value will be associated with the given `key`
* `source-image=justo`
    - The source image to create this disk. When creating a new instance, one of initializeParams.sourceImage or disks.source is required except for local SSD.
        
        To create a disk with one of the public operating system images, specify the image by its family name. For example, specify family/debian-9 to use the latest Debian 9 image:
        projects/debian-cloud/global/images/family/debian-9
        
        
        Alternatively, use a specific version of a public operating system image:
        projects/debian-cloud/global/images/debian-9-stretch-vYYYYMMDD
        
        
        To create a disk with a custom image that you created, specify the image name in the following format:
        global/images/my-custom-image
        
        
        You can also specify a custom image by its image family, which returns the latest version of the image in that family. Replace the image name with family/family-name:
        global/images/family/my-image-family
        
        
        If the source image is deleted later, this field will not be set.
* `source-image-encryption-key    kms-key-name=dolore`
    - The name of the encryption key that is stored in Google Cloud KMS.
* `raw-key=at`
    - Specifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource.
* `sha256=dolor`
    - [Output only] The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource.


* `...    interface=tempor`
    - Specifies the disk interface to use for attaching this disk, which is either SCSI or NVME. The default is SCSI. Persistent disks must always use SCSI and the request will fail if you attempt to attach a persistent disk in any other format than SCSI. Local SSDs can use either NVME or SCSI. For performance characteristics of SCSI over NVMe, see Local SSD performance.
* `kind=ipsum`
    - [Output Only] Type of the resource. Always compute#attachedDisk for attached disks.
* `licenses=est`
    - [Output Only] Any valid publicly visible licenses.
    - Each invocation of this argument appends the given value to the array.
* `mode=et`
    - The mode in which to attach this disk, either READ_WRITE or READ_ONLY. If not specified, the default is to attach the disk in READ_WRITE mode.
* `source=no`
    - Specifies a valid partial or full URL to an existing Persistent Disk resource. When creating a new instance, one of initializeParams.sourceImage or disks.source is required except for local SSD.
        
        If desired, you can also attach existing non-root persistent disks using this property. This field is only applicable for persistent disks.
        
        Note that for InstanceTemplate, specify the disk name, not the URL for the disk.
* `type=sed`
    - Specifies the type of the disk, either SCRATCH or PERSISTENT. If not specified, the default is PERSISTENT.


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

* **-p force-attach=boolean**
    - Whether to force attach the disk even if it&#39;s currently attached to another instance. This is only available for regional disks.

* **-p request-id=string**
    - An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed.
        
        For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments.
        
        The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).

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
