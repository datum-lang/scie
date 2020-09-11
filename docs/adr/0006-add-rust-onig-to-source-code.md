# 6. add rust-onig to source code

Date: 2020-09-11

## Status

2020-09-11 proposed

## Context

Onig use OnigRegSet as parameters, but it's private:

`rust
pub fn onig_regset_search(
     set: *mut OnigRegSet,
     str: *const OnigUChar,
     end: *const OnigUChar,
     start: *const OnigUChar,
     range: *const OnigUChar,
     lead: OnigRegSetLead,
     option: OnigOptionType,
     rmatch_pos: *mut ::std::os::raw::c_int,
 ) -> ::std::os::raw::c_int;
`


see in :

```
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OnigRegSetStruct {
    _unused: [u8; 0],
}
```


## Decision

Decision here...

## Consequences

Consequences here...
