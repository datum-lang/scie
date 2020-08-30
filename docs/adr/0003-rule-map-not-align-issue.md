# 3. rule map not align issue

Date: 2020-08-29

## Status

2020-08-29 proposed

## Context

In currently, the [VSCode-textmate](https://github.com/microsoft/vscode-textmate) generated ruleid2rule has some duplicated fields.

```json
  {
    "id": 15,
    "_name": "punctuation.definition.variable.makefile",
    "_nameIsCapturing": false,
    "_contentName": null,
    "_contentNameIsCapturing": false,
    "retokenize_captured_with_rule_id": 0,
    "_type": "CaptureRule"
  },
  {
    "id": 16,
    "_name": "punctuation.definition.variable.makefile",
    "_nameIsCapturing": false,
    "_contentName": null,
    "_contentNameIsCapturing": false,
    "retokenize_captured_with_rule_id": 0,
    "_type": "CaptureRule"
  }
```

But in our case with Rust, if we don't merge those field, we will had issues. So we decide to merge it will name.

The issues is `stackoverflow`, ::laughing::::laughing::::laughing::

```
thread 'grammar::grammar::tests::should_build_makefile_grammar' has overflowed its stack
fatal runtime error: stack overflow
error: test failed, to rerun pass '-p scie-grammar --lib'
```


## Decision

Decision here...

## Consequences

Consequences here...
