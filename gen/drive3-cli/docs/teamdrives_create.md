Creates a new Team Drive.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/drive* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive3 --scope <scope> teamdrives create ...`
# Required Scalar Argument
* **&lt;request-id&gt;** *(string)*
    - An ID, such as a random UUID, which uniquely identifies this user&#39;s request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
TeamDrive:
  background-image-file:
    id: string
    width: number
    x-coordinate: number
    y-coordinate: number
  background-image-link: string
  capabilities:
    can-add-children: boolean
    can-change-copy-requires-writer-permission-restriction: boolean
    can-change-domain-users-only-restriction: boolean
    can-change-team-drive-background: boolean
    can-change-team-members-only-restriction: boolean
    can-comment: boolean
    can-copy: boolean
    can-delete-children: boolean
    can-delete-team-drive: boolean
    can-download: boolean
    can-edit: boolean
    can-list-children: boolean
    can-manage-members: boolean
    can-read-revisions: boolean
    can-remove-children: boolean
    can-rename: boolean
    can-rename-team-drive: boolean
    can-share: boolean
    can-trash-children: boolean
  color-rgb: string
  created-time: string
  id: string
  kind: string
  name: string
  restrictions:
    admin-managed-restrictions: boolean
    copy-requires-writer-permission: boolean
    domain-users-only: boolean
    team-members-only: boolean
  theme-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .background-image-file    id=justo`
    - The ID of an image file in Drive to use for the background image.
* `width=0.396249186547`
    - The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.
* `x-coordinate=0.465606723652`
    - The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.
* `y-coordinate=0.9995102253`
    - The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.

* `..    background-image-link=dolor`
    - A short-lived link to this Team Drive&#39;s background image.
* `capabilities    can-add-children=false`
    - Whether the current user can add children to folders in this Team Drive.
* `can-change-copy-requires-writer-permission-restriction=false`
    - Whether the current user can change the copyRequiresWriterPermission restriction of this Team Drive.
* `can-change-domain-users-only-restriction=true`
    - Whether the current user can change the domainUsersOnly restriction of this Team Drive.
* `can-change-team-drive-background=true`
    - Whether the current user can change the background of this Team Drive.
* `can-change-team-members-only-restriction=true`
    - Whether the current user can change the teamMembersOnly restriction of this Team Drive.
* `can-comment=false`
    - Whether the current user can comment on files in this Team Drive.
* `can-copy=true`
    - Whether the current user can copy files in this Team Drive.
* `can-delete-children=true`
    - Whether the current user can delete children from folders in this Team Drive.
* `can-delete-team-drive=true`
    - Whether the current user can delete this Team Drive. Attempting to delete the Team Drive may still fail if there are untrashed items inside the Team Drive.
* `can-download=false`
    - Whether the current user can download files in this Team Drive.
* `can-edit=false`
    - Whether the current user can edit files in this Team Drive
* `can-list-children=true`
    - Whether the current user can list the children of folders in this Team Drive.
* `can-manage-members=true`
    - Whether the current user can add members to this Team Drive or remove them or change their role.
* `can-read-revisions=true`
    - Whether the current user can read the revisions resource of files in this Team Drive.
* `can-remove-children=false`
    - Deprecated - use canDeleteChildren or canTrashChildren instead.
* `can-rename=false`
    - Whether the current user can rename files or folders in this Team Drive.
* `can-rename-team-drive=true`
    - Whether the current user can rename this Team Drive.
* `can-share=true`
    - Whether the current user can share files or folders in this Team Drive.
* `can-trash-children=false`
    - Whether the current user can trash children from folders in this Team Drive.

* `..    color-rgb=aliquyam`
    - The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId.
* `created-time=no`
    - The time at which the Team Drive was created (RFC 3339 date-time).
* `id=aliquyam`
    - The ID of this Team Drive which is also the ID of the top level folder of this Team Drive.
* `kind=accusam`
    - Identifies what kind of resource this is. Value: the fixed string &#34;drive#teamDrive&#34;.
* `name=sed`
    - The name of this Team Drive.
* `restrictions    admin-managed-restrictions=false`
    - Whether administrative privileges on this Team Drive are required to modify restrictions.
* `copy-requires-writer-permission=true`
    - Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive.
* `domain-users-only=true`
    - Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive.
* `team-members-only=true`
    - Whether access to items inside this Team Drive is restricted to members of this Team Drive.

* `..    theme-id=est`
    - The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don&#39;t set colorRgb or backgroundImageFile.


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
