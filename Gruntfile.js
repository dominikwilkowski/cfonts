/*
 * grunt-font
 * https://github.com/dominikwilkowski/grunt-font
 *
 * Copyright (c) 2015 Dominik Wilkowski
 * Licensed under the MIT license.
 */

'use strict';

module.exports = function(grunt) {

	//------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Dependencies
	//------------------------------------------------------------------------------------------------------------------------------------------------------------
	grunt.loadNpmTasks('grunt-contrib-jshint');
	grunt.loadNpmTasks('grunt-exec');

	grunt.initConfig({

		//----------------------------------------------------------------------------------------------------------------------------------------------------------
		// JSHINT
		//----------------------------------------------------------------------------------------------------------------------------------------------------------
		jshint: {
			options: {
				jshintrc: '.jshintrc',
			},

			all: [
				'!node_modules/**/*',
				'*.js',
			],
		},


		//----------------------------------------------------------------------------------------------------------------------------------------------------------
		// TEST
		//----------------------------------------------------------------------------------------------------------------------------------------------------------
		exec: {
			test01: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü"',
			test02: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -c red,magenta -m 15',

			test03: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "simple"',
			test04: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "simple" -c yellow -m 20',

			test05: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "3d"',
			test06: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "3d" -c magenta,yellow -m 7',

			test07: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "console"',
			test08: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "console" -c blue -b white',

			test09: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "simple3d"',
			test10: 'cfonts -t "a b c defghijklmnopqrstuvwxyz|0123456789|\!?.+-_=@#$%&()/:;ü" -f "simple3d" -c cyan -b white',
		},

	});


	//------------------------------------------------------------------------------------------------------------------------------------------------------------
	// TASKS
	//------------------------------------------------------------------------------------------------------------------------------------------------------------
	grunt.registerTask('default', [/*'jshint', */'exec']);

};