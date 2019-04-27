Creates a new version for a site.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/firebase*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `firebasehosting1-beta1 --scope <scope> sites versions-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The parent to create the version for, in the format:
        &lt;code&gt;sites/&lt;var&gt;site-name&lt;/var&gt;&lt;/code&gt;
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Version:
  config:
    app-association: string
    clean-urls: boolean
    trailing-slash-behavior: string
  create-time: string
  create-user:
    email: string
    image-url: string
  delete-time: string
  delete-user:
    email: string
    image-url: string
  file-count: int64
  finalize-time: string
  finalize-user:
    email: string
    image-url: string
  labels: { string: string }
  name: string
  status: string
  version-bytes: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .config    app-association=amet`
    - How to handle well known App Association files.
* `clean-urls=true`
    - Defines whether to drop the file extension from uploaded files.
* `trailing-slash-behavior=consetetur`
    - Defines how to handle a trailing slash in the URL path.

* `..    create-time=ut`
    - Output only. The time at which the version was created.
* `create-user    email=ea`
    - The email address of the user when the user performed the action.
* `image-url=sed`
    - A profile image URL for the user. May not be present if the user has
        changed their email address or deleted their account.

* `..    delete-time=dolor`
    - Output only. The time at which the version was `DELETED`.
* `delete-user    email=dolor`
    - The email address of the user when the user performed the action.
* `image-url=dolor`
    - A profile image URL for the user. May not be present if the user has
        changed their email address or deleted their account.

* `..    file-count=-23`
    - Output only. The total number of files associated with the version.
        &lt;br&gt;This value is calculated after a version is `FINALIZED`.
* `finalize-time=consetetur`
    - Output only. The time at which the version was `FINALIZED`.
* `finalize-user    email=amet.`
    - The email address of the user when the user performed the action.
* `image-url=voluptua.`
    - A profile image URL for the user. May not be present if the user has
        changed their email address or deleted their account.

* `..    labels=key=lorem`
    - The labels used for extra metadata and/or filtering.
    - the value will be associated with the given `key`
* `name=gubergren`
    - The unique identifier for a version, in the format:
        &lt;code&gt;sites/&lt;var&gt;site-name&lt;/var&gt;/versions/&lt;var&gt;versionID&lt;/var&gt;&lt;/code&gt;
        This name is provided in the response body when you call the
        [`CreateVersion`](../sites.versions/create) endpoint.
* `status=justo`
    - The deploy status of a version.
        &lt;br&gt;
        &lt;br&gt;For a successful deploy, call the
        [`CreateVersion`](sites.versions/create) endpoint to make a new version
        (`CREATED` status),
        [upload all desired files](sites.versions/populateFiles) to the version,
        then [update](sites.versions/patch) the version to the `FINALIZED` status.
        &lt;br&gt;
        &lt;br&gt;Note that if you leave the version in the `CREATED` state for more
        than 12&amp;nbsp;hours, the system will automatically mark the version as
        `ABANDONED`.
        &lt;br&gt;
        &lt;br&gt;You can also change the status of a version to `DELETED` by calling the
        [`DeleteVersion`](sites.versions/delete) endpoint.
* `version-bytes=sit`
    - Output only. The total stored bytesize of the version.
        &lt;br&gt;This value is calculated after a version is `FINALIZED`.


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

* **-p version-id=string**
    - A unique id for the new version. This is only specified for legacy version
        creations.

* **-p size-bytes=string**
    - The self-reported size of the version. This value is used for a pre-emptive
        quota check for legacy version uploads.

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
