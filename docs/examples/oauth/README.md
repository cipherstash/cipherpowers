# OAuth 2.0 Examples

This directory contains practical examples of OAuth 2.0 implementation in CipherPowers.

## Examples

### Authorization Code Flow with PKCE

**File:** `authorization-code-pkce.js`

Complete example of authorization code flow with PKCE for web applications and SPAs.

**Key features:**
- PKCE code verifier and challenge generation
- State parameter for CSRF protection
- Secure token storage
- Automatic token refresh

**Use case:** User-facing applications requiring delegated access

### Client Credentials Flow

**File:** `client-credentials.js`

Example of client credentials flow for service-to-service authentication.

**Key features:**
- Direct token request
- Token caching
- Automatic retry on failure

**Use case:** Background services, API integrations, cron jobs

### Token Management

**File:** `token-manager-example.js`

Complete token manager implementation with refresh logic and mutex-based concurrency control.

**Key features:**
- Expiry-based refresh with buffer
- Race condition prevention
- Error handling and retry

## Usage

Each example is self-contained and can be used as a starting point:

```bash
# Install dependencies
npm install

# Run authorization code flow example
node docs/examples/oauth/authorization-code-pkce.js

# Run client credentials flow example
node docs/examples/oauth/client-credentials.js
```

## Configuration

All examples require environment variables:

```bash
export OAUTH_CLIENT_ID="your-client-id"
export OAUTH_CLIENT_SECRET="your-client-secret"
export OAUTH_REDIRECT_URI="http://localhost:8080/callback"
export OAUTH_TOKEN_ENDPOINT="https://auth.example.com/token"
export OAUTH_AUTHORIZATION_ENDPOINT="https://auth.example.com/authorize"
```

## Related Documentation

- [Authentication Practices](/Users/tobyhede/src/cipherpowers/docs/practices/authentication.md)
- [README.md - Authentication](/Users/tobyhede/src/cipherpowers/README.md#authentication)
- [CLAUDE.md - Authentication Patterns](/Users/tobyhede/src/cipherpowers/CLAUDE.md#authentication-patterns)
