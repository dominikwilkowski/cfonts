const js = require('@eslint/js');
const globals = require('globals');

module.exports = [
	{
		files: ['**/*.js'],
		languageOptions: {
			ecmaVersion: 2018,
			sourceType: 'commonjs',
			globals: {
				...globals.node,
				...globals.commonjs,
				...globals.es6,
				Atomics: 'readonly',
				SharedArrayBuffer: 'readonly',
			},
		},
		rules: {
			...js.configs.recommended.rules,
			'no-async-promise-executor': 'off',
			'no-console': 'off',
			'no-unused-vars': [
				'error',
				{
					argsIgnorePattern: '_',
				},
			],
		},
	},
];
