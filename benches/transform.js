const SOURCE = `
'use strict';

class Foo {
  foo() {}
}

class Bar extends Foo {
  foo() {
    super.foo();
  }
  async bar() {}
}

class Baz extends Bar {
  foo() {
    super.foo();
    this.baz()
  }
  baz() {

  }
  async other() {
    this.baz()
    await super.bar()
  }
}

/**
 * Extract red color out of a color integer:
 *
 * 0x00DEAD -> 0x00
 *
 * @param  {Number} color
 * @return {Number}
 */
function red( color )
{
    let foo = 3.14;
    return color >> 16;
}

/**
 * Extract green out of a color integer:
 *
 * 0x00DEAD -> 0xDE
 *
 * @param  {Number} color
 * @return {Number}
 */
function green( color )
{
    return ( color >> 8 ) & 0xFF;
}

/**
 * Extract blue color out of a color integer:
 *
 * 0x00DEAD -> 0xAD
 *
 * @param  {Number} color
 * @return {Number}
 */
function blue( color )
{
    return color & 0xFF;
}

/**
 * Converts an integer containing a color such as 0x00DEAD to a hex
 * string, such as '#00DEAD';
 *
 * @param  {Number} int
 * @return {String}
 */
function intToHex( int )
{
    const mask = '#000000';

    const hex = int.toString( 16 );

    return mask.substring( 0, 7 - hex.length ) + hex;
}

/**
 * Converts a hex string containing a color such as '#00DEAD' to
 * an integer, such as 0x00DEAD;
 *
 * @param  {Number} num
 * @return {String}
 */
function hexToInt( hex )
{
    return parseInt( hex.substring( 1 ), 16 );
}

module.exports = {
    red,
    green,
    blue,
    intToHex,
    hexToInt,
};
`;

const PARSERS = [
  ['swc', '../', (module) => module.transformSync(SOURCE, {})],
  ['swc-optimize', '../', (module) => module.transformSync(SOURCE, {})],
  ['babel', '@babel/core', (module) => module.transformSync(SOURCE, {
    presets: ["@babel/preset-env"],
    // This does less work than swc's InlineGlobals pass, but it's ok.
    // swc is faster than babel anyway.
    plugins: ["transform-node-env-inline"],
  })],
];

suite('transform', () => {
  PARSERS.map((args) => {
    const [name, requirePath, fn] = args;
    try {
      const func = fn.bind(null, require(requirePath));
      bench(name, func);
    } catch (e) {
      console.log(`Cannot load ${requirePath}: ${e.message}`);
    }
  });
});
