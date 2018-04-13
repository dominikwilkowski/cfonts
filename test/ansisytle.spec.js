/***************************************************************************************************************************************************************
 *
 * AnsiSytle unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js')
const AnsiSytle = CFonts.__test__.AnsiSytle;


test(`AnsiSytle - Has a bunch of keys`, () => {
	expect( Object.keys( AnsiSytle ).length ).toEqual( 42 );
});

test(`AnsiSytle - Includes the system key`, () => {
	expect( Object.keys( AnsiSytle ).includes('system') ).toEqual( true );
});

test(`AnsiSytle - The system key contains open and close`, () => {
	expect( Object.keys( AnsiSytle['system'] ) ).toEqual( [ 'open', 'close' ] );
});
