Notes:

1. old reg not bee clear


## Demo 1

patterns_length_ptr = {*mut i32 | 0x7f907b1042f0} 0x00007f907b1042f0

[0] = {*mut u8 | 0x7f907b1042c0} 0x00007f907b1042c0
[1] = {*mut u8 | 0x7f907b1042d0} 0x00007f907b1042d0
[2] = {*mut u8 | 0x7f907b1042e0} 0x00007f907b1042e0

patterns_ptr = {*mut *mut u8 | 0x7f907b104300} 0x00007f907b104300

in C

lengths = {int * | 0x7f907b1042f0} 0x00007f907b1042f0
patterns = {unsigned char ** | 0x7f907b104300} 0x00007f907b104300

regexes = {OnigRegExp ** | 0x7f907b104320} 0x00007f907b104320
 *regexes = {OnigRegExp * | 0x0} NULL
 [1] = {OnigRegExp * | 0x0} NULL
 [2] = {OnigRegExp * | 0x0} NULL
 [3] = {OnigRegExp * | 0x0} NULL

regs = {regex_t ** | 0x7f907b104340} 0x00007f907b104340
 *regs = {regex_t * | 0x0} NULL
 [1] = {regex_t * | 0x0} NULL
 [2] = {regex_t * | 0x0} NULL
