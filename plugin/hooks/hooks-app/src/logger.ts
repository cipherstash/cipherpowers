// plugin/hooks/hooks-app/src/logger.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { tmpdir } from 'os';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LogEntry {
  ts: string;
  level: LogLevel;
  event?: string;
  message?: string;
  [key: string]: unknown;
}

const LOG_LEVELS: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

/**
 * Get the log directory path.
 * Uses ${TMPDIR}/cipherpowers/ for isolation.
 */
function getLogDir(): string {
  return path.join(tmpdir(), 'cipherpowers');
}

/**
 * Get the log file path for today.
 * Format: hooks-YYYY-MM-DD.log
 */
function getLogFilePath(): string {
  const date = new Date().toISOString().split('T')[0];
  return path.join(getLogDir(), `hooks-${date}.log`);
}

/**
 * Check if logging is enabled via environment variable.
 * CIPHERPOWERS_LOG=1 enables logging.
 */
function isLoggingEnabled(): boolean {
  return process.env.CIPHERPOWERS_LOG === '1';
}

/**
 * Get the minimum log level from environment.
 * CIPHERPOWERS_LOG_LEVEL=debug|info|warn|error (default: info)
 */
function getMinLogLevel(): LogLevel {
  const level = process.env.CIPHERPOWERS_LOG_LEVEL as LogLevel;
  if (level && LOG_LEVELS[level] !== undefined) {
    return level;
  }
  return 'info';
}

/**
 * Check if a log level should be written based on minimum level.
 */
function shouldLog(level: LogLevel): boolean {
  return LOG_LEVELS[level] >= LOG_LEVELS[getMinLogLevel()];
}

/**
 * Ensure the log directory exists.
 */
async function ensureLogDir(): Promise<void> {
  const dir = getLogDir();
  await fs.mkdir(dir, { recursive: true });
}

/**
 * Write a log entry to the log file.
 * Each entry is a JSON line for easy parsing with jq.
 */
async function writeLog(entry: LogEntry): Promise<void> {
  if (!isLoggingEnabled()) return;
  if (!shouldLog(entry.level)) return;

  try {
    await ensureLogDir();
    const line = JSON.stringify(entry) + '\n';
    await fs.appendFile(getLogFilePath(), line, 'utf-8');
  } catch {
    // Silently fail - logging should never break the hook
  }
}

/**
 * Write a log entry unconditionally (bypasses CIPHERPOWERS_LOG check).
 * Used for startup/diagnostic logging to verify hooks are being invoked.
 */
async function writeLogAlways(entry: LogEntry): Promise<void> {
  try {
    await ensureLogDir();
    const line = JSON.stringify(entry) + '\n';
    await fs.appendFile(getLogFilePath(), line, 'utf-8');
  } catch {
    // Silently fail - logging should never break the hook
  }
}

/**
 * Create a log entry with timestamp.
 */
function createEntry(
  level: LogLevel,
  message: string,
  data?: Record<string, unknown>
): LogEntry {
  return {
    ts: new Date().toISOString(),
    level,
    message,
    ...data,
  };
}

/**
 * Logger interface for hooks-app.
 *
 * Enable logging: CIPHERPOWERS_LOG=1
 * Set level: CIPHERPOWERS_LOG_LEVEL=debug|info|warn|error
 *
 * Logs are written to: ${TMPDIR}/cipherpowers/hooks-YYYY-MM-DD.log
 * Format: JSON lines (one JSON object per line)
 *
 * Example:
 *   {"ts":"2025-11-25T10:30:00.000Z","level":"info","event":"PostToolUse","tool":"Edit"}
 */
export const logger = {
  debug: (message: string, data?: Record<string, unknown>) =>
    writeLog(createEntry('debug', message, data)),

  info: (message: string, data?: Record<string, unknown>) =>
    writeLog(createEntry('info', message, data)),

  warn: (message: string, data?: Record<string, unknown>) =>
    writeLog(createEntry('warn', message, data)),

  error: (message: string, data?: Record<string, unknown>) =>
    writeLog(createEntry('error', message, data)),

  /**
   * Log unconditionally (bypasses CIPHERPOWERS_LOG check).
   * Used for startup/diagnostic logging to verify hooks are invoked.
   */
  always: (message: string, data?: Record<string, unknown>) =>
    writeLogAlways(createEntry('info', message, data)),

  /**
   * Log a hook event with structured data.
   * Convenience method for common hook logging pattern.
   */
  event: (
    level: LogLevel,
    event: string,
    data?: Record<string, unknown>
  ) =>
    writeLog({
      ts: new Date().toISOString(),
      level,
      event,
      ...data,
    }),

  /**
   * Get the current log file path (for mise tasks).
   */
  getLogFilePath,

  /**
   * Get the log directory path (for mise tasks).
   */
  getLogDir,
};
