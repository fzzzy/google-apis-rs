Creates a new document.

Operation &lt;response: Document,
           metadata: KnowledgeOperationMetadata&gt;
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dialogflow2-beta1 --scope <scope> projects knowledge-bases-documents-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The knoweldge base to create a document for.
        Format: `projects/&lt;Project ID&gt;/knowledgeBases/&lt;Knowledge Base ID&gt;`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudDialogflowV2beta1Document:
  content: string
  content-uri: string
  display-name: string
  knowledge-types: [string]
  mime-type: string
  name: string
  raw-content: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    content=elitr`
    - The raw content of the document. This field is only permitted for
        EXTRACTIVE_QA and FAQ knowledge types.
        Note: This field is in the process of being deprecated, please use
        raw_content instead.
* `content-uri=consetetur`
    - The URI where the file content is located.
        
        For documents stored in Google Cloud Storage, these URIs must have
        the form `gs://&lt;bucket-name&gt;/&lt;object-name&gt;`.
        
        NOTE: External URLs must correspond to public webpages, i.e., they must
        be indexed by Google Search. In particular, URLs for showing documents in
        Google Cloud Storage (i.e. the URL in your browser) are not supported.
        Instead use the `gs://` format URI described above.
* `display-name=sea`
    - Required. The display name of the document. The name must be 1024 bytes or
        less; otherwise, the creation request fails.
* `knowledge-types=elitr`
    - Required. The knowledge type of document content.
    - Each invocation of this argument appends the given value to the array.
* `mime-type=at`
    - Required. The MIME type of this document.
* `name=sea`
    - The document resource name.
        The name must be empty when creating a document.
        Format: `projects/&lt;Project ID&gt;/knowledgeBases/&lt;Knowledge Base
        ID&gt;/documents/&lt;Document ID&gt;`.
* `raw-content=consetetur`
    - The raw content of the document. This field is only permitted for
        EXTRACTIVE_QA and FAQ knowledge types.


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