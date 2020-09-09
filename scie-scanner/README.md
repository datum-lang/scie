## vscode-oniguruma parameters

create options:

 - ONIG_OPTION_CAPTURE_GROUP
 - ONIG_ENCODING_UTF8
 - ONIG_SYNTAX_DEFAULT

```cpp
OnigRegExp* createOnigRegExp(unsigned char* data, int length) {
  OnigRegExp* result;
  regex_t* regex;

  lastOnigStatus = onig_new(&regex, data, data + length,
                            ONIG_OPTION_CAPTURE_GROUP, ONIG_ENCODING_UTF8,
                            ONIG_SYNTAX_DEFAULT, &lastOnigErrorInfo);

  if (lastOnigStatus != ONIG_NORMAL) {
    return NULL;
  }
```

search opions

 - ONIG_REGSET_POSITION_LEAD
 - ONIG_OPTION_NONE

```cpp
    bestResultIndex = onig_regset_search(scanner->rset, strData, strData + strLength, strData + position, strData + strLength,
                                         ONIG_REGSET_POSITION_LEAD, ONIG_OPTION_NONE, &bestLocation);

```