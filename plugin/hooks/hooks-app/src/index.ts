// plugin/hooks/hooks-app/src/index.ts

// Existing exports
export { dispatch } from './dispatcher';
export { executeGate } from './gate-loader';
export { handleAction } from './action-handler';
export { loadConfig } from './config';
export { injectContext } from './context';

export type {
  HookInput,
  GateResult,
  GateExecute,
  GateConfig,
  HookConfig,
  GatesConfig
} from './types';

// New session exports
export { Session } from './session';
export type {
  SessionState,
  SessionStateArrayKey,
  SessionStateScalarKey
} from './types';
