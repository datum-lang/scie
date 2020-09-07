# C issues

## Lost begin_captures

```bash
meta.preprocessor.c.include
```

### Correct

```json
"beginCaptures": [
  null,
  {
    "id": 86,
    "_name": "keyword.control.import.include.c",
    "_nameIsCapturing": false,
    "_contentName": null,
    "_contentNameIsCapturing": false,
    "retokenizeCapturedWithRuleId": 0,
    "_type": "CaptureRule"
  }
],
```

### Error

```json
"begin_captures": [],
```