'use strict';

const StripColor = (text) => {
	const pattern = [
		'[\\u001B\\u009B][[\\]()#;?]*(?:(?:(?:[a-zA-Z\\d]*(?:;[a-zA-Z\\d]*)*)?\\u0007)',
		'(?:(?:\\d{1,4}(?:;\\d{0,4})*)?[\\dA-PRZcf-ntqry=><~]))',
	].join('|');
	const ansi = new RegExp(pattern, 'g');

	if (typeof text === 'string') {
		return text.replace(ansi, '');
	} else {
		return text;
	}
};

module.exports = exports = { StripColor };
