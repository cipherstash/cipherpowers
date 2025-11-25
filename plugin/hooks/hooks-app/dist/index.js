"use strict";
// plugin/hooks/hooks-app/src/index.ts
Object.defineProperty(exports, "__esModule", { value: true });
exports.logger = exports.Session = exports.injectContext = exports.loadConfig = exports.handleAction = exports.executeGate = exports.dispatch = void 0;
// Existing exports
var dispatcher_1 = require("./dispatcher");
Object.defineProperty(exports, "dispatch", { enumerable: true, get: function () { return dispatcher_1.dispatch; } });
var gate_loader_1 = require("./gate-loader");
Object.defineProperty(exports, "executeGate", { enumerable: true, get: function () { return gate_loader_1.executeGate; } });
var action_handler_1 = require("./action-handler");
Object.defineProperty(exports, "handleAction", { enumerable: true, get: function () { return action_handler_1.handleAction; } });
var config_1 = require("./config");
Object.defineProperty(exports, "loadConfig", { enumerable: true, get: function () { return config_1.loadConfig; } });
var context_1 = require("./context");
Object.defineProperty(exports, "injectContext", { enumerable: true, get: function () { return context_1.injectContext; } });
// New session exports
var session_1 = require("./session");
Object.defineProperty(exports, "Session", { enumerable: true, get: function () { return session_1.Session; } });
// Logging exports
var logger_1 = require("./logger");
Object.defineProperty(exports, "logger", { enumerable: true, get: function () { return logger_1.logger; } });
