# Authentication Practices

This document describes authentication patterns and best practices for OAuth 2.0 implementation in CipherPowers.

## Overview

CipherPowers uses OAuth 2.0 for authentication, supporting multiple flows for different application types. This guide covers implementation patterns, security considerations, and common scenarios.

## OAuth Flows

### Authorization Code Flow with PKCE

**When to use:**
- Web applications with server-side components
- Single-page applications (SPAs)
- Mobile applications

**Implementation pattern:**

```javascript
// 1. Generate PKCE code verifier and challenge
const codeVerifier = generateCodeVerifier(); // 43-128 character random string
const codeChallenge = await generateCodeChallenge(codeVerifier);

// 2. Store code verifier securely
sessionStorage.set('pkce_verifier', codeVerifier);

// 3. Redirect to authorization endpoint
const authUrl = buildAuthorizationUrl({
  clientId: OAUTH_CLIENT_ID,
  redirectUri: OAUTH_REDIRECT_URI,
  scope: 'read write',
  state: generateState(),
  codeChallenge,
  codeChallengeMethod: 'S256'
});

// 4. Handle callback with authorization code
const tokenResponse = await exchangeCodeForToken({
  code: authorizationCode,
  codeVerifier: sessionStorage.get('pkce_verifier'),
  redirectUri: OAUTH_REDIRECT_URI
});
```

### Client Credentials Flow

**When to use:**
- Service-to-service authentication
- Background jobs
- Server-side processes with no user interaction

**Implementation pattern:**

```javascript
const tokenResponse = await getClientCredentialsToken({
  clientId: OAUTH_CLIENT_ID,
  clientSecret: OAUTH_CLIENT_SECRET,
  scope: 'service:read service:write'
});
```

## Token Management

### Token Storage

**Development environment:**
- Store in memory with session-based persistence
- Use secure session storage (not localStorage)

**Production environment:**
- Store in encrypted session storage
- Use environment-specific key management
- Consider Redis for distributed systems

**Never store tokens in:**
- Git repositories
- Browser localStorage (vulnerable to XSS)
- Logs or error messages
- Client-side cookies without HttpOnly flag

### Token Refresh

Implement automatic token refresh with buffer time:

```javascript
class TokenManager {
  constructor(expiryBuffer = 300) { // 5 minute buffer
    this.expiryBuffer = expiryBuffer;
    this.refreshMutex = new Mutex();
  }

  async getValidToken() {
    const token = await this.getStoredToken();

    if (this.isExpiringSoon(token)) {
      // Use mutex to prevent concurrent refresh attempts
      return await this.refreshMutex.runExclusive(async () => {
        // Check again inside mutex in case another call just refreshed
        const currentToken = await this.getStoredToken();
        if (this.isExpiringSoon(currentToken)) {
          return await this.refreshToken(currentToken);
        }
        return currentToken;
      });
    }

    return token;
  }

  isExpiringSoon(token) {
    const expiryTime = token.expiresAt;
    const currentTime = Date.now() / 1000;
    return (expiryTime - currentTime) < this.expiryBuffer;
  }
}
```

### Token Expiry Handling

```javascript
async function makeAuthenticatedRequest(url, options) {
  const token = await tokenManager.getValidToken();

  try {
    const response = await fetch(url, {
      ...options,
      headers: {
        ...options.headers,
        'Authorization': `Bearer ${token.accessToken}`
      }
    });

    if (response.status === 401) {
      // Token invalid, clear cache and re-authenticate
      await tokenManager.clearToken();
      throw new AuthenticationError('Token invalid, please re-authenticate');
    }

    return response;
  } catch (error) {
    // Handle network errors, etc.
    throw error;
  }
}
```

## Security Best Practices

### PKCE Implementation

Always use PKCE (Proof Key for Code Exchange) even for confidential clients:

```javascript
// Generate cryptographically secure random string
function generateCodeVerifier() {
  const array = new Uint8Array(32);
  crypto.getRandomValues(array);
  return base64UrlEncode(array);
}

// Generate SHA-256 hash of code verifier
async function generateCodeChallenge(verifier) {
  const encoder = new TextEncoder();
  const data = encoder.encode(verifier);
  const hash = await crypto.subtle.digest('SHA-256', data);
  return base64UrlEncode(new Uint8Array(hash));
}

// Base64-URL encoding (not standard Base64!)
function base64UrlEncode(buffer) {
  const base64 = btoa(String.fromCharCode(...buffer));
  return base64
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}
```

### State Parameter Validation

Prevent CSRF attacks by validating state parameter:

```javascript
// Before redirecting to OAuth provider
const state = generateSecureRandomString();
sessionStorage.set('oauth_state', state);

// In callback handler
const returnedState = params.get('state');
const storedState = sessionStorage.get('oauth_state');

if (returnedState !== storedState) {
  throw new Error('State mismatch - possible CSRF attack');
}

// Clear state after validation
sessionStorage.remove('oauth_state');
```

### Credential Management

**Environment variables:**
```bash
# .env.development
OAUTH_CLIENT_ID=dev-client-id
OAUTH_CLIENT_SECRET=dev-secret
OAUTH_REDIRECT_URI=http://localhost:8080/callback

# .env.production
OAUTH_CLIENT_ID=prod-client-id
OAUTH_CLIENT_SECRET=prod-secret
OAUTH_REDIRECT_URI=https://app.example.com/callback
```

**Never:**
- Commit secrets to git
- Share production credentials across environments
- Use same OAuth client for dev/staging/production
- Hardcode credentials in source code

## Error Handling

### Retry Strategy

Implement exponential backoff for token endpoint requests:

```javascript
async function requestTokenWithRetry(params, maxRetries = 3) {
  let attempt = 0;

  while (attempt < maxRetries) {
    try {
      return await requestToken(params);
    } catch (error) {
      if (error.status >= 500 && attempt < maxRetries - 1) {
        // Server error, retry with exponential backoff
        const delay = Math.pow(2, attempt) * 1000;
        await sleep(delay);
        attempt++;
      } else if (error.status === 429) {
        // Rate limited, respect Retry-After header
        const retryAfter = parseInt(error.headers['retry-after']) || 60;
        await sleep(retryAfter * 1000);
        attempt++;
      } else {
        // Client error or max retries reached
        throw error;
      }
    }
  }
}
```

### Common Error Responses

| Error Code | Meaning | Action |
|------------|---------|--------|
| `invalid_request` | Malformed request | Check request parameters |
| `invalid_client` | Client authentication failed | Verify client credentials |
| `invalid_grant` | Authorization code invalid/expired | Clear cached tokens, re-authenticate |
| `unauthorized_client` | Client not authorized for grant type | Check OAuth client configuration |
| `unsupported_grant_type` | Grant type not supported | Verify grant_type parameter |
| `invalid_scope` | Requested scope invalid | Check scope parameter |

## Testing

### Unit Tests

Mock OAuth provider responses:

```javascript
describe('TokenManager', () => {
  beforeEach(() => {
    mockOAuthProvider.reset();
  });

  it('refreshes token when expiring soon', async () => {
    mockOAuthProvider.respondWith({
      access_token: 'new-token',
      expires_in: 3600
    });

    const expiringSoonToken = {
      accessToken: 'old-token',
      expiresAt: Date.now() / 1000 + 200 // Expires in 200 seconds
    };

    await tokenManager.storeToken(expiringSoonToken);
    const token = await tokenManager.getValidToken();

    expect(token.accessToken).toBe('new-token');
  });
});
```

### Integration Tests

Test complete OAuth flow:

```javascript
describe('OAuth Flow', () => {
  it('completes authorization code flow with PKCE', async () => {
    // Start OAuth flow
    const { authUrl, codeVerifier } = await oauth.startAuthorizationFlow();

    // Simulate user authorization (use test OAuth server)
    const callbackUrl = await testOAuthServer.authorize(authUrl);

    // Exchange code for token
    const token = await oauth.handleCallback(callbackUrl, codeVerifier);

    expect(token.accessToken).toBeDefined();
    expect(token.expiresAt).toBeGreaterThan(Date.now() / 1000);
  });
});
```

## Troubleshooting

### Redirect URI Mismatch

**Symptom:** `redirect_uri_mismatch` error

**Solutions:**
1. Verify exact match between code and OAuth provider configuration
2. Check for trailing slashes: `http://localhost:8080/callback` vs `http://localhost:8080/callback/`
3. Verify protocol: `http` vs `https`
4. Check port numbers in development

### Token Refresh Race Conditions

**Symptom:** Multiple token refresh requests, occasional `invalid_grant` errors

**Solution:** Implement mutex/lock around token refresh:

```javascript
import { Mutex } from 'async-mutex';

class TokenManager {
  constructor() {
    this.refreshMutex = new Mutex();
  }

  async refreshToken(token) {
    return await this.refreshMutex.runExclusive(async () => {
      // Only one refresh at a time
      return await this._doRefreshToken(token);
    });
  }
}
```

### PKCE Code Challenge Failures

**Symptom:** `invalid_request` or `invalid_grant` during token exchange

**Common causes:**
1. Using standard Base64 encoding instead of Base64-URL encoding
2. Code verifier not stored between authorization and token requests
3. SHA-256 hash not computed correctly
4. Code verifier length outside 43-128 character range

### Time Synchronization Issues

**Symptom:** Token refresh happens too early or too late

**Solution:**
- Sync server clocks in containerized environments
- Use NTP in production
- Add configurable expiry buffer (default: 300 seconds)

## Monitoring

### Metrics to Track

- Token refresh success/failure rate
- Average token lifetime before refresh
- Authorization flow completion rate
- OAuth endpoint response times
- Rate limit hit frequency

### Logging Best Practices

**DO log:**
- OAuth flow steps (authorization, token exchange, refresh)
- Error codes and messages
- Last 4 characters of tokens (for debugging)
- Request/response status codes

**DO NOT log:**
- Complete tokens (access or refresh)
- Client secrets
- Authorization codes
- PKCE code verifiers
- User credentials

Example logging:

```javascript
logger.info('Token refreshed successfully', {
  tokenSuffix: token.slice(-4),
  expiresIn: token.expiresIn,
  scope: token.scope
});
```

## References

- [OAuth 2.0 RFC 6749](https://tools.ietf.org/html/rfc6749)
- [PKCE RFC 7636](https://tools.ietf.org/html/rfc7636)
- [OAuth 2.0 Security Best Practices](https://tools.ietf.org/html/draft-ietf-oauth-security-topics)

## Related Documentation

- [README.md - Authentication section](/Users/tobyhede/src/cipherpowers/README.md#authentication)
- [CLAUDE.md - Authentication Patterns](/Users/tobyhede/src/cipherpowers/CLAUDE.md#authentication-patterns)
- [OAuth Examples](/Users/tobyhede/src/cipherpowers/docs/examples/oauth/)
