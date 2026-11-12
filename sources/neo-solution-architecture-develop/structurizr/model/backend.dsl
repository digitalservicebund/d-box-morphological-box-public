backend = group "Infrastructure" {
    auth = softwaresystem "BundID IdP"

    postbox = group "Post Box Component" {
        matrix_multi = softwaresystem "Matrix Backend" "Matrix service" {
            citizens = container "Synapse (Citizens)" {
                c2s = component "C2S API"
                s2s = component "S2S API"
            }

            citizens_mas = container "MAS (Citizens)" "Matrix Authentication Service"

            citizens_psql = database "PostgreSQL (Citizens)"

            authorities = container "Synapse (Authorities)" {
                c2s = component "C2S API"
                s2s = component "S2S API"
            }

            authorities_mas = container "MAS (Authorities)" "Matrix Authentication Service"

            authorities_psql = database "PostgreSQL (Authorities)"

            citizens -> citizens_mas "Auth delegation" "OIDC"
            citizens -> citizens_psql
            citizens_mas -> citizens_psql
            authorities -> authorities_mas
            authorities -> authorities_psql
            authorities_mas -> authorities_psql
        }

        citizens_bundid_proxy = softwaresystem "Keycloak" "Proxy IdP to translate Matrix-OIDC to BundID-SAML" {
            bundid_extension = container "keycloak-extension-bundid" "Keycloak-Extension to connect the BundID-IdentityProvider" {
                technology "SAML"
            }
        }

        citizens_bundid_proxy -> citizens_bundid_proxy "Verify BundID authentication response" "SAML"
        citizens_bundid_proxy -> citizens_bundid_proxy "Create account with bPK2 as ID (identity federation)"


    }
}
