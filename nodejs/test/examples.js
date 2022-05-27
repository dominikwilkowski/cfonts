/***************************************************************************************************************************************************************
 *
 * cfonts, Sexy fonts for the console. (CLI output)
 *
 * Testing the API side of things.
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPL-3.0-or-later
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 **************************************************************************************************************************************************************/

'use strict';

const CFonts = require('../lib/index.js');
const path = require('path');
const fs = require('fs');

const prettyFont = CFonts.render('H', {
	font: 'console',
	colors: ['red'],
});

console.log(prettyFont.string);
console.log(prettyFont.array);
console.log(prettyFont.lines);
console.log(prettyFont.options);

CFonts.say('Hello world!', {
	font: 'block',
	align: 'left',
	colors: ['white'],
	background: 'transparent',
	letterSpacing: 1,
	lineHeight: 1,
	space: true,
	maxLength: '0',
	gradient: false,
	independentGradient: false,
});

CFonts.say('Hello world!', {
	background: 'GREEN',
	backgroundColor: 'RED',
});

CFonts.say('Hello|world!', {
	font: 'console',
	align: 'center',
	colors: ['red'],
});

CFonts.say(
	`Hello
world\nagain|!`,
	{
		font: 'simple',
		align: 'center',
		colors: ['red'],
	}
);

CFonts.say('Hi|world!', {
	gradient: ['red', 'green'],
});

CFonts.say('Hi|world!', {
	gradient: ['red', 'green'],
	independentGradient: true,
});

CFonts.say('Hi|world!', {
	gradient: ['red', 'green'],
	transitionGradient: true,
});

CFonts.say('Hi|world!', {
	gradient: ['red', 'green'],
	independentGradient: true,
	transitionGradient: true,
});

CFonts.say('KeystoneJS', {
	font: 'chrome',
	colors: ['cyan', 'yellow', '#ffa500'],
	letterSpacing: 1,
	lineHeight: 1,
	space: true,
	maxLength: '0',
});

const prettyFont2 = CFonts.render('New|Line|with a really long last line to see how page wrapping works', {
	font: 'slick',
	gradient: ['cyan', 'red'],
	background: 'black',
	space: false,
	maxLength: 100,
	env: 'browser',
});

const prettyFont3 = CFonts.render('New|Line|with a really long last line to see how page wrapping works', {
	font: 'grid',
	colors: ['cyan', 'gray'],
	background: 'black',
	align: 'center',
	env: 'browser',
});

const prettyFont4 = CFonts.render('New|Line|with a really long last line to see how page wrapping works', {
	font: 'pallet',
	gradient: ['red', 'red'],
	maxLength: 50,
	env: 'browser',
});

const prettyFont5 = CFonts.render('New|Line|with a really long last line to see how page wrapping works', {
	font: 'console',
	colors: ['white'],
	background: 'cyan',
	align: 'right',
	env: 'browser',
});

const html = `
<!doctype html>
<html lang="en">
<head>
	<meta charset="utf-8">
	<meta name="viewport" content="width=device-width">
	<title>CFonts browser test</title>
</head>
<body style="background:#f1f1f1">
<h1>CFonts browser test</h1>
<h2>Font one</h2>
${prettyFont2.string}
<h2>Font two</h2>
${prettyFont3.string}
<h2>Font three</h2>
${prettyFont4.string}
<h2>Font four</h2>
${prettyFont5.string}
</body>
</html>
`;

const check = fs.writeFileSync(path.normalize(`${__dirname}/test.html`), html, { encoding: 'utf8' });

// for issue #13
Array.prototype.foo = () => {
	return 0;
};
CFonts.say('Hello', { colors: ['green'] });
