import test from 'ava';

import UtfString from './utf-string';
//
// test('constructor', (t) => {
//   const utfString = new UtfString("hello, world");
//   t.is(utfString.utf8Length, 12);
// });

test('utf8 length', (t) => {
  const utfString = new UtfString("a💻bYX");
  console.log(utfString);
  t.is(utfString.utf8Length, 8);
});
