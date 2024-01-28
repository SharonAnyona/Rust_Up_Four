#rust syntax

// You can't use a variable in rust without initializing it first
// We use let to declare a variable in rust
// All statements in rust end with a semi-colon
// In rust a variable is immutable by default (mut keyword is used)
// we use " _ " to allow an unused variable be declared without giving a warning.
// We use " .. " to represent a umber taht we do not care about e.g let(x,..)=(1,2);
//signed integer can be both positive and negative
// unsigned integer is always positive
         signed unsigned
         i8      u8
         i16     u16
         i32     u32

// in 32 bit processor architecture the size of word is 4bytes meaning the processor can access 4bytes at a time  and in a 64bit processor it can access 8bytes at a time(64bits at a time) 
// usize gives you the guarantee to be always big enough to hold any pointer or any offset in a data structure