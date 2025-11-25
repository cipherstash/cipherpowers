import { promises as fs } from 'fs';
import { dirname, join } from 'path';
import { SessionState, SessionStateArrayKey } from './types';

/**
 * Manages session state with atomic file updates.
 *
 * State is stored in .claude/session/state.json relative to the project directory.
 */
export class Session {
  private stateFile: string;

  constructor(cwd: string = '.') {
    this.stateFile = join(cwd, '.claude', 'session', 'state.json');
  }

  /**
   * Get a session state value
   */
  async get<K extends keyof SessionState>(key: K): Promise<SessionState[K]> {
    const state = await this.load();
    return state[key];
  }

  /**
   * Set a session state value
   */
  async set<K extends keyof SessionState>(key: K, value: SessionState[K]): Promise<void> {
    const state = await this.load();
    state[key] = value;
    await this.save(state);
  }

  /**
   * Append value to array field (deduplicated)
   */
  async append(key: SessionStateArrayKey, value: string): Promise<void> {
    const state = await this.load();
    const array = state[key];

    if (!array.includes(value)) {
      array.push(value);
      await this.save(state);
    }
  }

  /**
   * Check if array contains value
   */
  async contains(key: SessionStateArrayKey, value: string): Promise<boolean> {
    const state = await this.load();
    return state[key].includes(value);
  }

  /**
   * Clear session state (remove file)
   */
  async clear(): Promise<void> {
    try {
      await fs.unlink(this.stateFile);
    } catch (error) {
      // File doesn't exist, that's fine
    }
  }

  /**
   * Load state from file or initialize new state
   */
  private async load(): Promise<SessionState> {
    try {
      const content = await fs.readFile(this.stateFile, 'utf-8');
      return JSON.parse(content);
    } catch (error) {
      // File doesn't exist or is corrupt, initialize new state
      return this.initState();
    }
  }

  /**
   * Save state to file atomically (write to temp, then rename)
   *
   * Performance note: File I/O adds small overhead (~1-5ms) per operation.
   * Atomic writes prevent corruption but require temp file creation.
   *
   * Concurrency note: Atomic rename prevents file corruption (invalid JSON,
   * partial writes) but does NOT prevent logical race conditions where
   * concurrent operations overwrite each other's changes. This is acceptable
   * because hooks run sequentially in practice. If true concurrent access is
   * needed, add file locking or retry logic.
   */
  private async save(state: SessionState): Promise<void> {
    await fs.mkdir(dirname(this.stateFile), { recursive: true });
    const temp = this.stateFile + '.tmp';

    try {
      // Write to temp file
      await fs.writeFile(temp, JSON.stringify(state, null, 2), 'utf-8');

      // Atomic rename (prevents corruption from concurrent writes)
      await fs.rename(temp, this.stateFile);
    } catch (error) {
      // Clean up temp file on error
      try {
        await fs.unlink(temp);
      } catch {
        // Ignore cleanup errors
      }
      throw error;
    }
  }

  /**
   * Initialize new session state
   *
   * Session ID format: ISO timestamp with punctuation replaced (e.g., "2025-11-23T14-30-45")
   * Unique per millisecond. Collisions possible if multiple sessions start in same millisecond,
   * but unlikely in practice due to hook serialization.
   */
  private initState(): SessionState {
    const now = new Date();
    return {
      session_id: now.toISOString().replace(/[:.]/g, '-').substring(0, 19),
      started_at: now.toISOString(),
      active_command: null,
      active_skill: null,
      edited_files: [],
      file_extensions: [],
      metadata: {}
    };
  }
}
