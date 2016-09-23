vm [![Build Status](https://travis-ci.org/nathanfaucett/rs-vm.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-vm)
=====

virtual machine

## Instructions

```bash
types: u8, u16, u32, u64, usize
       i8, i16, i32, i32, isize

halt, nop,

load_{type}
push_{type}
pop_{type}

add_{type} a, b, out
sub_{type} a, b, out
div_{type} a, b, out
mul_{type} a, b, out
rem_{type} a, b, out

and_{type} a, b, out
or_{type} a, b, out
xor_{type} a, b, out
neg_{type} a, out
not_{type} a, out
shl_{type} a, b, out
shr_{type} a, b, out

lt_{type} a, b, out
lte_{type} a, b, out
gt_{type} a, b, out
gte_{type} a, b, out
eq_{type} a, b, out
neq_{type} a, b, out

# if value is not 0 jumps to target's index
if_jmp value, target
jmp target
```
