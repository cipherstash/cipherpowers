"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const dispatcher_1 = require("./dispatcher");
async function main() {
    try {
        // Read stdin
        const chunks = [];
        for await (const chunk of process.stdin) {
            chunks.push(chunk);
        }
        const inputStr = Buffer.concat(chunks).toString('utf-8');
        // Parse input
        let input;
        try {
            input = JSON.parse(inputStr);
        }
        catch (error) {
            console.error(JSON.stringify({
                continue: false,
                message: 'Invalid JSON input'
            }));
            process.exit(1);
        }
        // Validate required fields
        if (!input.hook_event_name || !input.cwd) {
            // Graceful exit - missing required fields
            return;
        }
        // Dispatch
        const result = await (0, dispatcher_1.dispatch)(input);
        // Build output
        const output = {};
        if (result.context) {
            output.additionalContext = result.context;
        }
        if (result.blockReason) {
            output.decision = 'block';
            output.reason = result.blockReason;
        }
        if (result.stopMessage) {
            output.continue = false;
            output.message = result.stopMessage;
        }
        // Write output
        if (Object.keys(output).length > 0) {
            console.log(JSON.stringify(output));
        }
    }
    catch (error) {
        console.error(JSON.stringify({
            continue: false,
            message: `Unexpected error: ${error}`
        }));
        process.exit(1);
    }
}
main();
