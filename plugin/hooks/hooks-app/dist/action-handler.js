"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.handleAction = handleAction;
async function handleAction(action, gateResult, _config, _input) {
    switch (action) {
        case 'CONTINUE':
            return {
                continue: true,
                context: gateResult.additionalContext
            };
        case 'BLOCK':
            return {
                continue: false,
                blockReason: gateResult.reason || 'Gate failed'
            };
        case 'STOP':
            return {
                continue: false,
                stopMessage: gateResult.message || 'Gate stopped execution'
            };
        default:
            // Gate chaining - action is another gate name
            return {
                continue: true,
                context: gateResult.additionalContext,
                chainedGate: action
            };
    }
}
