const oniguruma = require('oniguruma');

let onigScanner = new oniguruma.OnigScanner(["^", "\\\n", "%|\\*", "(^[ \t]+)?(?=#)", "(\\$?\\$)[@%<?^+*]", "\\$?\\$\\("]);
let r = onigScanner.findNextMatchSync("%.o: %.c $(DEPS)", 4);
console.log(r);
