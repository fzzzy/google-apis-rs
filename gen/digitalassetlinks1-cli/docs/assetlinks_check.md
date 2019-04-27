Determines whether the specified (directional) relationship exists between
the specified source and target assets.

The relation describes the intent of the link between the two assets as
claimed by the source asset.  An example for such relationships is the
delegation of privileges or permissions.

This command is most often used by infrastructure systems to check
preconditions for an action.  For example, a client may want to know if it
is OK to send a web URL to a particular mobile app instead. The client can
check for the relevant asset link from the website to the mobile app to
decide if the operation should be allowed.

A note about security: if you specify a secure asset as the source, such as
an HTTPS website or an Android app, the API will ensure that any
statements used to generate the response have been made in a secure way by
the owner of that asset.  Conversely, if the source asset is an insecure
HTTP website (that is, the URL starts with `http://` instead of `https://`),
the API cannot verify its statements securely, and it is not possible to
ensure that the website&#39;s statements have not been altered by a third
party.  For more information, see the [Digital Asset Links technical design
specification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).

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

* **-p target-android-app-certificate-sha256-fingerprint=string**
    - The uppercase SHA-265 fingerprint of the certificate.  From the PEM
         certificate, it can be acquired like this:
        
            $ keytool -printcert -file $CERTFILE | grep SHA256:
            SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \
                42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
        
        or like this:
        
            $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256
            SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \
                16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
        
        In this example, the contents of this field would be `14:6D:E9:83:C5:73:
        06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:
        44:E5`.
        
        If these tools are not available to you, you can convert the PEM
        certificate into the DER format, compute the SHA-256 hash of that string
        and represent the result as a hexstring (that is, uppercase hexadecimal
        representations of each octet, separated by colons).

* **-p target-android-app-package-name=string**
    - Android App assets are naturally identified by their Java package name.
        For example, the Google Maps app uses the package name
        `com.google.android.apps.maps`.
        REQUIRED

* **-p source-web-site=string**
    - Web assets are identified by a URL that contains only the scheme, hostname
        and port parts.  The format is
        
            http[s]://&lt;hostname&gt;[:&lt;port&gt;]
        
        Hostnames must be fully qualified: they must end in a single period
        (&#34;`.`&#34;).
        
        Only the schemes &#34;http&#34; and &#34;https&#34; are currently allowed.
        
        Port numbers are given as a decimal number, and they must be omitted if the
        standard port numbers are used: 80 for http and 443 for https.
        
        We call this limited URL the &#34;site&#34;.  All URLs that share the same scheme,
        hostname and port are considered to be a part of the site and thus belong
        to the web asset.
        
        Example: the asset with the site `https://www.google.com` contains all
        these URLs:
        
          *   `https://www.google.com/`
          *   `https://www.google.com:443/`
          *   `https://www.google.com/foo`
          *   `https://www.google.com/foo?bar`
          *   `https://www.google.com/foo#bar`
          *   `https://user@password:www.google.com/`
        
        But it does not contain these URLs:
        
          *   `http://www.google.com/`       (wrong scheme)
          *   `https://google.com/`          (hostname does not match)
          *   `https://www.google.com:444/`  (port does not match)
        REQUIRED

* **-p source-android-app-package-name=string**
    - Android App assets are naturally identified by their Java package name.
        For example, the Google Maps app uses the package name
        `com.google.android.apps.maps`.
        REQUIRED

* **-p target-web-site=string**
    - Web assets are identified by a URL that contains only the scheme, hostname
        and port parts.  The format is
        
            http[s]://&lt;hostname&gt;[:&lt;port&gt;]
        
        Hostnames must be fully qualified: they must end in a single period
        (&#34;`.`&#34;).
        
        Only the schemes &#34;http&#34; and &#34;https&#34; are currently allowed.
        
        Port numbers are given as a decimal number, and they must be omitted if the
        standard port numbers are used: 80 for http and 443 for https.
        
        We call this limited URL the &#34;site&#34;.  All URLs that share the same scheme,
        hostname and port are considered to be a part of the site and thus belong
        to the web asset.
        
        Example: the asset with the site `https://www.google.com` contains all
        these URLs:
        
          *   `https://www.google.com/`
          *   `https://www.google.com:443/`
          *   `https://www.google.com/foo`
          *   `https://www.google.com/foo?bar`
          *   `https://www.google.com/foo#bar`
          *   `https://user@password:www.google.com/`
        
        But it does not contain these URLs:
        
          *   `http://www.google.com/`       (wrong scheme)
          *   `https://google.com/`          (hostname does not match)
          *   `https://www.google.com:444/`  (port does not match)
        REQUIRED

* **-p source-android-app-certificate-sha256-fingerprint=string**
    - The uppercase SHA-265 fingerprint of the certificate.  From the PEM
         certificate, it can be acquired like this:
        
            $ keytool -printcert -file $CERTFILE | grep SHA256:
            SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \
                42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
        
        or like this:
        
            $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256
            SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \
                16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5
        
        In this example, the contents of this field would be `14:6D:E9:83:C5:73:
        06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:
        44:E5`.
        
        If these tools are not available to you, you can convert the PEM
        certificate into the DER format, compute the SHA-256 hash of that string
        and represent the result as a hexstring (that is, uppercase hexadecimal
        representations of each octet, separated by colons).

* **-p relation=string**
    - Query string for the relation.
        
        We identify relations with strings of the format `&lt;kind&gt;/&lt;detail&gt;`, where
        `&lt;kind&gt;` must be one of a set of pre-defined purpose categories, and
        `&lt;detail&gt;` is a free-form lowercase alphanumeric string that describes the
        specific use case of the statement.
        
        Refer to [our API documentation](/digital-asset-links/v1/relation-strings)
        for the current list of supported relations.
        
        For a query to match an asset link, both the query&#39;s and the asset link&#39;s
        relation strings must match exactly.
        
        Example: A query with relation `delegate_permission/common.handle_all_urls`
        matches an asset link with relation
        `delegate_permission/common.handle_all_urls`.

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
