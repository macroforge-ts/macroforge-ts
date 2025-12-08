// Re-export Result from @rydshift/mirror for use in generated code
// Using CommonJS for compatibility with the rest of the package
const mirror = require("@rydshift/mirror");
const Result = mirror.Result;

module.exports = { Result };
module.exports.Result = Result;
