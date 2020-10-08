const fs = require('fs');
const vsctm = require('./vendor/main.js');
const oniguruma = require('oniguruma');

function readFile(path) {
    return new Promise((resolve, reject) => {
        fs.readFile(path, (error, data) => error ? reject(error) : resolve(data));
    })
}

let promise = readFile('./syntaxes/json/c.json').then(data => vsctm.parseRawGrammar(data.toString(), "c.json"));
promise.then((grammar) => {
    // console.log(grammar.patterns.length);
})


// Create a registry that can create a grammar from a scope name.
const registry = new vsctm.Registry({
    onigLib: Promise.resolve({
        createOnigScanner: (sources) => new oniguruma.OnigScanner(sources),
        createOnigString: (str) => new oniguruma.OnigString(str)
    }),
    loadGrammar: (scopeName) => {
        return readFile('./syntaxes/json/javascript.json').then(data => vsctm.parseRawGrammar(data.toString(), "c.json"))
    }
});

registry.loadGrammar('source.js').then(grammar => {
    var content = fs.readFileSync("../benchmark/samples/large.js.txt");
    var text = content.toString().split("\n");
    let ruleStack = vsctm.INITIAL;

    const start = Date.now();

    for (let i = 0; i < text.length; i++) {
        const line = text[i];
        const lineTokens = grammar.tokenizeLine(line, ruleStack);
        // for (let j = 0; j < lineTokens.tokens.length; j++) {
        //     const token = lineTokens.tokens[j];
        //     console.log(` - token from ${token.startIndex} to ${token.endIndex} ` +
        //         `(${line.substring(token.startIndex, token.endIndex)}) ` +
        //         `with scopes ${token.scopes.join(', ')}`
        //     );
        // }
        ruleStack = lineTokens.ruleStack;
    }

    const millis = Date.now() - start;
    console.log(`seconds elapsed = ${Math.floor(millis)}`);
});
//
// console.log("____________________________");
//
// let onigScanner = new oniguruma.OnigScanner(["\\G"]);
// let result = onigScanner.findNextMatchSync("\t$(CC) -o $@ $^ $(CFLAGS)\n", 0);
// console.log(result)
//
// let onigScanner2 = new oniguruma.OnigScanner(["\G"]);
// let result2 = onigScanner2.findNextMatchSync("\t$(CC) -o $@ $^ $(CFLAGS)\n", 0);
// console.log(result2)
