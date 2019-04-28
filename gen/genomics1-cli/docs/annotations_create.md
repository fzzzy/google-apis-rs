Creates a new annotation. Caller must have WRITE permission
for the associated annotation set.

The following fields are required:

* annotationSetId
* referenceName or
  referenceId

### Transcripts

For annotations of type TRANSCRIPT, the following fields of
transcript must be provided:

* exons.start
* exons.end

All other fields may be optionally specified, unless documented as being
server-generated (for example, the `id` field). The annotated
range must be no longer than 100Mbp (mega base pairs). See the
Annotation resource
for additional restrictions on each field.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/genomics*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `genomics1 --scope <scope> annotations create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Annotation:
  annotation-set-id: string
  end: string
  id: string
  name: string
  reference-id: string
  reference-name: string
  reverse-strand: boolean
  start: string
  transcript:
    coding-sequence:
      end: string
      start: string
    gene-id: string
  type: string
  variant:
    alternate-bases: string
    clinical-significance: string
    effect: string
    gene-id: string
    transcript-ids: [string]
    type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    annotation-set-id=sit`
    - The annotation set to which this annotation belongs.
* `end=stet`
    - The end position of the range on the reference, 0-based exclusive.
* `id=sed`
    - The server-generated annotation ID, unique across all annotations.
* `name=et`
    - The display name of this annotation.
* `reference-id=dolores`
    - The ID of the Google Genomics reference associated with this range.
* `reference-name=kasd`
    - The display name corresponding to the reference specified by
        `referenceId`, for example `chr1`, `1`, or `chrX`.
* `reverse-strand=true`
    - Whether this range refers to the reverse strand, as opposed to the forward
        strand. Note that regardless of this field, the start/end position of the
        range always refer to the forward strand.
* `start=takimata`
    - The start position of the range on the reference, 0-based inclusive.
* `transcript.coding-sequence    end=justo`
    - The end of the coding sequence on this annotation&#39;s reference sequence,
        0-based exclusive. Note that this position is relative to the reference
        start, and *not* the containing annotation start.
* `start=amet.`
    - The start of the coding sequence on this annotation&#39;s reference sequence,
        0-based inclusive. Note that this position is relative to the reference
        start, and *not* the containing annotation start.

* `..    gene-id=erat`
    - The annotation ID of the gene from which this transcript is transcribed.

* `..    type=labore`
    - The data type for this annotation. Must match the containing annotation
        set&#39;s type.
* `variant    alternate-bases=sea`
    - The alternate allele for this variant. If multiple alternate alleles
        exist at this location, create a separate variant for each one, as they
        may represent distinct conditions.
* `clinical-significance=nonumy`
    - Describes the clinical significance of a variant.
        It is adapted from the ClinVar controlled vocabulary for clinical
        significance described at:
        http://www.ncbi.nlm.nih.gov/clinvar/docs/clinsig/
* `effect=dolores`
    - Effect of the variant on the coding sequence.
* `gene-id=gubergren`
    - Google annotation ID of the gene affected by this variant. This should
        be provided when the variant is created.
* `transcript-ids=sadipscing`
    - Google annotation IDs of the transcripts affected by this variant. These
        should be provided when the variant is created.
    - Each invocation of this argument appends the given value to the array.
* `type=aliquyam`
    - Type has been adapted from ClinVar&#39;s list of variant types.



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