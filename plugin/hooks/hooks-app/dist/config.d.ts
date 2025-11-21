import { GatesConfig } from './types';
/**
 * Validate config invariants to catch configuration errors early.
 * Throws descriptive errors when invariants are violated.
 */
export declare function validateConfig(config: GatesConfig): void;
export declare function loadConfig(cwd: string): Promise<GatesConfig | null>;
