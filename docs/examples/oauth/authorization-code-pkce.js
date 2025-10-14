/**
 * OAuth 2.0 Authorization Code Flow with PKCE Example
 *
 * This example demonstrates a complete OAuth authorization code flow
 * with PKCE (Proof Key for Code Exchange) for enhanced security.
 *
 * Use case: Web applications and SPAs requiring user authentication
 */

const crypto = require('crypto');
const express = require('express');
const axios = require('axios');

// Configuration from environment variables
const config = {
  clientId: process.env.OAUTH_CLIENT_ID,
  clientSecret: process.env.OAUTH_CLIENT_SECRET,
  redirectUri: process.env.OAUTH_REDIRECT_URI,
  authorizationEndpoint: process.env.OAUTH_AUTHORIZATION_ENDPOINT,
  tokenEndpoint: process.env.OAUTH_TOKEN_ENDPOINT,
  scope: 'read write'
};

// In-memory session storage (use Redis in production)
const sessions = new Map();

/**
 * Generate cryptographically secure random string for code verifier
 * @returns {string} Code verifier (43-128 characters)
 */
function generateCodeVerifier() {
  return crypto.randomBytes(32)
    .toString('base64')
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}

/**
 * Generate code challenge from code verifier using SHA-256
 * @param {string} verifier - Code verifier
 * @returns {string} Base64-URL encoded SHA-256 hash
 */
function generateCodeChallenge(verifier) {
  return crypto.createHash('sha256')
    .update(verifier)
    .digest('base64')
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}

/**
 * Generate secure random state parameter for CSRF protection
 * @returns {string} State parameter
 */
function generateState() {
  return crypto.randomBytes(16).toString('hex');
}

/**
 * Build authorization URL with PKCE parameters
 * @param {string} codeChallenge - PKCE code challenge
 * @param {string} state - CSRF state parameter
 * @returns {string} Complete authorization URL
 */
function buildAuthorizationUrl(codeChallenge, state) {
  const params = new URLSearchParams({
    response_type: 'code',
    client_id: config.clientId,
    redirect_uri: config.redirectUri,
    scope: config.scope,
    state: state,
    code_challenge: codeChallenge,
    code_challenge_method: 'S256'
  });

  return `${config.authorizationEndpoint}?${params.toString()}`;
}

/**
 * Exchange authorization code for access token
 * @param {string} code - Authorization code from callback
 * @param {string} codeVerifier - Original code verifier
 * @returns {Promise<Object>} Token response
 */
async function exchangeCodeForToken(code, codeVerifier) {
  const params = new URLSearchParams({
    grant_type: 'authorization_code',
    code: code,
    redirect_uri: config.redirectUri,
    client_id: config.clientId,
    client_secret: config.clientSecret,
    code_verifier: codeVerifier
  });

  try {
    const response = await axios.post(config.tokenEndpoint, params, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    });

    return {
      accessToken: response.data.access_token,
      refreshToken: response.data.refresh_token,
      expiresIn: response.data.expires_in,
      expiresAt: Math.floor(Date.now() / 1000) + response.data.expires_in,
      scope: response.data.scope
    };
  } catch (error) {
    console.error('Token exchange failed:', error.response?.data || error.message);
    throw error;
  }
}

// Express application for handling OAuth flow
const app = express();

/**
 * Route: Initiate OAuth flow
 *
 * 1. Generate PKCE code verifier and challenge
 * 2. Generate state parameter
 * 3. Store verifier and state in session
 * 4. Redirect to authorization endpoint
 */
app.get('/login', (req, res) => {
  const codeVerifier = generateCodeVerifier();
  const codeChallenge = generateCodeChallenge(codeVerifier);
  const state = generateState();

  // Store PKCE verifier and state (use secure session in production)
  const sessionId = crypto.randomBytes(16).toString('hex');
  sessions.set(sessionId, {
    codeVerifier,
    state,
    createdAt: Date.now()
  });

  // Store session ID in cookie
  res.cookie('session_id', sessionId, {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'lax',
    maxAge: 600000 // 10 minutes
  });

  // Redirect to authorization endpoint
  const authUrl = buildAuthorizationUrl(codeChallenge, state);
  res.redirect(authUrl);
});

/**
 * Route: OAuth callback
 *
 * 1. Validate state parameter (CSRF protection)
 * 2. Retrieve code verifier from session
 * 3. Exchange authorization code for tokens
 * 4. Store tokens securely
 * 5. Redirect to application
 */
app.get('/callback', async (req, res) => {
  const { code, state, error } = req.query;
  const sessionId = req.cookies?.session_id;

  // Handle authorization errors
  if (error) {
    return res.status(400).send(`Authorization failed: ${error}`);
  }

  // Retrieve session
  const session = sessions.get(sessionId);
  if (!session) {
    return res.status(400).send('Session not found or expired');
  }

  // Validate state parameter (CSRF protection)
  if (state !== session.state) {
    return res.status(400).send('State mismatch - possible CSRF attack');
  }

  // Clean up session state
  sessions.delete(sessionId);

  try {
    // Exchange authorization code for tokens
    const tokens = await exchangeCodeForToken(code, session.codeVerifier);

    // Store tokens securely (use encrypted storage in production)
    // For this example, we'll just display them
    res.send(`
      <h1>Authentication Successful!</h1>
      <p>Access token received (last 4 chars): ...${tokens.accessToken.slice(-4)}</p>
      <p>Expires in: ${tokens.expiresIn} seconds</p>
      <p>Scope: ${tokens.scope}</p>
      <p><strong>Note:</strong> In production, store tokens securely and never display them.</p>
    `);

    // Log success (never log full tokens!)
    console.log('Authentication successful', {
      tokenSuffix: tokens.accessToken.slice(-4),
      expiresIn: tokens.expiresIn,
      scope: tokens.scope
    });
  } catch (error) {
    res.status(500).send('Token exchange failed');
  }
});

/**
 * Route: Home page
 */
app.get('/', (req, res) => {
  res.send(`
    <h1>OAuth 2.0 Authorization Code Flow with PKCE</h1>
    <p>This example demonstrates secure OAuth authentication with PKCE.</p>
    <p><a href="/login">Login with OAuth</a></p>
  `);
});

// Start server
const PORT = process.env.PORT || 8080;
app.listen(PORT, () => {
  console.log(`OAuth example server running on http://localhost:${PORT}`);
  console.log(`Redirect URI: ${config.redirectUri}`);
  console.log('Visit http://localhost:' + PORT + ' to start');
});

// Session cleanup (remove expired sessions every 5 minutes)
setInterval(() => {
  const now = Date.now();
  const expiryTime = 600000; // 10 minutes

  for (const [sessionId, session] of sessions.entries()) {
    if (now - session.createdAt > expiryTime) {
      sessions.delete(sessionId);
    }
  }
}, 300000);
