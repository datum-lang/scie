Code:

```makefile
hellomake: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)
```

## Scie Version

```json
{
    "content_name_scopes_list": {
        "scope": "meta.scope.prerequisites.makefile",
        "parent": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "Makefile",
                "parent": null
            }
        }
    },
    "rule_id": 36,
    "end_rule": null,
    "name_scopes_list": {
        "scope": "meta.scope.prerequisites.makefile",
        "parent": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "Makefile",
                "parent": null
            }
        }
    },
    "stringify": "",
    "begin_rule_captured_eol": false,
    "anchor_pos": 10,
    "depth": 3,
    "enter_pos": 10,
    "parent": {
        "content_name_scopes_list": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "Makefile",
                "parent": null
            }
        },
        "rule_id": 28,
        "end_rule": null,
        "name_scopes_list": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "Makefile",
                "parent": null
            }
        },
        "begin_rule_captured_eol": false,
        "anchor_pos": -1,
        "depth": 2,
        "enter_pos": 0,
        "parent": {
            "content_name_scopes_list": {
                "scope": "Makefile",
                "parent": null
            },
            "rule_id": 1,
            "end_rule": null,
            "name_scopes_list": {
                "scope": "Makefile",
                "parent": null
            },
            "begin_rule_captured_eol": false,
            "anchor_pos": -1,
            "depth": 1,
            "enter_pos": -1,
            "parent": null
        }
    }
}
```

VS Code Version

```json
{
    "nameScopesList": {
        "scope": "meta.scope.prerequisites.makefile",
        "parent": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "source.makefile",
                "parent": null,
                "metadata": 16793600
            },
            "metadata": 16793600
        },
        "metadata": 16793600
    },
    "_enterPos": -1,
    "endRule": null,
    "parent": {
        "nameScopesList": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "source.makefile",
                "parent": null,
                "metadata": 16793600
            },
            "metadata": 16793600
        },
        "_enterPos": -1,
        "endRule": null,
        "parent": {
            "nameScopesList": {
                "scope": "source.makefile",
                "parent": null,
                "metadata": 16793600
            },
            "_enterPos": -1,
            "endRule": null,
            "parent": null,
            "_anchorPos": -1,
            "beginRuleCapturedEOL": false,
            "ruleId": 1,
            "depth": 1,
            "contentNameScopesList": {
                "scope": "source.makefile",
                "parent": null,
                "metadata": 16793600
            }
        },
        "_anchorPos": -1,
        "beginRuleCapturedEOL": false,
        "ruleId": 28,
        "depth": 2,
        "contentNameScopesList": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "source.makefile",
                "parent": null,
                "metadata": 16793600
            },
            "metadata": 16793600
        }
    },
    "_anchorPos": -1,
    "beginRuleCapturedEOL": false,
    "ruleId": 36,
    "depth": 3,
    "contentNameScopesList": {
        "scope": "meta.scope.prerequisites.makefile",
        "parent": {
            "scope": "meta.scope.target.makefile",
            "parent": {
                "scope": "source.makefile",
                "parent": null,
                "metadata": 16793600
            },
            "metadata": 16793600
        },
        "metadata": 16793600
    }
}
```
