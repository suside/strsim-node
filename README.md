# strsim-node

NodeJS bindings for [strsim-rs](https://github.com/dguo/strsim-rs), a Rust implementation of string similarity metrics.

# limitations

Current version is build for musl libc / alpine linux only.

# examples

```js
const assert = require('assert');
const {
  damerau_levenshtein,
  jaro_winkler,
  jaro,
  levenshtein,
  normalized_damerau_levenshtein,
  normalized_levenshtein,
  osa_distance,
  sorensen_dice,
} = require('strsim-node');

assert(levenshtein('kitten', 'sitting') === 3);

assert(Math.abs(normalized_levenshtein('kitten', 'sitting') - 0.571) < 0.001);

assert(osa_distance('ac', 'cba') === 3);

assert(damerau_levenshtein('ac', 'cba') === 2);

assert(Math.abs(normalized_damerau_levenshtein('levenshtein', 'löwenbräu') - 0.272) < 0.001);

assert(Math.abs(jaro('Friedrich Nietzsche', 'Jean-Paul Sartre') - 0.392) < 0.001);

assert(Math.abs(jaro_winkler('cheeseburger', 'cheese fries') - 0.911) < 0.001);

assert(sorensen_dice('web applications', 'applications of the web') === 0.7878787878787878);
```