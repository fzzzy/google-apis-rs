Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/bigquery*
* *https://www.googleapis.com/auth/cloud-platform*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/bigquery*.
You can set the scope for this method like this: `bigquery2 --scope <scope> datasets update ...`
# Required Scalar Arguments
* **&lt;project-id&gt;** *(string)*
    - Project ID of the dataset being updated
* **&lt;dataset-id&gt;** *(string)*
    - Dataset ID of the dataset being updated
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Dataset:
  creation-time: string
  dataset-reference:
    dataset-id: string
    project-id: string
  default-partition-expiration-ms: string
  default-table-expiration-ms: string
  description: string
  etag: string
  friendly-name: string
  id: string
  kind: string
  labels: { string: string }
  last-modified-time: string
  location: string
  self-link: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    creation-time=et`
    - [Output-only] The time when this dataset was created, in milliseconds since the epoch.
* `dataset-reference    dataset-id=duo`
    - [Required] A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
* `project-id=aliquyam`
    - [Optional] The ID of the project containing this dataset.

* `..    default-partition-expiration-ms=sea`
    - [Optional] The default partition expiration for all partitioned tables in the dataset, in milliseconds. Once this property is set, all newly-created partitioned tables in the dataset will have an expirationMs property in the timePartitioning settings set to this value, and changing the value will only affect new tables, not existing ones. The storage in a partition will have an expiration time of its partition time plus this value. Setting this property overrides the use of defaultTableExpirationMs for partitioned tables: only one of defaultTableExpirationMs and defaultPartitionExpirationMs will be used for any new partitioned table. If you provide an explicit timePartitioning.expirationMs when creating or updating a partitioned table, that value takes precedence over the default partition expiration time indicated by this property.
* `default-table-expiration-ms=lorem`
    - [Optional] The default lifetime of all tables in the dataset, in milliseconds. The minimum value is 3600000 milliseconds (one hour). Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table&#39;s expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property.
* `description=eos`
    - [Optional] A user-friendly description of the dataset.
* `etag=erat`
    - [Output-only] A hash of the resource.
* `friendly-name=sadipscing`
    - [Optional] A descriptive name for the dataset.
* `id=dolor`
    - [Output-only] The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field.
* `kind=eirmod`
    - [Output-only] The resource type.
* `labels=key=elitr`
    - The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See Creating and Updating Dataset Labels for more information.
    - the value will be associated with the given `key`
* `last-modified-time=amet`
    - [Output-only] The date when this dataset or any of its tables was last modified, in milliseconds since the epoch.
* `location=no`
    - The geographic location where the dataset should reside. The default value is US. See details at https://cloud.google.com/bigquery/docs/dataset-locations.
* `self-link=labore`
    - [Output-only] A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource.


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
