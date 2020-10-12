# 10. align extensions

Date: 2020-10-12

## Status

2020-10-12 proposed

## Context

Some extensions not align:

```
error path: "/Users/fdhuang/charj/scie/extensions/perl/syntaxes/perl.tmLanguage.json", err: Error("invalid type: integer `1`, expected a boolean", line: 38, column: 27)
```

### replace "applyEndPatternLast": 1,

"applyEndPatternLast": 1, => "applyEndPatternLast": true,

### xml.tmLanguage.json not correnct issues

related: https://github.com/atom/language-xml/issues/96

solution: replace from xml.bundle

https://github.com/textmate/xml.tmbundle

### R issues

```
{
    "end": "\\)",
    "endCaptures": {
        "0": {
            "name": "punctuation.section.parens.end.r"
        }
    }
},
```

## Decision

Decision here...

## Consequences

Consequences here...
