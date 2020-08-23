# Docs

## Refs

[WebIDE 的开发记录其五（monaco-editor + textmate）](https://www.ubug.io/blog/workpad-part-5)

> 主要就是因为 Textmate 语法解析依赖的 Oniguruma 是一个 C 语言下的解析功能，VSCode 可以使用 node 环境来调用原生的模块，但是在 web 环境下无法实现，即使通过 asm.js 转换后，性能依然会有 100-1000 倍的损失（16年9月的说明，目前未测试），而且 IE 不支持~~~

 - textmate 提供语言的语法配置，再加上一些 monaco-editor 的语言支持，生成 grammarProvider
 - textmate 注册 TextmateRegistry，然后挂载不同语言的 grammarProvider 保存解析配置之类的
 - 使用 onigasm 注册语法解析器 grammarRegistry，然后创建不同语言实际的解析器 TextmateTokenizer

