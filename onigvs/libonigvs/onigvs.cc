/*---------------------------------------------------------
 * Copyright (C) Microsoft Corporation. All rights reserved.
 *--------------------------------------------------------*/

#include <cstdlib>
#include <cstdio>
#include <cstring>
#include "oniguruma/src/oniguruma.h"


typedef struct OnigRegExp_ {
    unsigned char* strData;
    int strLength;
    regex_t* regex;
    OnigRegion* region;
    bool hasGAnchor;
    int lastSearchStrCacheId;
    int lastSearchPosition;
    bool lastSearchMatched;
} OnigRegExp;

typedef struct OnigScanner_ {
    OnigRegSet* rset;
    OnigRegExp** regexes;
    int count;
} OnigScanner;

int lastOnigStatus = 0;
OnigErrorInfo lastOnigErrorInfo;


char* getLastOnigError()
{
    static char s[ONIG_MAX_ERROR_MESSAGE_LEN];
    onig_error_code_to_str((UChar*)s, lastOnigStatus, &lastOnigErrorInfo);
    return s;
}

#define MAX_REGIONS 1000

int encodeOnigRegion(OnigRegion *result, int index) {
    static int encodedResult[2 * (1 + MAX_REGIONS)];
    int i;
    if (result == NULL || result->num_regs > MAX_REGIONS) {
        return 0;
    }

    encodedResult[0] = index;
    encodedResult[1] = result->num_regs;
    for (i = 0; i < result->num_regs; i++) {
        encodedResult[2 * i + 2] = result->beg[i];
        encodedResult[2 * i + 3] = result->end[i];
    }
    return (long)encodedResult;
}
