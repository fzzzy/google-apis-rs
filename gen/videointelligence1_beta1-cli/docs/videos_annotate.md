Performs asynchronous video annotation. Progress and results can be
retrieved through the `google.longrunning.Operations` interface.
`Operation.metadata` contains `AnnotateVideoProgress` (progress).
`Operation.response` contains `AnnotateVideoResponse` (results).
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `videointelligence1-beta1 --scope <scope> videos annotate ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudVideointelligenceV1beta1_AnnotateVideoRequest:
  features: [string]
  input-content: string
  input-uri: string
  location-id: string
  output-uri: string
  video-context:
    label-detection-mode: string
    label-detection-model: string
    safe-search-detection-model: string
    shot-change-detection-model: string
    stationary-camera: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    features=eirmod`
    - Requested video annotation features.
    - Each invocation of this argument appends the given value to the array.
* `input-content=sit`
    - The video data bytes. Encoding: base64. If unset, the input video(s)
        should be specified via `input_uri`. If set, `input_uri` should be unset.
* `input-uri=stet`
    - Input video location. Currently, only
        [Google Cloud Storage](https://cloud.google.com/storage/) URIs are
        supported, which must be specified in the following format:
        `gs://bucket-id/object-id` (other URI formats return
        google.rpc.Code.INVALID_ARGUMENT). For more information, see
        [Request URIs](/storage/docs/reference-uris).
        A video URI may include wildcards in `object-id`, and thus identify
        multiple videos. Supported wildcards: &#39;*&#39; to match 0 or more characters;
        &#39;?&#39; to match 1 character. If unset, the input video should be embedded
        in the request as `input_content`. If set, `input_content` should be unset.
* `location-id=sed`
    - Optional cloud region where annotation should take place. Supported cloud
        regions: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region
        is specified, a region will be determined based on video file location.
* `output-uri=et`
    - Optional location where the output (in JSON format) should be stored.
        Currently, only [Google Cloud Storage](https://cloud.google.com/storage/)
        URIs are supported, which must be specified in the following format:
        `gs://bucket-id/object-id` (other URI formats return
        google.rpc.Code.INVALID_ARGUMENT). For more information, see
        [Request URIs](/storage/docs/reference-uris).
* `video-context    label-detection-mode=dolores`
    - If label detection has been requested, what labels should be detected
        in addition to video-level labels or segment-level labels. If unspecified,
        defaults to `SHOT_MODE`.
* `label-detection-model=kasd`
    - Model to use for label detection.
        Supported values: &#34;latest&#34; and &#34;stable&#34; (the default).
* `safe-search-detection-model=accusam`
    - Model to use for safe search detection.
        Supported values: &#34;latest&#34; and &#34;stable&#34; (the default).
* `shot-change-detection-model=takimata`
    - Model to use for shot change detection.
        Supported values: &#34;latest&#34; and &#34;stable&#34; (the default).
* `stationary-camera=false`
    - Whether the video has been shot from a stationary (i.e. non-moving) camera.
        When set to true, might improve detection accuracy for moving objects.



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

* **-p bearer-token=string**
    - OAuth bearer token.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pp=boolean**
    - Pretty-print response.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
