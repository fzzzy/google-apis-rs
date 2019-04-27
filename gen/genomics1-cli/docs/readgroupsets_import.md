Creates read group sets by asynchronously importing the provided
information.

The caller must have WRITE permissions to the dataset.

## Notes on [BAM](https://samtools.github.io/hts-specs/SAMv1.pdf) import

- Tags will be converted to strings - tag types are not preserved
- Comments (`@CO`) in the input file header will not be preserved
- Original header order of references (`@SQ`) will not be preserved
- Any reverse stranded unmapped reads will be reverse complemented, and
their qualities (also the &#34;BQ&#34; and &#34;OQ&#34; tags, if any) will be reversed
- Unmapped reads will be stripped of positional information (reference name
and position)
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/devstorage.read_write*
* *https://www.googleapis.com/auth/genomics*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `genomics1 --scope <scope> readgroupsets import ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ImportReadGroupSetsRequest:
  dataset-id: string
  partition-strategy: string
  reference-set-id: string
  source-uris: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    dataset-id=kasd`
    - Required. The ID of the dataset these read group sets will belong to. The
        caller must have WRITE permissions to this dataset.
* `partition-strategy=sanctus`
    - The partition strategy describes how read groups are partitioned into read
        group sets.
* `reference-set-id=takimata`
    - The reference set to which the imported read group sets are aligned to, if
        any. The reference names of this reference set must be a superset of those
        found in the imported file headers. If no reference set id is provided, a
        best effort is made to associate with a matching reference set.
* `source-uris=at`
    - A list of URIs pointing at [BAM
        files](https://samtools.github.io/hts-specs/SAMv1.pdf)
        in Google Cloud Storage.
        Those URIs can include wildcards (*), but do not add or remove
        matching files before import has completed.
        
        Note that Google Cloud Storage object listing is only eventually
        consistent: files added may be not be immediately visible to
        everyone. Thus, if using a wildcard it is preferable not to start
        the import immediately after the files are created.
    - Each invocation of this argument appends the given value to the array.


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

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
