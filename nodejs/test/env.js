// we force color detection in tests to make them reproducible in different environments like ci
process.env.FORCE_COLOR = 3;

const { Options } = require('../src/Options.js');
Options.reset();
