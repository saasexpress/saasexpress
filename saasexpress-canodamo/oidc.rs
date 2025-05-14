// OIDC Core Structs
// These structs represent the most common fields in the OpenID Connect model

/// Represents a Bearer Access Token as defined in OAuth 2.0 (RFC 6750)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    /// The actual token string
    pub access_token: String,
    /// Type of token, typically "Bearer"
    pub token_type: String,
    /// Duration in seconds that the token is valid
    pub expires_in: Option<u64>,
    /// Optional refresh token
    pub refresh_token: Option<String>,
    /// Space-delimited scope values
    pub scope: Option<String>,
}

/// Represents an ID Token, which is a JWT containing claims about the authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdToken {
    /// Issuer identifier - URL of the identity provider
    pub iss: String,
    /// Subject identifier - unique identifier for the user
    pub sub: String,
    /// Audience(s) - client ID or array of client IDs
    #[serde(flatten)]
    pub aud: Audience,
    /// Expiration time as UNIX timestamp
    pub exp: u64,
    /// Time when token was issued as UNIX timestamp
    pub iat: u64,
    /// Time when authentication occurred as UNIX timestamp
    pub auth_time: Option<u64>,
    /// Nonce value used to associate a client session with the ID token
    pub nonce: Option<String>,
    /// Authentication Context Class Reference
    pub acr: Option<String>,
    /// Authentication Methods References
    pub amr: Option<Vec<String>>,
    /// Authorized party that requested the token
    pub azp: Option<String>,

    // Standard claims that may be included in the ID token
    /// User's full name
    pub name: Option<String>,
    /// User's given/first name
    pub given_name: Option<String>,
    /// User's family/last name
    pub family_name: Option<String>,
    /// User's middle name
    pub middle_name: Option<String>,
    /// User's preferred nickname
    pub nickname: Option<String>,
    /// User's preferred username
    pub preferred_username: Option<String>,
    /// URL of user's profile page
    pub profile: Option<String>,
    /// URL of user's picture
    pub picture: Option<String>,
    /// URL of user's website
    pub website: Option<String>,
    /// User's email address
    pub email: Option<String>,
    /// Whether email has been verified
    pub email_verified: Option<bool>,
    /// User's gender
    pub gender: Option<String>,
    /// User's birthdate
    pub birthdate: Option<String>,
    /// User's time zone
    pub zoneinfo: Option<String>,
    /// User's locale
    pub locale: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// Whether phone number has been verified
    pub phone_number_verified: Option<bool>,
    /// User's address information
    pub address: Option<Address>,
    /// Time when user info was last updated
    pub updated_at: Option<u64>,
}

/// Helper enum to handle the `aud` field which can be a string or array of strings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Audience {
    Single(String),
    Multiple(Vec<String>),
}

/// Address Claim - represents a physical mailing address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Full mailing address, formatted for display
    pub formatted: Option<String>,
    /// Street address component
    pub street_address: Option<String>,
    /// City or locality component
    pub locality: Option<String>,
    /// State, province, prefecture, or region component
    pub region: Option<String>,
    /// Zip code or postal code component
    pub postal_code: Option<String>,
    /// Country name component
    pub country: Option<String>,
}

/// Represents the OIDC Authorization Request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// Response type, e.g., "code", "token", "id_token", etc.
    pub response_type: String,
    /// Client identifier
    pub client_id: String,
    /// Redirection URI to which the response will be sent
    pub redirect_uri: String,
    /// Opaque value used to maintain state between the request and callback
    pub state: Option<String>,
    /// Space-delimited scope values (MUST include "openid")
    pub scope: String,
    /// Value used to associate a client session with an ID token
    pub nonce: Option<String>,
    /// Display type hint: "page", "popup", "touch", or "wap"
    pub display: Option<String>,
    /// Prompt options: "none", "login", "consent", "select_account"
    pub prompt: Option<String>,
    /// Maximum allowable elapsed time in seconds
    pub max_age: Option<u64>,
    /// End-user's preferred languages and scripts for UI
    pub ui_locales: Option<String>,
    /// ID token hint previously issued by the authorization server
    pub id_token_hint: Option<String>,
    /// Hint to the authorization server about the login identifier
    pub login_hint: Option<String>,
    /// Authentication Context Class Reference values
    pub acr_values: Option<String>,
}

/// Represents the UserInfo Response containing claims about the authenticated user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    /// Subject identifier - unique identifier for the user
    pub sub: String,
    /// User's full name
    pub name: Option<String>,
    /// User's given/first name
    pub given_name: Option<String>,
    /// User's family/last name
    pub family_name: Option<String>,
    /// User's middle name
    pub middle_name: Option<String>,
    /// User's preferred nickname
    pub nickname: Option<String>,
    /// User's preferred username
    pub preferred_username: Option<String>,
    /// URL of user's profile page
    pub profile: Option<String>,
    /// URL of user's picture
    pub picture: Option<String>,
    /// URL of user's website
    pub website: Option<String>,
    /// User's email address
    pub email: Option<String>,
    /// Whether email has been verified
    pub email_verified: Option<bool>,
    /// User's gender
    pub gender: Option<String>,
    /// User's birthdate
    pub birthdate: Option<String>,
    /// User's time zone
    pub zoneinfo: Option<String>,
    /// User's locale
    pub locale: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// Whether phone number has been verified
    pub phone_number_verified: Option<bool>,
    /// User's address information
    pub address: Option<Address>,
    /// Time when user info was last updated
    pub updated_at: Option<u64>,
}

/// Represents an OIDC Token Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// OAuth 2.0 Access Token
    pub access_token: String,
    /// Token type, typically "Bearer"
    pub token_type: String,
    /// Duration in seconds the access token is valid
    pub expires_in: Option<u64>,
    /// Refresh token to obtain new access tokens
    pub refresh_token: Option<String>,
    /// OpenID Connect ID Token as a JWT
    pub id_token: String,
    /// Space-delimited scope values
    pub scope: Option<String>,
}

/// Represents OIDC Provider Configuration Information (from .well-known/openid-configuration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// URL using the https scheme with no query or fragment component
    pub issuer: String,
    /// URL of the authorization endpoint
    pub authorization_endpoint: String,
    /// URL of the token endpoint
    pub token_endpoint: String,
    /// URL of the UserInfo endpoint
    pub userinfo_endpoint: Option<String>,
    /// URL of the JSON Web Key Set document
    pub jwks_uri: String,
    /// URL of the registration endpoint
    pub registration_endpoint: Option<String>,
    /// Supported scope values
    pub scopes_supported: Option<Vec<String>>,
    /// Supported response types
    pub response_types_supported: Vec<String>,
    /// Supported response modes
    pub response_modes_supported: Option<Vec<String>>,
    /// Supported grant types
    pub grant_types_supported: Option<Vec<String>>,
    /// Supported Authentication Context Class References
    pub acr_values_supported: Option<Vec<String>>,
    /// Supported subject identifier types
    pub subject_types_supported: Vec<String>,
    /// Supported JWS signing algorithms for the ID Token
    pub id_token_signing_alg_values_supported: Vec<String>,
    /// Supported JWE encryption algorithms for the ID Token
    pub id_token_encryption_alg_values_supported: Option<Vec<String>>,
    /// Supported encryption encodings for the ID Token
    pub id_token_encryption_enc_values_supported: Option<Vec<String>>,
    /// Supported JWS signing algorithms for UserInfo endpoint
    pub userinfo_signing_alg_values_supported: Option<Vec<String>>,
    /// Supported JWE encryption algorithms for UserInfo endpoint
    pub userinfo_encryption_alg_values_supported: Option<Vec<String>>,
    /// Supported encryption encodings for UserInfo endpoint
    pub userinfo_encryption_enc_values_supported: Option<Vec<String>>,
    /// Supported client authentication methods for token endpoint
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    /// Supported JWS signing algorithms for JWT client authentication
    pub token_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    /// Supported display parameter values
    pub display_values_supported: Option<Vec<String>>,
    /// Supported claim types
    pub claim_types_supported: Option<Vec<String>>,
    /// Supported claims
    pub claims_supported: Option<Vec<String>>,
    /// URL of service documentation
    pub service_documentation: Option<String>,
    /// Languages and scripts supported for UI
    pub ui_locales_supported: Option<Vec<String>>,
    /// URL that the OpenID Provider provides to the end-user for managing the session
    pub end_session_endpoint: Option<String>,
    /// Supported prompt parameter values
    pub prompt_values_supported: Option<Vec<String>>,
}

/// Represents an OAuth 2.0 error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub error: String,
    /// Human-readable error description
    pub error_description: Option<String>,
    /// URI identifying a human-readable web page about the error
    pub error_uri: Option<String>,
    /// State parameter from the request
    pub state: Option<String>,
}
