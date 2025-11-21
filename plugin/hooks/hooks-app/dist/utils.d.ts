/**
 * Check if a file exists at the given path.
 * Used by config and context modules to probe file system.
 */
export declare function fileExists(filePath: string): Promise<boolean>;
