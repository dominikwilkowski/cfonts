/**
 * What the package's own hosts need from a composition: the boundary builder behind it
 *
 * The key is a symbol the entries never export, so the builder stays the package's own
 * and a consumer's host sees the composition alone
 */
export const inner: unique symbol = Symbol("cfonts.inner");
