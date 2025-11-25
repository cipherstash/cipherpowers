import { GatesConfig } from './types';
/**
 * Validate config invariants to catch configuration errors early.
 * Throws descriptive errors when invariants are violated.
 */
export declare function validateConfig(config: GatesConfig): void;
/**
 * Load and merge project and plugin configs.
 *
 * Priority:
 * 1. Project: .claude/gates.json (highest)
 * 2. Project: gates.json
 * 3. Plugin: ${CLAUDE_PLUGIN_ROOT}/hooks/gates.json (fallback/defaults)
 *
 * Configs are MERGED - project overrides plugin for same keys.
 */
export declare function loadConfig(cwd: string): Promise<GatesConfig | null>;
