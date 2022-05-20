const { spawnSync } = require('child_process');
const path = require('path');


function test_stdout(args, fixture, FORCE_COLOR = "3") {
	let { output } = spawnSync(path.normalize(`${__dirname}/../../target/release/cfonts`), args, { env: {FORCE_COLOR} });
	output = output.toString();
	if(output !== "," + fixture + ",") {
		return `OUTPUT:\n${require("util").inspect(output)}\nFIXTURE:\n${require("util").inspect(fixture)}`;
	} else {
		return 0;
	}
}

function run_tests(tests) {
	let fails = 0;
	tests.forEach(test => {
		let output = test_stdout(test.args, test.fixture, test.FORCE_COLOR);
		process.stdout.write(`• Running test ${`"\x1b[33m${test.name}\x1b[39m"`.padEnd(45, ' ')} `);
		if(output === 0) {
			process.stdout.write("\x1b[42m TEST SUCCESSFUL \x1b[49m\n");
		} else {
			fails ++;
			process.stdout.write(`\x1b[41m TEST FAILED \x1b[49m\n${output}\n`);
		}
	});

	if(fails > 0) {
		process.exit(1);
	}
}

const TESTS = [
	{
		name: "Default print",
		args: ["test"],
		fixture: '\n\n' +
			' ████████╗ ███████╗ ███████╗ ████████╗\n' +
			' ╚══██╔══╝ ██╔════╝ ██╔════╝ ╚══██╔══╝\n' +
			'    ██║    █████╗   ███████╗    ██║   \n' +
			'    ██║    ██╔══╝   ╚════██║    ██║   \n' +
			'    ██║    ███████╗ ███████║    ██║   \n' +
			'    ╚═╝    ╚══════╝ ╚══════╝    ╚═╝   \n' +
			'\n\n',
	},
	{
		name: "Print with single color",
		args: ["test", "-c" ,"red"],
		fixture: '\n\n' +
			' \x1B[31m████████\x1B[39m╗ \x1B[31m███████\x1B[39m╗ \x1B[31m███████\x1B[39m╗ \x1B[31m████████\x1B[39m╗\n' +
			' ╚══\x1B[31m██\x1B[39m╔══╝ \x1B[31m██\x1B[39m╔════╝ \x1B[31m██\x1B[39m╔════╝ ╚══\x1B[31m██\x1B[39m╔══╝\n' +
			' \x1B[31m   ██\x1B[39m║    \x1B[31m█████\x1B[39m╗   \x1B[31m███████\x1B[39m╗ \x1B[31m   ██\x1B[39m║   \n' +
			' \x1B[31m   ██\x1B[39m║    \x1B[31m██\x1B[39m╔══╝   ╚════\x1B[31m██\x1B[39m║ \x1B[31m   ██\x1B[39m║   \n' +
			' \x1B[31m   ██\x1B[39m║    \x1B[31m███████\x1B[39m╗ \x1B[31m███████\x1B[39m║ \x1B[31m   ██\x1B[39m║   \n' +
			'    ╚═╝    ╚══════╝ ╚══════╝    ╚═╝   \n' +
			'\n\n',
	},
	{
		name: "Print with two colors",
		args: ["test", "-c" ,"red,blue"],
		fixture: '\n\n' +
			' \x1B[31m████████\x1B[39m\x1B[34m╗\x1B[39m \x1B[31m███████\x1B[39m\x1B[34m╗\x1B[39m \x1B[31m███████\x1B[39m\x1B[34m╗\x1B[39m \x1B[31m████████\x1B[39m\x1B[34m╗\x1B[39m\n' +
			' \x1B[34m╚══\x1B[39m\x1B[31m██\x1B[39m\x1B[34m╔══╝\x1B[39m \x1B[31m██\x1B[39m\x1B[34m╔════╝\x1B[39m \x1B[31m██\x1B[39m\x1B[34m╔════╝\x1B[39m \x1B[34m╚══\x1B[39m\x1B[31m██\x1B[39m\x1B[34m╔══╝\x1B[39m\n' +
			' \x1B[31m   ██\x1B[39m\x1B[34m║   \x1B[39m \x1B[31m█████\x1B[39m\x1B[34m╗  \x1B[39m \x1B[31m███████\x1B[39m\x1B[34m╗\x1B[39m \x1B[31m   ██\x1B[39m\x1B[34m║   \x1B[39m\n' +
			' \x1B[31m   ██\x1B[39m\x1B[34m║   \x1B[39m \x1B[31m██\x1B[39m\x1B[34m╔══╝  \x1B[39m \x1B[34m╚════\x1B[39m\x1B[31m██\x1B[39m\x1B[34m║\x1B[39m \x1B[31m   ██\x1B[39m\x1B[34m║   \x1B[39m\n' +
			' \x1B[31m   ██\x1B[39m\x1B[34m║   \x1B[39m \x1B[31m███████\x1B[39m\x1B[34m╗\x1B[39m \x1B[31m███████\x1B[39m\x1B[34m║\x1B[39m \x1B[31m   ██\x1B[39m\x1B[34m║   \x1B[39m\n' +
			' \x1B[34m   ╚═╝   \x1B[39m \x1B[34m╚══════╝\x1B[39m \x1B[34m╚══════╝\x1B[39m \x1B[34m   ╚═╝   \x1B[39m\n' +
			'\n\n',
	},
	{
		name: "Print with two colors",
		args: ["test", "-c" ,"red,#ff8800"],
		FORCE_COLOR: "0",
		fixture: '\n\n' +
			' ████████╗ ███████╗ ███████╗ ████████╗\n' +
			' ╚══██╔══╝ ██╔════╝ ██╔════╝ ╚══██╔══╝\n' +
			'    ██║    █████╗   ███████╗    ██║   \n' +
			'    ██║    ██╔══╝   ╚════██║    ██║   \n' +
			'    ██║    ███████╗ ███████║    ██║   \n' +
			'    ╚═╝    ╚══════╝ ╚══════╝    ╚═╝   \n' +
			'\n\n',
	},
];

run_tests(TESTS);
