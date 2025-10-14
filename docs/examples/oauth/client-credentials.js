/**
 * OAuth 2.0 Client Credentials Flow Example
 *
 * This example demonstrates the client credentials flow for
 * service-to-service authentication without user interaction.
 *
 * Use case: Background services, API integrations, cron jobs
 */

const axios = require('axios');

// Configuration from environment variables
const config = {
  clientId: process.env.OAUTH_CLIENT_ID,
  clientSecret: process.env.OAUTH_CLIENT_SECRET,
  tokenEndpoint: process.env.OAUTH_TOKEN_ENDPOINT,
  scope: 'service:read service:write'
};

/**
 * Simple in-memory token cache
 */
class TokenCache {
  constructor() {
    this.token = null;
  }

  set(token) {
    this.token = {
      ...token,
      cachedAt: Date.now()
    };
  }

  get() {
    if (!this.token) {
      return null;
    }

    // Check if token is expired
    const now = Math.floor(Date.now() / 1000);
    const expiryBuffer = 300; // 5 minute buffer

    if (this.token.expiresAt - now < expiryBuffer) {
      // Token expired or expiring soon
      this.token = null;
      return null;
    }

    return this.token;
  }

  clear() {
    this.token = null;
  }
}

const tokenCache = new TokenCache();

/**
 * Request access token using client credentials
 * @returns {Promise<Object>} Token response
 */
async function getClientCredentialsToken() {
  const params = new URLSearchParams({
    grant_type: 'client_credentials',
    client_id: config.clientId,
    client_secret: config.clientSecret,
    scope: config.scope
  });

  try {
    console.log('Requesting client credentials token...');

    const response = await axios.post(config.tokenEndpoint, params, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    });

    const token = {
      accessToken: response.data.access_token,
      expiresIn: response.data.expires_in,
      expiresAt: Math.floor(Date.now() / 1000) + response.data.expires_in,
      scope: response.data.scope,
      tokenType: response.data.token_type
    };

    console.log('Token obtained successfully', {
      tokenSuffix: token.accessToken.slice(-4),
      expiresIn: token.expiresIn,
      scope: token.scope
    });

    return token;
  } catch (error) {
    console.error('Token request failed:', {
      status: error.response?.status,
      error: error.response?.data?.error,
      description: error.response?.data?.error_description
    });
    throw error;
  }
}

/**
 * Get valid access token (from cache or request new one)
 * @returns {Promise<string>} Valid access token
 */
async function getValidAccessToken() {
  // Check cache first
  const cachedToken = tokenCache.get();
  if (cachedToken) {
    console.log('Using cached token', {
      tokenSuffix: cachedToken.accessToken.slice(-4),
      expiresIn: cachedToken.expiresAt - Math.floor(Date.now() / 1000)
    });
    return cachedToken.accessToken;
  }

  // Request new token
  const token = await getClientCredentialsToken();
  tokenCache.set(token);

  return token.accessToken;
}

/**
 * Make authenticated API request
 * @param {string} url - API endpoint URL
 * @param {Object} options - Request options
 * @returns {Promise<Object>} API response
 */
async function makeAuthenticatedRequest(url, options = {}) {
  const accessToken = await getValidAccessToken();

  try {
    const response = await axios({
      url,
      ...options,
      headers: {
        ...options.headers,
        'Authorization': `Bearer ${accessToken}`
      }
    });

    return response.data;
  } catch (error) {
    // Handle 401 by clearing cache and retrying once
    if (error.response?.status === 401) {
      console.log('Token invalid, clearing cache and retrying...');
      tokenCache.clear();

      // Retry with new token
      const newAccessToken = await getValidAccessToken();
      const retryResponse = await axios({
        url,
        ...options,
        headers: {
          ...options.headers,
          'Authorization': `Bearer ${newAccessToken}`
        }
      });

      return retryResponse.data;
    }

    throw error;
  }
}

/**
 * Retry with exponential backoff
 * @param {Function} fn - Async function to retry
 * @param {number} maxRetries - Maximum retry attempts
 * @returns {Promise<*>} Function result
 */
async function retryWithBackoff(fn, maxRetries = 3) {
  let attempt = 0;

  while (attempt < maxRetries) {
    try {
      return await fn();
    } catch (error) {
      attempt++;

      // Don't retry client errors (except 429 rate limit)
      if (error.response?.status < 500 && error.response?.status !== 429) {
        throw error;
      }

      if (attempt >= maxRetries) {
        throw error;
      }

      // Calculate backoff delay
      let delay;
      if (error.response?.status === 429) {
        // Respect Retry-After header for rate limits
        delay = (parseInt(error.response.headers['retry-after']) || 60) * 1000;
      } else {
        // Exponential backoff for server errors
        delay = Math.pow(2, attempt - 1) * 1000;
      }

      console.log(`Request failed, retrying in ${delay}ms (attempt ${attempt}/${maxRetries})...`);
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }
}

/**
 * Example: Make authenticated API calls with automatic token management
 */
async function main() {
  try {
    console.log('Client Credentials Flow Example\n');

    // Example API endpoint (replace with your actual API)
    const apiEndpoint = 'https://api.example.com/v1/resources';

    // Make first request (will obtain token)
    console.log('\n1. Making first API request (will obtain new token)...');
    const result1 = await retryWithBackoff(() =>
      makeAuthenticatedRequest(apiEndpoint, { method: 'GET' })
    );
    console.log('Request successful:', result1);

    // Make second request (will use cached token)
    console.log('\n2. Making second API request (will use cached token)...');
    const result2 = await retryWithBackoff(() =>
      makeAuthenticatedRequest(apiEndpoint, { method: 'GET' })
    );
    console.log('Request successful:', result2);

    // Simulate token expiry by clearing cache
    console.log('\n3. Clearing cache to simulate expiry...');
    tokenCache.clear();

    // Make third request (will obtain new token)
    console.log('\n4. Making third API request (will obtain new token)...');
    const result3 = await retryWithBackoff(() =>
      makeAuthenticatedRequest(apiEndpoint, { method: 'GET' })
    );
    console.log('Request successful:', result3);

    console.log('\nExample completed successfully!');
  } catch (error) {
    console.error('\nExample failed:', error.message);
    process.exit(1);
  }
}

// Run example if executed directly
if (require.main === module) {
  // Validate configuration
  if (!config.clientId || !config.clientSecret || !config.tokenEndpoint) {
    console.error('Error: Missing required environment variables');
    console.error('Required: OAUTH_CLIENT_ID, OAUTH_CLIENT_SECRET, OAUTH_TOKEN_ENDPOINT');
    process.exit(1);
  }

  main();
}

// Export for use as module
module.exports = {
  getClientCredentialsToken,
  getValidAccessToken,
  makeAuthenticatedRequest,
  retryWithBackoff,
  tokenCache
};
