// --------------------
// C4 extra: Dynamic Diagrams https://c4model.com/diagrams/dynamic
// --------------------

// Flow of a citizen logging into a client using the BundID password authentication method
dynamic matrix_multi dynamic_account_creation "BundID Password flow" {
    description "TODO: how to log in?"
    autolayout lr
    properties {
        mermaid.sequenceDiagram true
    }

    1: citizen -> frontend_ios "Start Login"
    2: frontend_ios -> matrix_multi.citizens_mas "OIDC-native login flow" "Matrix C2S API"
    3: matrix_multi.citizens_mas -> citizens_bundid_proxy "Forward" "OIDC"
    4: citizens_bundid_proxy -> auth "Forward" "SAML"
    5: auth -> citizen "Request credentials"
    6: citizen -> auth "Provide credentials" "HTTPS"
    7: auth -> citizens_bundid_proxy "Return authentication response" "SAML"
    8: citizens_bundid_proxy -> citizens_bundid_proxy "Verify BundID authentication response" "SAML"
    9: citizens_bundid_proxy -> citizens_bundid_proxy "Create account with bPK2 as ID (identity federation)"
    10: citizens_bundid_proxy -> matrix_multi.citizens_mas "Return authentication response" "OIDC"
    11: matrix_multi.citizens_mas -> matrix_multi.citizens_mas "Verify Keycloak authentication response" "OIDC"
    12: matrix_multi.citizens_mas -> matrix_multi.citizens_mas "Create Matrix account with bPK2 as localpart"
    13: matrix_multi.citizens_mas -> frontend_ios "Grant access_token" "Matrix C2S API"

}
