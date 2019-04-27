Updates a file&#39;s metadata and/or content with patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.appdata*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/drive.metadata*
* *https://www.googleapis.com/auth/drive.scripts*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive3 --scope <scope> files update ...`
# Required Scalar Argument
* **&lt;file-id&gt;** *(string)*
    - The ID of the file.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
File:
  app-properties: { string: string }
  capabilities:
    can-add-children: boolean
    can-change-copy-requires-writer-permission: boolean
    can-change-viewers-can-copy-content: boolean
    can-comment: boolean
    can-copy: boolean
    can-delete: boolean
    can-delete-children: boolean
    can-download: boolean
    can-edit: boolean
    can-list-children: boolean
    can-move-children-out-of-team-drive: boolean
    can-move-children-within-team-drive: boolean
    can-move-item-into-team-drive: boolean
    can-move-item-out-of-team-drive: boolean
    can-move-item-within-team-drive: boolean
    can-move-team-drive-item: boolean
    can-read-revisions: boolean
    can-read-team-drive: boolean
    can-remove-children: boolean
    can-rename: boolean
    can-share: boolean
    can-trash: boolean
    can-trash-children: boolean
    can-untrash: boolean
  content-hints:
    indexable-text: string
    thumbnail:
      image: string
      mime-type: string
  copy-requires-writer-permission: boolean
  created-time: string
  description: string
  explicitly-trashed: boolean
  file-extension: string
  folder-color-rgb: string
  full-file-extension: string
  has-augmented-permissions: boolean
  has-thumbnail: boolean
  head-revision-id: string
  icon-link: string
  id: string
  image-media-metadata:
    aperture: number
    camera-make: string
    camera-model: string
    color-space: string
    exposure-bias: number
    exposure-mode: string
    exposure-time: number
    flash-used: boolean
    focal-length: number
    height: integer
    iso-speed: integer
    lens: string
    location:
      altitude: number
      latitude: number
      longitude: number
    max-aperture-value: number
    metering-mode: string
    rotation: integer
    sensor: string
    subject-distance: integer
    time: string
    white-balance: string
    width: integer
  is-app-authorized: boolean
  kind: string
  last-modifying-user:
    display-name: string
    email-address: string
    kind: string
    me: boolean
    permission-id: string
    photo-link: string
  md5-checksum: string
  mime-type: string
  modified-by-me: boolean
  modified-by-me-time: string
  modified-time: string
  name: string
  original-filename: string
  owned-by-me: boolean
  parents: [string]
  permission-ids: [string]
  properties: { string: string }
  quota-bytes-used: string
  shared: boolean
  shared-with-me-time: string
  sharing-user:
    display-name: string
    email-address: string
    kind: string
    me: boolean
    permission-id: string
    photo-link: string
  size: string
  spaces: [string]
  starred: boolean
  team-drive-id: string
  thumbnail-link: string
  thumbnail-version: string
  trashed: boolean
  trashed-time: string
  trashing-user:
    display-name: string
    email-address: string
    kind: string
    me: boolean
    permission-id: string
    photo-link: string
  version: string
  video-media-metadata:
    duration-millis: string
    height: integer
    width: integer
  viewed-by-me: boolean
  viewed-by-me-time: string
  viewers-can-copy-content: boolean
  web-content-link: string
  web-view-link: string
  writers-can-share: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    app-properties=key=amet.`
    - A collection of arbitrary key-value pairs which are private to the requesting app.
        Entries with null values are cleared in update and copy requests.
    - the value will be associated with the given `key`
* `capabilities    can-add-children=true`
    - Whether the current user can add children to this folder. This is always false when the item is not a folder.
* `can-change-copy-requires-writer-permission=true`
    - Whether the current user can change the copyRequiresWriterPermission restriction of this file.
* `can-change-viewers-can-copy-content=true`
    - Deprecated
* `can-comment=true`
    - Whether the current user can comment on this file.
* `can-copy=false`
    - Whether the current user can copy this file. For a Team Drive item, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder.
* `can-delete=true`
    - Whether the current user can delete this file.
* `can-delete-children=true`
    - Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-download=true`
    - Whether the current user can download this file.
* `can-edit=true`
    - Whether the current user can edit this file.
* `can-list-children=false`
    - Whether the current user can list the children of this folder. This is always false when the item is not a folder.
* `can-move-children-out-of-team-drive=false`
    - Whether the current user can move children of this folder outside of the Team Drive. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-move-children-within-team-drive=false`
    - Whether the current user can move children of this folder within the Team Drive. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-move-item-into-team-drive=true`
    - Whether the current user can move this item into a Team Drive. If the item is in a Team Drive, this field is equivalent to canMoveTeamDriveItem.
* `can-move-item-out-of-team-drive=false`
    - Whether the current user can move this Team Drive item outside of this Team Drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added. Only populated for Team Drive items.
* `can-move-item-within-team-drive=false`
    - Whether the current user can move this Team Drive item within this Team Drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added. Only populated for Team Drive items.
* `can-move-team-drive-item=true`
    - Deprecated - use canMoveItemWithinTeamDrive or canMoveItemOutOfTeamDrive instead.
* `can-read-revisions=false`
    - Whether the current user can read the revisions resource of this file. For a Team Drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read.
* `can-read-team-drive=false`
    - Whether the current user can read the Team Drive to which this file belongs. Only populated for Team Drive files.
* `can-remove-children=false`
    - Whether the current user can remove children from this folder. This is always false when the item is not a folder. For Team Drive items, use canDeleteChildren or canTrashChildren instead.
* `can-rename=false`
    - Whether the current user can rename this file.
* `can-share=true`
    - Whether the current user can modify the sharing settings for this file.
* `can-trash=true`
    - Whether the current user can move this file to trash.
* `can-trash-children=false`
    - Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-untrash=true`
    - Whether the current user can restore this file from trash.

* `..content-hints    indexable-text=sed`
    - Text to be indexed for the file to improve fullText queries. This is limited to 128KB in length and may contain HTML elements.
* `thumbnail    image=takimata`
    - The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5).
* `mime-type=sit`
    - The MIME type of the thumbnail.


* `...    copy-requires-writer-permission=false`
    - Whether the options to copy, print, or download this file, should be disabled for readers and commenters.
* `created-time=nonumy`
    - The time at which the file was created (RFC 3339 date-time).
* `description=erat`
    - A short description of the file.
* `explicitly-trashed=true`
    - Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder.
* `file-extension=erat`
    - The final component of fullFileExtension. This is only available for files with binary content in Drive.
* `folder-color-rgb=et`
    - The color for a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource.
        If an unsupported color is specified, the closest color in the palette will be used instead.
* `full-file-extension=amet`
    - The full file extension extracted from the name field. May contain multiple concatenated extensions, such as &#34;tar.gz&#34;. This is only available for files with binary content in Drive.
        This is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension.
* `has-augmented-permissions=false`
    - Whether any users are granted file access directly on this file. This field is only populated for Team Drive files.
* `has-thumbnail=false`
    - Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field.
* `head-revision-id=rebum.`
    - The ID of the file&#39;s head revision. This is currently only available for files with binary content in Drive.
* `icon-link=justo`
    - A static, unauthenticated link to the file&#39;s icon.
* `id=labore`
    - The ID of the file.
* `image-media-metadata    aperture=0.730713036025`
    - The aperture used to create the photo (f-number).
* `camera-make=takimata`
    - The make of the camera used to create the photo.
* `camera-model=dolor`
    - The model of the camera used to create the photo.
* `color-space=takimata`
    - The color space of the photo.
* `exposure-bias=0.730502575286`
    - The exposure bias of the photo (APEX value).
* `exposure-mode=no`
    - The exposure mode used to create the photo.
* `exposure-time=0.19959847489`
    - The length of the exposure, in seconds.
* `flash-used=false`
    - Whether a flash was used to create the photo.
* `focal-length=0.832472669783`
    - The focal length used to create the photo, in millimeters.
* `height=72`
    - The height of the image in pixels.
* `iso-speed=95`
    - The ISO speed used to create the photo.
* `lens=sit`
    - The lens used to create the photo.
* `location    altitude=0.661919049247`
    - The altitude stored in the image.
* `latitude=0.056790752143`
    - The latitude stored in the image.
* `longitude=0.919872822184`
    - The longitude stored in the image.

* `..    max-aperture-value=0.100433120847`
    - The smallest f-number of the lens at the focal length used to create the photo (APEX value).
* `metering-mode=consetetur`
    - The metering mode used to create the photo.
* `rotation=28`
    - The rotation in clockwise degrees from the image&#39;s original orientation.
* `sensor=clita`
    - The type of sensor used to create the photo.
* `subject-distance=92`
    - The distance to the subject of the photo, in meters.
* `time=vero`
    - The date and time the photo was taken (EXIF DateTime).
* `white-balance=dolores`
    - The white balance mode used to create the photo.
* `width=69`
    - The width of the image in pixels.

* `..    is-app-authorized=true`
    - Whether the file was created or opened by the requesting app.
* `kind=amet`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#file&#34;.
* `last-modifying-user    display-name=sed`
    - A plain text displayable name for this user.
* `email-address=sit`
    - The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
* `kind=sit`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#user&#34;.
* `me=false`
    - Whether this user is the requesting user.
* `permission-id=et`
    - The user&#39;s ID as visible in Permission resources.
* `photo-link=sanctus`
    - A link to the user&#39;s profile photo, if available.

* `..    md5-checksum=takimata`
    - The MD5 checksum for the content of the file. This is only applicable to files with binary content in Drive.
* `mime-type=kasd`
    - The MIME type of the file.
        Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded.
        If a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource.
* `modified-by-me=true`
    - Whether the file has been modified by this user.
* `modified-by-me-time=sadipscing`
    - The last time the file was modified by the user (RFC 3339 date-time).
* `modified-time=et`
    - The last time the file was modified by anyone (RFC 3339 date-time).
        Note that setting modifiedTime will also update modifiedByMeTime for the user.
* `name=clita`
    - The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of Team Drives, My Drive root folder, and Application Data folder the name is constant.
* `original-filename=ipsum`
    - The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Drive.
* `owned-by-me=false`
    - Whether the user owns the file. Not populated for Team Drive files.
* `parents=elitr`
    - The IDs of the parent folders which contain the file.
        If not specified as part of a create request, the file will be placed directly in the user&#39;s My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list.
    - Each invocation of this argument appends the given value to the array.
* `permission-ids=magna`
    - List of permission IDs for users with access to this file.
    - Each invocation of this argument appends the given value to the array.
* `properties=key=aliquyam`
    - A collection of arbitrary key-value pairs which are visible to all apps.
        Entries with null values are cleared in update and copy requests.
    - the value will be associated with the given `key`
* `quota-bytes-used=kasd`
    - The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled.
* `shared=false`
    - Whether the file has been shared. Not populated for Team Drive files.
* `shared-with-me-time=et`
    - The time at which the file was shared with the user, if applicable (RFC 3339 date-time).
* `sharing-user    display-name=sit`
    - A plain text displayable name for this user.
* `email-address=eirmod`
    - The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
* `kind=no`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#user&#34;.
* `me=false`
    - Whether this user is the requesting user.
* `permission-id=sed`
    - The user&#39;s ID as visible in Permission resources.
* `photo-link=tempor`
    - A link to the user&#39;s profile photo, if available.

* `..    size=clita`
    - The size of the file&#39;s content in bytes. This is only applicable to files with binary content in Drive.
* `spaces=kasd`
    - The list of spaces which contain the file. The currently supported values are &#39;drive&#39;, &#39;appDataFolder&#39; and &#39;photos&#39;.
    - Each invocation of this argument appends the given value to the array.
* `starred=false`
    - Whether the user has starred the file.
* `team-drive-id=et`
    - ID of the Team Drive the file resides in.
* `thumbnail-link=vero`
    - A short-lived link to the file&#39;s thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file&#39;s content.
* `thumbnail-version=sadipscing`
    - The thumbnail version for use in thumbnail cache invalidation.
* `trashed=true`
    - Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, and other users cannot see files in the owner&#39;s trash.
* `trashed-time=vero`
    - The time that the item was trashed (RFC 3339 date-time). Only populated for Team Drive files.
* `trashing-user    display-name=nonumy`
    - A plain text displayable name for this user.
* `email-address=magna`
    - The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester.
* `kind=ea`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#user&#34;.
* `me=false`
    - Whether this user is the requesting user.
* `permission-id=ipsum`
    - The user&#39;s ID as visible in Permission resources.
* `photo-link=labore`
    - A link to the user&#39;s profile photo, if available.

* `..    version=lorem`
    - A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user.
* `video-media-metadata    duration-millis=aliquyam`
    - The duration of the video in milliseconds.
* `height=94`
    - The height of the video in pixels.
* `width=67`
    - The width of the video in pixels.

* `..    viewed-by-me=true`
    - Whether the file has been viewed by this user.
* `viewed-by-me-time=et`
    - The last time the file was viewed by the user (RFC 3339 date-time).
* `viewers-can-copy-content=false`
    - Deprecated - use copyRequiresWriterPermission instead.
* `web-content-link=kasd`
    - A link for downloading the content of the file in a browser. This is only available for files with binary content in Drive.
* `web-view-link=kasd`
    - A link for opening the file in a relevant Google editor or viewer in a browser.
* `writers-can-share=true`
    - Whether users with only writer permission can modify the file&#39;s permissions. Not populated for Team Drive files.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.

# Required Upload Flags

This method supports the upload of data, which *requires* all of the following flags to be set:

* **-u (simple|resumable)**
    - **simple** - Upload media all at once.
    - **resumable** - Upload media in a resumable fashion.
* **-f file**
    - Path to file to upload. It must be seekable.

The following flag *may* be set: 

* **-m mime**
    - the mime type, like 'application/octet-stream', which is the default


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

* **-p use-content-as-indexable-text=boolean**
    - Whether to use the uploaded content as indexable text.

* **-p remove-parents=string**
    - A comma-separated list of parent IDs to remove.

* **-p keep-revision-forever=boolean**
    - Whether to set the &#39;keepForever&#39; field in the new head revision. This is only applicable to files with binary content in Drive.

* **-p supports-team-drives=boolean**
    - Whether the requesting application supports Team Drives.

* **-p ocr-language=string**
    - A language hint for OCR processing during image import (ISO 639-1 code).

* **-p add-parents=string**
    - A comma-separated list of parent IDs to add.

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
