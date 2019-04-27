Updates the InspectTemplate.
See https://cloud.google.com/dlp/docs/creating-templates to learn more.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dlp2 --scope <scope> projects inspect-templates-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Resource name of organization and inspectTemplate to be updated, for
        example `organizations/433245324/inspectTemplates/432452342` or
        projects/project-id/inspectTemplates/432452342.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GooglePrivacyDlpV2UpdateInspectTemplateRequest:
  inspect-template:
    create-time: string
    description: string
    display-name: string
    inspect-config:
      content-options: [string]
      exclude-info-types: boolean
      include-quote: boolean
      limits:
        max-findings-per-item: integer
        max-findings-per-request: integer
      min-likelihood: string
    name: string
    update-time: string
  update-mask: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .inspect-template    create-time=rebum.`
    - The creation timestamp of a inspectTemplate, output only field.
* `description=lorem`
    - Short description (max 256 chars).
* `display-name=lorem`
    - Display name (max 256 chars).
* `inspect-config    content-options=diam`
    - List of options defining data content to scan.
        If empty, text, images, and other content will be included.
    - Each invocation of this argument appends the given value to the array.
* `exclude-info-types=true`
    - When true, excludes type information of the findings.
* `include-quote=true`
    - When true, a contextual quote from the data that triggered a finding is
        included in the response; see Finding.quote.
* `limits    max-findings-per-item=50`
    - Max number of findings that will be returned for each item scanned.
        When set within `InspectDataSourceRequest`,
        the maximum returned is 1000 regardless if this is set higher.
        When set within `InspectContentRequest`, this field is ignored.
* `max-findings-per-request=1`
    - Max number of findings that will be returned per request/job.
        When set within `InspectContentRequest`, the maximum returned is 1000
        regardless if this is set higher.

* `..    min-likelihood=ut`
    - Only returns findings equal or above this threshold. The default is
        POSSIBLE.
        See https://cloud.google.com/dlp/docs/likelihood to learn more.

* `..    name=dolor`
    - The template name. Output only.
        
        The template will have one of the following formats:
        `projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR
        `organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`
* `update-time=sea`
    - The last update timestamp of a inspectTemplate, output only field.

* `..    update-mask=ut`
    - Mask to control which fields get updated.


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
