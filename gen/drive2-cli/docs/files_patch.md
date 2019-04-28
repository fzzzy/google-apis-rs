Updates file metadata and/or content. This method supports patch semantics.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.appdata*
* *https://www.googleapis.com/auth/drive.apps.readonly*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/drive.metadata*
* *https://www.googleapis.com/auth/drive.scripts*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive2 --scope <scope> files patch ...`
# Required Scalar Argument
* **&lt;file-id&gt;** *(string)*
    - The ID of the file to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
File:
  alternate-link: string
  app-data-contents: boolean
  can-comment: boolean
  can-read-revisions: boolean
  capabilities:
    can-add-children: boolean
    can-change-copy-requires-writer-permission: boolean
    can-change-restricted-download: boolean
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
  copy-requires-writer-permission: boolean
  copyable: boolean
  created-date: string
  default-open-with-link: string
  description: string
  download-url: string
  editable: boolean
  embed-link: string
  etag: string
  explicitly-trashed: boolean
  export-links: { string: string }
  file-extension: string
  file-size: string
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
    date: string
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
    white-balance: string
    width: integer
  indexable-text:
    text: string
  is-app-authorized: boolean
  kind: string
  labels:
    hidden: boolean
    modified: boolean
    restricted: boolean
    starred: boolean
    trashed: boolean
    viewed: boolean
  last-modifying-user:
    display-name: string
    email-address: string
    is-authenticated-user: boolean
    kind: string
    permission-id: string
    picture:
      url: string
  last-modifying-user-name: string
  last-viewed-by-me-date: string
  marked-viewed-by-me-date: string
  md5-checksum: string
  mime-type: string
  modified-by-me-date: string
  modified-date: string
  open-with-links: { string: string }
  original-filename: string
  owned-by-me: boolean
  owner-names: [string]
  permission-ids: [string]
  quota-bytes-used: string
  self-link: string
  shareable: boolean
  shared: boolean
  shared-with-me-date: string
  sharing-user:
    display-name: string
    email-address: string
    is-authenticated-user: boolean
    kind: string
    permission-id: string
    picture:
      url: string
  spaces: [string]
  team-drive-id: string
  thumbnail:
    image: string
    mime-type: string
  thumbnail-link: string
  thumbnail-version: string
  title: string
  trashed-date: string
  trashing-user:
    display-name: string
    email-address: string
    is-authenticated-user: boolean
    kind: string
    permission-id: string
    picture:
      url: string
  user-permission:
    additional-roles: [string]
    auth-key: string
    deleted: boolean
    domain: string
    email-address: string
    etag: string
    expiration-date: string
    id: string
    kind: string
    name: string
    photo-link: string
    role: string
    self-link: string
    type: string
    value: string
    with-link: boolean
  version: string
  video-media-metadata:
    duration-millis: string
    height: integer
    width: integer
  web-content-link: string
  web-view-link: string
  writers-can-share: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternate-link=kasd`
    - A link for opening the file in a relevant Google editor or viewer.
* `app-data-contents=false`
    - Whether this file is in the Application Data folder.
* `can-comment=false`
    - Deprecated: use capabilities/canComment.
* `can-read-revisions=false`
    - Deprecated: use capabilities/canReadRevisions.
* `capabilities    can-add-children=true`
    - Whether the current user can add children to this folder. This is always false when the item is not a folder.
* `can-change-copy-requires-writer-permission=true`
    - Whether the current user can change the copyRequiresWriterPermission restriction of this file.
* `can-change-restricted-download=false`
    - Deprecated
* `can-comment=false`
    - Whether the current user can comment on this file.
* `can-copy=false`
    - Whether the current user can copy this file. For a Team Drive item, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder.
* `can-delete=false`
    - Whether the current user can delete this file.
* `can-delete-children=false`
    - Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-download=true`
    - Whether the current user can download this file.
* `can-edit=true`
    - Whether the current user can edit this file.
* `can-list-children=true`
    - Whether the current user can list the children of this folder. This is always false when the item is not a folder.
* `can-move-children-out-of-team-drive=true`
    - Whether the current user can move children of this folder outside of the Team Drive. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-move-children-within-team-drive=true`
    - Whether the current user can move children of this folder within the Team Drive. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-move-item-into-team-drive=true`
    - Whether the current user can move this item into a Team Drive. If the item is in a Team Drive, this field is equivalent to canMoveTeamDriveItem.
* `can-move-item-out-of-team-drive=true`
    - Whether the current user can move this Team Drive item outside of this Team Drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added. Only populated for Team Drive items.
* `can-move-item-within-team-drive=true`
    - Whether the current user can move this Team Drive item within this Team Drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added. Only populated for Team Drive items.
* `can-move-team-drive-item=false`
    - Deprecated - use canMoveItemWithinTeamDrive or canMoveItemOutOfTeamDrive instead.
* `can-read-revisions=false`
    - Whether the current user can read the revisions resource of this file. For a Team Drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read.
* `can-read-team-drive=false`
    - Whether the current user can read the Team Drive to which this file belongs. Only populated for Team Drive files.
* `can-remove-children=true`
    - Whether the current user can remove children from this folder. This is always false when the item is not a folder. For Team Drive items, use canDeleteChildren or canTrashChildren instead.
* `can-rename=true`
    - Whether the current user can rename this file.
* `can-share=false`
    - Whether the current user can modify the sharing settings for this file.
* `can-trash=true`
    - Whether the current user can move this file to trash.
* `can-trash-children=false`
    - Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for Team Drive items.
* `can-untrash=false`
    - Whether the current user can restore this file from trash.

* `..    copy-requires-writer-permission=true`
    - Whether the options to copy, print, or download this file, should be disabled for readers and commenters.
* `copyable=true`
    - Deprecated: use capabilities/canCopy.
* `created-date=voluptua.`
    - Create time for this file (formatted RFC 3339 timestamp).
* `default-open-with-link=est`
    - A link to open this file with the user&#39;s default app for this file. Only populated when the drive.apps.readonly scope is used.
* `description=consetetur`
    - A short description of the file.
* `download-url=tempor`
    - No description provided.
* `editable=false`
    - Deprecated: use capabilities/canEdit.
* `embed-link=lorem`
    - A link for embedding the file.
* `etag=eirmod`
    - ETag of the file.
* `explicitly-trashed=true`
    - Whether this file has been explicitly trashed, as opposed to recursively trashed.
* `export-links=key=clita`
    - Links for exporting Google Docs to specific formats.
    - the value will be associated with the given `key`
* `file-extension=et`
    - The final component of fullFileExtension with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Drive; it is not populated for Google Docs or shortcut files.
* `file-size=eirmod`
    - The size of the file in bytes. This field is only populated for files with content stored in Drive; it is not populated for Google Docs or shortcut files.
* `folder-color-rgb=ea`
    - Folder color as an RGB hex string if the file is a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette. Not populated for Team Drive files.
* `full-file-extension=et`
    - The full file extension; extracted from the title. May contain multiple concatenated extensions, such as &#34;tar.gz&#34;. Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Drive; it is not populated for Google Docs or shortcut files.
* `has-augmented-permissions=true`
    - Whether any users are granted file access directly on this file. This field is only populated for Team Drive files.
* `has-thumbnail=true`
    - Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field.
* `head-revision-id=sea`
    - The ID of the file&#39;s head revision. This field is only populated for files with content stored in Drive; it is not populated for Google Docs or shortcut files.
* `icon-link=ipsum`
    - A link to the file&#39;s icon.
* `id=gubergren`
    - The ID of the file.
* `image-media-metadata    aperture=0.761199558813`
    - The aperture used to create the photo (f-number).
* `camera-make=sed`
    - The make of the camera used to create the photo.
* `camera-model=sit`
    - The model of the camera used to create the photo.
* `color-space=eirmod`
    - The color space of the photo.
* `date=stet`
    - The date and time the photo was taken (EXIF format timestamp).
* `exposure-bias=0.910337635131`
    - The exposure bias of the photo (APEX value).
* `exposure-mode=sanctus`
    - The exposure mode used to create the photo.
* `exposure-time=0.399302648226`
    - The length of the exposure, in seconds.
* `flash-used=false`
    - Whether a flash was used to create the photo.
* `focal-length=0.632992171287`
    - The focal length used to create the photo, in millimeters.
* `height=23`
    - The height of the image in pixels.
* `iso-speed=71`
    - The ISO speed used to create the photo.
* `lens=ipsum`
    - The lens used to create the photo.
* `location    altitude=0.402801225086`
    - The altitude stored in the image.
* `latitude=0.750828337627`
    - The latitude stored in the image.
* `longitude=0.588317255233`
    - The longitude stored in the image.

* `..    max-aperture-value=0.255956720197`
    - The smallest f-number of the lens at the focal length used to create the photo (APEX value).
* `metering-mode=dolore`
    - The metering mode used to create the photo.
* `rotation=0`
    - The rotation in clockwise degrees from the image&#39;s original orientation.
* `sensor=invidunt`
    - The type of sensor used to create the photo.
* `subject-distance=31`
    - The distance to the subject of the photo, in meters.
* `white-balance=dolor`
    - The white balance mode used to create the photo.
* `width=28`
    - The width of the image in pixels.

* `..indexable-text    text=diam`
    - The text to be indexed for this file.

* `..    is-app-authorized=false`
    - Whether the file was created or opened by the requesting app.
* `kind=eirmod`
    - The type of file. This is always drive#file.
* `labels    hidden=false`
    - Deprecated.
* `modified=false`
    - Whether the file has been modified by this user.
* `restricted=false`
    - Deprecated - use copyRequiresWriterPermission instead.
* `starred=true`
    - Whether this file is starred by the user.
* `trashed=true`
    - Whether this file has been trashed. This label applies to all users accessing the file; however, only owners are allowed to see and untrash files.
* `viewed=false`
    - Whether this file has been viewed by this user.

* `..last-modifying-user    display-name=sanctus`
    - A plain text displayable name for this user.
* `email-address=dolor`
    - The email address of the user.
* `is-authenticated-user=false`
    - Whether this user is the same as the authenticated user for whom the request was made.
* `kind=dolore`
    - This is always drive#user.
* `permission-id=duo`
    - The user&#39;s ID as visible in the permissions collection.
* `picture    url=ipsum`
    - A URL that points to a profile picture of this user.


* `...    last-modifying-user-name=diam`
    - Name of the last user to modify this file.
* `last-viewed-by-me-date=sed`
    - Last time this file was viewed by the user (formatted RFC 3339 timestamp).
* `marked-viewed-by-me-date=et`
    - Deprecated.
* `md5-checksum=tempor`
    - An MD5 checksum for the content of this file. This field is only populated for files with content stored in Drive; it is not populated for Google Docs or shortcut files.
* `mime-type=justo`
    - The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content&#39;s MIME type.
* `modified-by-me-date=takimata`
    - Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date.
* `modified-date=invidunt`
    - Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set.
* `open-with-links=key=amet`
    - A map of the id of each of the user&#39;s apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used.
    - the value will be associated with the given `key`
* `original-filename=dolor`
    - The original filename of the uploaded content if available, or else the original value of the title field. This is only available for files with binary content in Drive.
* `owned-by-me=true`
    - Whether the file is owned by the current user. Not populated for Team Drive files.
* `owner-names=et`
    - Name(s) of the owner(s) of this file. Not populated for Team Drive files.
    - Each invocation of this argument appends the given value to the array.
* `permission-ids=elitr`
    - List of permission IDs for users with access to this file.
    - Each invocation of this argument appends the given value to the array.
* `quota-bytes-used=kasd`
    - The number of quota bytes used by this file.
* `self-link=sit`
    - A link back to this file.
* `shareable=false`
    - Deprecated: use capabilities/canShare.
* `shared=false`
    - Whether the file has been shared. Not populated for Team Drive files.
* `shared-with-me-date=ipsum`
    - Time at which this file was shared with the user (formatted RFC 3339 timestamp).
* `sharing-user    display-name=amet.`
    - A plain text displayable name for this user.
* `email-address=dolor`
    - The email address of the user.
* `is-authenticated-user=false`
    - Whether this user is the same as the authenticated user for whom the request was made.
* `kind=ut`
    - This is always drive#user.
* `permission-id=magna`
    - The user&#39;s ID as visible in the permissions collection.
* `picture    url=amet.`
    - A URL that points to a profile picture of this user.


* `...    spaces=eirmod`
    - The list of spaces which contain the file. Supported values are &#39;drive&#39;, &#39;appDataFolder&#39; and &#39;photos&#39;.
    - Each invocation of this argument appends the given value to the array.
* `team-drive-id=ea`
    - ID of the Team Drive the file resides in.
* `thumbnail    image=ea`
    - The URL-safe Base64 encoded bytes of the thumbnail image. It should conform to RFC 4648 section 5.
* `mime-type=et`
    - The MIME type of the thumbnail.

* `..    thumbnail-link=sanctus`
    - A short-lived link to the file&#39;s thumbnail. Typically lasts on the order of hours. Only populated when the requesting app can access the file&#39;s content.
* `thumbnail-version=diam`
    - The thumbnail version for use in thumbnail cache invalidation.
* `title=eos`
    - The title of this file. Note that for immutable items such as the top level folders of Team Drives, My Drive root folder, and Application Data folder the title is constant.
* `trashed-date=tempor`
    - The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for Team Drive files.
* `trashing-user    display-name=sea`
    - A plain text displayable name for this user.
* `email-address=ea`
    - The email address of the user.
* `is-authenticated-user=false`
    - Whether this user is the same as the authenticated user for whom the request was made.
* `kind=duo`
    - This is always drive#user.
* `permission-id=tempor`
    - The user&#39;s ID as visible in the permissions collection.
* `picture    url=consetetur`
    - A URL that points to a profile picture of this user.


* `...user-permission    additional-roles=eirmod`
    - Additional roles for this user. Only commenter is currently allowed, though more may be supported in the future.
    - Each invocation of this argument appends the given value to the array.
* `auth-key=aliquyam`
    - Deprecated.
* `deleted=true`
    - Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions.
* `domain=aliquyam`
    - The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is user, group or domain.
* `email-address=accusam`
    - The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is user or group.
* `etag=sed`
    - The ETag of the permission.
* `expiration-date=sea`
    - The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions:  
        - They can only be set on user and group permissions 
        - The date must be in the future 
        - The date cannot be more than a year in the future 
        - The date can only be set on drive.permissions.update or drive.permissions.patch requests
* `id=amet.`
    - The ID of the user this permission refers to, and identical to the permissionId in the About and Files resources. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored.
* `kind=clita`
    - This is always drive#permission.
* `name=clita`
    - The name for this permission.
* `photo-link=est`
    - A link to the profile photo, if available.
* `role=sit`
    - The primary role for this user. While new values may be supported in the future, the following are currently allowed:  
        - owner 
        - organizer 
        - fileOrganizer 
        - writer 
        - reader
* `self-link=ipsum`
    - A link back to this permission.
* `type=dolore`
    - The account type. Allowed values are:  
        - user 
        - group 
        - domain 
        - anyone
* `value=sadipscing`
    - The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a drive.permissions.insert request, exactly one of the id or value fields must be specified unless the permission type is anyone, in which case both id and value are ignored.
* `with-link=false`
    - Whether the link is required for this permission.

* `..    version=sadipscing`
    - A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user.
* `video-media-metadata    duration-millis=ea`
    - The duration of the video in milliseconds.
* `height=52`
    - The height of the video in pixels.
* `width=5`
    - The width of the video in pixels.

* `..    web-content-link=sadipscing`
    - A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials.
* `web-view-link=ipsum`
    - A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive&#39;s Website Hosting.
* `writers-can-share=true`
    - Whether writers can share the document with other users. Not populated for Team Drive files.


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

* **-p supports-team-drives=boolean**
    - Whether the requesting application supports Team Drives.

* **-p convert=boolean**
    - This parameter is deprecated and has no function.

* **-p new-revision=boolean**
    - Whether a blob upload should create a new revision. If false, the blob data in the current head revision is replaced. If true or not set, a new blob is created as head revision, and previous unpinned revisions are preserved for a short period of time. Pinned revisions are stored indefinitely, using additional storage quota, up to a maximum of 200 revisions. For details on how revisions are retained, see the Drive Help Center.

* **-p add-parents=string**
    - Comma-separated list of parent IDs to add.

* **-p set-modified-date=boolean**
    - Whether to set the modified date using the value supplied in the request body. Setting this field to true is equivalent to modifiedDateBehavior=fromBodyOrNow, and false is equivalent to modifiedDateBehavior=now. To prevent any changes to the modified date set modifiedDateBehavior=noChange.

* **-p ocr-language=string**
    - If ocr is true, hints at the language to use. Valid values are BCP 47 codes.

* **-p use-content-as-indexable-text=boolean**
    - Whether to use the content as indexable text.

* **-p update-viewed-date=boolean**
    - Whether to update the view date after successfully updating the file.

* **-p modified-date-behavior=string**
    - Determines the behavior in which modifiedDate is updated. This overrides setModifiedDate.

* **-p remove-parents=string**
    - Comma-separated list of parent IDs to remove.

* **-p pinned=boolean**
    - Whether to pin the new revision. A file can have a maximum of 200 pinned revisions.

* **-p ocr=boolean**
    - Whether to attempt OCR on .jpg, .png, .gif, or .pdf uploads.

* **-p timed-text-track-name=string**
    - The timed text track name.

* **-p timed-text-language=string**
    - The language of the timed text.

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