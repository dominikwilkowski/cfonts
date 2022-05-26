// yeah yeah I know... node in a rust repo... this was the fasted way I was able to spin this up, perhaps later I refactor this into a REAL language ;)
const { spawnSync } = require('child_process');
const path = require('path');

const { TESTS } = require('./tests.js');


function compare_implementations(args, fixture, FORCE_COLOR = "3", NO_COLOR = false) {
	const env = { ...process.env, FORCE_COLOR, ...(NO_COLOR !== false ? { NO_COLOR } : {}) };

	let { stdout: rust_output } = spawnSync(
		path.normalize(`${__dirname}/../../target/release/cfonts`),
		args,
		{
			encoding: 'utf-8',
			env,
		}
	);
	rust_output = rust_output.toString();

	args = args.map(arg => arg.includes('#') || arg.includes('|') ? `"${arg}"` : arg);
	let { stdout: node_output } = spawnSync(
		'node',
		[path.normalize(`${__dirname}/../../../nodejs/bin/index.js`), ...args],
		{
			shell: true,
			encoding: 'utf-8',
			env,
		}
	);
	node_output = node_output.toString();

	if(rust_output !== node_output || rust_output !== fixture) {
		let env_vars = [`FORCE_COLOR=${FORCE_COLOR}`];
		if( NO_COLOR ) {
			env_vars.push(`NO_COLOR=""`);
		}
		return `[${ env_vars.join(" ")} cfonts ${args.join(" ")}]\n` +
			`RUST:\n` +
			`${rust_output}\n` +
			`NODE:\n${node_output}\n` +
			`RUST OUTPUT:\n${require("util").inspect(rust_output)}\n` +
			`NODE OUTPUT:\n${require("util").inspect(node_output)}\n` +
			`FIXTURE:\n${require("util").inspect(fixture)}`;
	} else {
		return 0;
	}
}

function print_output(args, fixture, FORCE_COLOR = "3", NO_COLOR = false) {
	const env = { ...process.env, FORCE_COLOR, ...(NO_COLOR !== false ? { NO_COLOR } : {}) };

	let { stdout: rust_output } = spawnSync(
		path.normalize(`${__dirname}/../../target/release/cfonts`),
		args,
		{
			encoding: 'utf-8',
			env,
		}
	);
	rust_output = rust_output.toString();

	let env_vars = [`FORCE_COLOR=${FORCE_COLOR}`];
	if( NO_COLOR ) {
		env_vars.push(`NO_COLOR=""`);
	}

	console.log(`\x1b[30;43m\n ${ env_vars.join(" ")} cfonts ${args.join(" ")}\x1b[39;49m\n${rust_output}`);
}

function tests(tests) {
	let fails = 0;
	const failed = [];
	tests.forEach(test => {
		if( process.argv.includes('print') ) {
			print_output(test.args, test.fixture, test.FORCE_COLOR, test.NO_COLOR);
		} else {
			let output = compare_implementations(test.args, test.fixture, test.FORCE_COLOR, test.NO_COLOR);
			process.stdout.write(`• Running test ${`"\x1b[33m${test.name}\x1b[39m"`.padEnd(57, ' ')} `);
			if(output === 0) {
				process.stdout.write("\x1b[42m TEST SUCCESSFUL \x1b[49m\n");
			} else {
				fails ++;
				failed.push(test.name);
				process.stdout.write(`\x1b[41m TEST FAILED \x1b[49m\n${output}\n`);
			}
		}
	});

	if(fails > 0) {
		console.error(`\nErrors found in ${fails} test${fails.length > 1 ? 's' : ''}:`);
		failed.forEach(failed_test => {
			console.error(`    \x1b[33m${failed_test}\x1b[39m`);
		});
		process.exit(1);
	}
}

tests(TESTS);
