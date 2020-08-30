# Docs

## Refs

[WebIDE 的开发记录其五（monaco-editor + textmate）](https://www.ubug.io/blog/workpad-part-5)

> 主要就是因为 Textmate 语法解析依赖的 Oniguruma 是一个 C 语言下的解析功能，VSCode 可以使用 node 环境来调用原生的模块，但是在 web 环境下无法实现，即使通过 asm.js 转换后，性能依然会有 100-1000 倍的损失（16年9月的说明，目前未测试），而且 IE 不支持~~~

 - textmate 提供语言的语法配置，再加上一些 monaco-editor 的语言支持，生成 grammarProvider
 - textmate 注册 TextmateRegistry，然后挂载不同语言的 grammarProvider 保存解析配置之类的
 - 使用 onigasm 注册语法解析器 grammarRegistry，然后创建不同语言实际的解析器 TextmateTokenizer

## 解析顺序设计

 - Text
 - Json
 - makefile
 - xml
 - sql
 - javascript


```bash
ls -al scie-grammar/test-cases/_first-mate/fixtures | grep ^\- | sort -nrk 5
```

so sizes:

```
-rw-r--r--   1 fdhuang  staff  99897 Aug 22 15:17 php.json
-rw-r--r--   1 fdhuang  staff  58827 Aug 22 15:17 objective-c.json
-rw-r--r--   1 fdhuang  staff  45589 Aug 22 15:17 python.json
-rw-r--r--   1 fdhuang  staff  44635 Aug 22 15:17 ruby.json
-rw-r--r--   1 fdhuang  staff  41551 Aug 22 15:17 scss.json
-rw-r--r--   1 fdhuang  staff  26889 Aug 22 15:17 latex.json
-rw-r--r--   1 fdhuang  staff  22516 Aug 22 15:17 css.json
-rw-r--r--   1 fdhuang  staff  20923 Aug 23 11:20 c.json
-rw-r--r--   1 fdhuang  staff  19815 Aug 23 12:06 java.json
-rw-r--r--   1 fdhuang  staff  17065 Aug 22 15:17 javascript.json
-rw-r--r--   1 fdhuang  staff  16437 Aug 22 15:17 thrift.json
-rw-r--r--   1 fdhuang  staff  11314 Aug 22 15:17 sql.json
-rw-r--r--   1 fdhuang  staff  10975 Aug 22 15:17 coffee-script.json
-rw-r--r--   1 fdhuang  staff  10683 Aug 22 15:17 html.json
-rw-r--r--   1 fdhuang  staff   9552 Aug 22 15:17 makefile.json
-rw-r--r--   1 fdhuang  staff   8622 Aug 22 15:17 c-plus-plus.json
-rw-r--r--   1 fdhuang  staff   7560 Aug 22 15:17 ruby-on-rails.json
-rw-r--r--   1 fdhuang  staff   4396 Aug 22 15:17 python-regex.json
-rw-r--r--   1 fdhuang  staff   4100 Aug 22 15:17 json.json
-rw-r--r--   1 fdhuang  staff   3331 Aug 22 15:17 javascript-regex.json
-rw-r--r--   1 fdhuang  staff   3000 Aug 22 15:17 html-erb.json
-rw-r--r--   1 fdhuang  staff   2608 Aug 22 15:17 git-commit.json
-rw-r--r--   1 fdhuang  staff   1202 Aug 22 15:17 html-rails.json
-rw-r--r--   1 fdhuang  staff    750 Aug 27 23:21 text.json
-rw-r--r--   1 fdhuang  staff    690 Aug 22 15:17 apply-end-pattern-last.json
-rw-r--r--   1 fdhuang  staff    481 Aug 22 15:17 hyperlink.json
-rw-r--r--   1 fdhuang  staff    408 Aug 22 15:17 todo.json
-rw-r--r--   1 fdhuang  staff    361 Aug 22 15:17 multiline.json
-rw-r--r--   1 fdhuang  staff    284 Aug 22 15:17 hello.json
-rw-r--r--   1 fdhuang  staff    268 Aug 22 15:17 nested-captures.json
-rw-r--r--   1 fdhuang  staff    245 Aug 22 15:17 infinite-loop.json
-rw-r--r--   1 fdhuang  staff    233 Aug 22 15:17 content-name.json
-rw-r--r--   1 fdhuang  staff    223 Aug 22 15:17 imaginary.json
-rw-r--r--   1 fdhuang  staff    216 Aug 22 15:17 objective-c-plus-plus.json
-rw-r--r--   1 fdhuang  staff    206 Aug 22 15:17 forever.json
-rw-r--r--   1 fdhuang  staff    181 Aug 22 15:17 include-external-repository-rule.json
-rw-r--r--   1 fdhuang  staff    141 Aug 22 15:17 loops.json
```
