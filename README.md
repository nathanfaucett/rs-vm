vm [![Build Status](https://travis-ci.org/nathanfaucett/rs-vm.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-vm)
=====

virtual machine

## Instructions

```bash
types: u8, u16, u32, u64
       i8, i16, i32, i32

halt, nop,

copy_{type} # duplicates top of stack
push_{type} type, value, # type_int, type_ptr, type_ptr_off, type_idr_ptr, type_idr_ptr_off,
pop_{type}

load_{type} # loads value from top of stack and puts it on top of stack
save_{type} # pops address then pops value and puts it in it

# takes top two values from stack
add_{type}
sub_{type}
div_{type}
mul_{type}
rem_{type}

# takes top two values from stack
and_{type}
or_{type}
xor_{type}
shl_{type}
shr_{type}

# takes top value from stack
neg_{type}
not_{type}

# takes top two values from stack
lt_{type}
lte_{type}
gt_{type}
gte_{type}
eq_{type}
neq_{type}

# if value is not 0 jumps to target's value
if_jmp value, target
jmp target
```
