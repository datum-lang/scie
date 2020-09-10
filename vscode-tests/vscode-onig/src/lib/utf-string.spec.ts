import test from 'ava';

import UtfString from './utf-string';

test('constructor', (t) => {
  const utfString = new UtfString("hello, world");
  t.log(utfString);
  t.is(utfString.utf8Length, 12);
});
