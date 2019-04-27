Updates or creates a policy.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidmanagement* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidmanagement*.
You can set the scope for this method like this: `androidmanagement1 --scope <scope> enterprises policies-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Policy:
  account-types-with-management-disabled: [string]
  add-user-disabled: boolean
  adjust-volume-disabled: boolean
  always-on-vpn-package:
    lockdown-enabled: boolean
    package-name: string
  android-device-policy-tracks: [string]
  app-auto-update-policy: string
  auto-time-required: boolean
  block-applications-enabled: boolean
  bluetooth-config-disabled: boolean
  bluetooth-contact-sharing-disabled: boolean
  bluetooth-disabled: boolean
  camera-disabled: boolean
  cell-broadcasts-config-disabled: boolean
  create-windows-disabled: boolean
  credentials-config-disabled: boolean
  data-roaming-disabled: boolean
  debugging-features-allowed: boolean
  default-permission-policy: string
  device-owner-lock-screen-info:
    default-message: string
    localized-messages: { string: string }
  encryption-policy: string
  ensure-verify-apps-enabled: boolean
  factory-reset-disabled: boolean
  frp-admin-emails: [string]
  fun-disabled: boolean
  install-apps-disabled: boolean
  install-unknown-sources-allowed: boolean
  keyguard-disabled: boolean
  keyguard-disabled-features: [string]
  kiosk-custom-launcher-enabled: boolean
  location-mode: string
  long-support-message:
    default-message: string
    localized-messages: { string: string }
  maximum-time-to-lock: string
  mobile-networks-config-disabled: boolean
  modify-accounts-disabled: boolean
  mount-physical-media-disabled: boolean
  name: string
  network-escape-hatch-enabled: boolean
  network-reset-disabled: boolean
  outgoing-beam-disabled: boolean
  outgoing-calls-disabled: boolean
  password-requirements:
    maximum-failed-passwords-for-wipe: integer
    password-expiration-timeout: string
    password-history-length: integer
    password-minimum-length: integer
    password-minimum-letters: integer
    password-minimum-lower-case: integer
    password-minimum-non-letter: integer
    password-minimum-numeric: integer
    password-minimum-symbols: integer
    password-minimum-upper-case: integer
    password-quality: string
  permitted-input-methods:
    package-names: [string]
  play-store-mode: string
  private-key-selection-enabled: boolean
  recommended-global-proxy:
    excluded-hosts: [string]
    host: string
    pac-uri: string
    port: integer
  remove-user-disabled: boolean
  safe-boot-disabled: boolean
  screen-capture-disabled: boolean
  set-user-icon-disabled: boolean
  set-wallpaper-disabled: boolean
  share-location-disabled: boolean
  short-support-message:
    default-message: string
    localized-messages: { string: string }
  skip-first-use-hints-enabled: boolean
  sms-disabled: boolean
  status-bar-disabled: boolean
  status-reporting-settings:
    application-reports-enabled: boolean
    device-settings-enabled: boolean
    display-info-enabled: boolean
    hardware-status-enabled: boolean
    memory-info-enabled: boolean
    network-info-enabled: boolean
    power-management-events-enabled: boolean
    software-info-enabled: boolean
  stay-on-plugged-modes: [string]
  system-update:
    end-minutes: integer
    start-minutes: integer
    type: string
  tethering-config-disabled: boolean
  uninstall-apps-disabled: boolean
  unmute-microphone-disabled: boolean
  usb-file-transfer-disabled: boolean
  usb-mass-storage-enabled: boolean
  version: string
  vpn-config-disabled: boolean
  wifi-config-disabled: boolean
  wifi-configs-lockdown-enabled: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-types-with-management-disabled=nonumy`
    - Account types that can&#39;t be managed by the user.
    - Each invocation of this argument appends the given value to the array.
* `add-user-disabled=true`
    - Whether adding new users and profiles is disabled.
* `adjust-volume-disabled=true`
    - Whether adjusting the master volume is disabled.
* `always-on-vpn-package    lockdown-enabled=false`
    - Disallows networking when the VPN is not connected.
* `package-name=at`
    - The package name of the VPN app.

* `..    android-device-policy-tracks=labore`
    - The app tracks for Android Device Policy the device can access. The device receives the latest version among all accessible tracks. If no tracks are specified, then the device only uses the production track.
    - Each invocation of this argument appends the given value to the array.
* `app-auto-update-policy=invidunt`
    - The app auto update policy, which controls when automatic app updates can be applied.
* `auto-time-required=false`
    - Whether auto time is required, which prevents the user from manually setting the date and time.
* `block-applications-enabled=false`
    - Whether applications other than the ones configured in applications are blocked from being installed. When set, applications that were installed under a previous policy but no longer appear in the policy are automatically uninstalled.
* `bluetooth-config-disabled=false`
    - Whether configuring bluetooth is disabled.
* `bluetooth-contact-sharing-disabled=true`
    - Whether bluetooth contact sharing is disabled.
* `bluetooth-disabled=true`
    - Whether bluetooth is disabled. Prefer this setting over bluetooth_config_disabled because bluetooth_config_disabled can be bypassed by the user.
* `camera-disabled=true`
    - Whether all cameras on the device are disabled.
* `cell-broadcasts-config-disabled=false`
    - Whether configuring cell broadcast is disabled.
* `create-windows-disabled=false`
    - Whether creating windows besides app windows is disabled.
* `credentials-config-disabled=true`
    - Whether configuring user credentials is disabled.
* `data-roaming-disabled=true`
    - Whether roaming data services are disabled.
* `debugging-features-allowed=false`
    - Whether the user is allowed to enable debugging features.
* `default-permission-policy=sed`
    - The default permission policy for runtime permission requests.
* `device-owner-lock-screen-info    default-message=ea`
    - The default message displayed if no localized message is specified or the user&#39;s locale doesn&#39;t match with any of the localized messages. A default message must be provided if any localized messages are provided.
* `localized-messages=key=gubergren`
    - A map containing &lt;locale, message&gt; pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr.
    - the value will be associated with the given `key`

* `..    encryption-policy=aliquyam`
    - Whether encryption is enabled
* `ensure-verify-apps-enabled=true`
    - Whether app verification is force-enabled.
* `factory-reset-disabled=true`
    - Whether factory resetting from settings is disabled.
* `frp-admin-emails=sea`
    - Email addresses of device administrators for factory reset protection. When the device is factory reset, it will require one of these admins to log in with the Google account email and password to unlock the device. If no admins are specified, the device won&#39;t provide factory reset protection.
    - Each invocation of this argument appends the given value to the array.
* `fun-disabled=false`
    - Whether the user is allowed to have fun. Controls whether the Easter egg game in Settings is disabled.
* `install-apps-disabled=false`
    - Whether user installation of apps is disabled.
* `install-unknown-sources-allowed=true`
    - Whether the user is allowed to enable the &#34;Unknown Sources&#34; setting, which allows installation of apps from unknown sources.
* `keyguard-disabled=false`
    - Whether the keyguard is disabled.
* `keyguard-disabled-features=sit`
    - Disabled keyguard customizations, such as widgets.
    - Each invocation of this argument appends the given value to the array.
* `kiosk-custom-launcher-enabled=true`
    - Whether the kiosk custom launcher is enabled. This replaces the home screen with a launcher that locks down the device to the apps installed via the applications setting. The apps appear on a single page in alphabetical order. It is recommended to also use status_bar_disabled to block access to device settings.
* `location-mode=ut`
    - The degree of location detection enabled. The user may change the value unless the user is otherwise blocked from accessing device settings.
* `long-support-message    default-message=justo`
    - The default message displayed if no localized message is specified or the user&#39;s locale doesn&#39;t match with any of the localized messages. A default message must be provided if any localized messages are provided.
* `localized-messages=key=est`
    - A map containing &lt;locale, message&gt; pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr.
    - the value will be associated with the given `key`

* `..    maximum-time-to-lock=amet`
    - Maximum time in milliseconds for user activity until the device locks. A value of 0 means there is no restriction.
* `mobile-networks-config-disabled=true`
    - Whether configuring mobile networks is disabled.
* `modify-accounts-disabled=true`
    - Whether adding or removing accounts is disabled.
* `mount-physical-media-disabled=false`
    - Whether the user mounting physical external media is disabled.
* `name=justo`
    - The name of the policy in the form enterprises/{enterpriseId}/policies/{policyId}.
* `network-escape-hatch-enabled=false`
    - Whether the network escape hatch is enabled. If a network connection can&#39;t be made at boot time, the escape hatch prompts the user to temporarily connect to a network in order to refresh the device policy. After applying policy, the temporary network will be forgotten and the device will continue booting. This prevents being unable to connect to a network if there is no suitable network in the last policy and the device boots into an app in lock task mode, or the user is otherwise unable to reach device settings.
* `network-reset-disabled=false`
    - Whether resetting network settings is disabled.
* `outgoing-beam-disabled=true`
    - Whether using NFC to beam data from apps is disabled.
* `outgoing-calls-disabled=false`
    - Whether outgoing calls are disabled.
* `password-requirements    maximum-failed-passwords-for-wipe=82`
    - Number of incorrect device-unlock passwords that can be entered before a device is wiped. A value of 0 means there is no restriction.
* `password-expiration-timeout=eos`
    - Password expiration timeout.
* `password-history-length=23`
    - The length of the password history. After setting this field, the user won&#39;t be able to enter a new password that is the same as any password in the history. A value of 0 means there is no restriction.
* `password-minimum-length=82`
    - The minimum allowed password length. A value of 0 means there is no restriction. Only enforced when password_quality is NUMERIC, NUMERIC_COMPLEX, ALPHABETIC, ALPHANUMERIC, or COMPLEX.
* `password-minimum-letters=8`
    - Minimum number of letters required in the password. Only enforced when password_quality is COMPLEX.
* `password-minimum-lower-case=70`
    - Minimum number of lower case letters required in the password. Only enforced when password_quality is COMPLEX.
* `password-minimum-non-letter=34`
    - Minimum number of non-letter characters (numerical digits or symbols) required in the password. Only enforced when password_quality is COMPLEX.
* `password-minimum-numeric=84`
    - Minimum number of numerical digits required in the password. Only enforced when password_quality is COMPLEX.
* `password-minimum-symbols=27`
    - Minimum number of symbols required in the password. Only enforced when password_quality is COMPLEX.
* `password-minimum-upper-case=53`
    - Minimum number of upper case letters required in the password. Only enforced when password_quality is COMPLEX.
* `password-quality=diam`
    - The required password quality.

* `..permitted-input-methods    package-names=kasd`
    - A list of package names.
    - Each invocation of this argument appends the given value to the array.

* `..    play-store-mode=invidunt`
    - This mode controls which apps are available to the user in the Play Store and the behavior on the device when apps are removed from the policy.
* `private-key-selection-enabled=true`
    - Allows showing UI on a device for a user to choose a private key alias if there are no matching rules in ChoosePrivateKeyRules. For devices below Android P, setting this may leave enterprise keys vulnerable.
* `recommended-global-proxy    excluded-hosts=lorem`
    - For a direct proxy, the hosts for which the proxy is bypassed. The host names may contain wildcards such as *.example.com.
    - Each invocation of this argument appends the given value to the array.
* `host=clita`
    - The host of the direct proxy.
* `pac-uri=invidunt`
    - The URI of the PAC script used to configure the proxy.
* `port=11`
    - The port of the direct proxy.

* `..    remove-user-disabled=false`
    - Whether removing other users is disabled.
* `safe-boot-disabled=false`
    - Whether rebooting the device into safe boot is disabled.
* `screen-capture-disabled=true`
    - Whether screen capture is disabled.
* `set-user-icon-disabled=false`
    - Whether changing the user icon is disabled.
* `set-wallpaper-disabled=true`
    - Whether changing the wallpaper is disabled.
* `share-location-disabled=true`
    - Whether location sharing is disabled.
* `short-support-message    default-message=elitr`
    - The default message displayed if no localized message is specified or the user&#39;s locale doesn&#39;t match with any of the localized messages. A default message must be provided if any localized messages are provided.
* `localized-messages=key=nonumy`
    - A map containing &lt;locale, message&gt; pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr.
    - the value will be associated with the given `key`

* `..    skip-first-use-hints-enabled=true`
    - Flag to skip hints on the first use. Enterprise admin can enable the system recommendation for apps to skip their user tutorial and other introductory hints on first start-up.
* `sms-disabled=true`
    - Whether sending and receiving SMS messages is disabled.
* `status-bar-disabled=true`
    - Whether the status bar is disabled. This disables notifications, quick settings, and other screen overlays that allow escape from full-screen mode.
* `status-reporting-settings    application-reports-enabled=true`
    - Whether app reports are enabled.
* `device-settings-enabled=true`
    - Whether device settings reporting is enabled.
* `display-info-enabled=true`
    - Whether displays reporting is enabled.
* `hardware-status-enabled=false`
    - Whether hardware status reporting is enabled.
* `memory-info-enabled=false`
    - Whether memory reporting is enabled.
* `network-info-enabled=false`
    - Whether network info reporting is enabled.
* `power-management-events-enabled=true`
    - Whether power management event reporting is enabled.
* `software-info-enabled=true`
    - Whether software info reporting is enabled.

* `..    stay-on-plugged-modes=ut`
    - The battery plugged in modes for which the device stays on. When using this setting, it is recommended to clear maximum_time_to_lock so that the device doesn&#39;t lock itself while it stays on.
    - Each invocation of this argument appends the given value to the array.
* `system-update    end-minutes=62`
    - If the type is WINDOWED, the end of the maintenance window, measured as the number of minutes after midnight in device&#39;s local time. This value must be between 0 and 1439, inclusive. If this value is less than start_minutes, then the maintenance window spans midnight. If the maintenance window specified is smaller than 30 minutes, the actual window is extended to 30 minutes beyond the start time.
* `start-minutes=93`
    - If the type is WINDOWED, the start of the maintenance window, measured as the number of minutes after midnight in the device&#39;s local time. This value must be between 0 and 1439, inclusive.
* `type=voluptua.`
    - The type of system update to configure.

* `..    tethering-config-disabled=false`
    - Whether configuring tethering and portable hotspots is disabled.
* `uninstall-apps-disabled=true`
    - Whether user uninstallation of applications is disabled.
* `unmute-microphone-disabled=false`
    - Whether the microphone is muted and adjusting microphone volume is disabled.
* `usb-file-transfer-disabled=true`
    - Whether transferring files over USB is disabled.
* `usb-mass-storage-enabled=true`
    - Whether USB storage is enabled.
* `version=sed`
    - The version of the policy. This is a read-only field. The version is incremented each time the policy is updated.
* `vpn-config-disabled=true`
    - Whether configuring VPN is disabled.
* `wifi-config-disabled=false`
    - Whether configuring Wi-Fi access points is disabled.
* `wifi-configs-lockdown-enabled=true`
    - Whether Wi-Fi networks defined in Open Network Configuration are locked so they can&#39;t be edited by the user.


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

* **-p update-mask=string**
    - The field mask indicating the fields to update. If not set, all modifiable fields will be modified.

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
