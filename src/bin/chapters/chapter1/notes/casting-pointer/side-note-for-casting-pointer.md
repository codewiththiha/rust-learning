1. Why you can't cast f64 directly to *const i64
In Rust, you cannot cast a value directly into a pointer.

x is a value (stored in a memory slot).

&x is a Reference. In Rust, references are "smart"—the compiler guarantees they are never null and always point to valid memory.

*const i64 is a Raw Pointer. This is the "dumb" C-style pointer.

The "Two-Step" Logic:

&x: You first get a Reference to the memory location of x.

as *const f64: You convert that "Safe Reference" into an "Unsafe Raw Pointer" of the same type.

as *const i64: Once it is a raw pointer, Rust allows you to cast it to any other raw pointer type (just like C).

You cannot jump from f64 (the data) to *const i64 (an address) because the data itself is not a memory address.

2. What is *const vs *mut? (Raw Pointers)
In Rust, the syntax for a raw pointer is *const T or *mut T.

In C, you would write:

const double* ptr (pointer to a constant double)

double* ptr (pointer to a mutable double)

In Rust, because the * symbol is used for both types and dereferencing, the syntax is explicitly:

*const f64

*mut f64

Important: You asked why you can't just do *f64. In Rust, *f64 is not a type. If you wrote *f64, the compiler would think you are trying to dereference the type f64, which makes no sense. The const or mut part is mandatory for raw pointer types to tell the compiler if you are allowed to write to that memory or not.

3. const as a Keyword vs. *const as a Pointer Type
This is a point of confusion for many beginners. The word "const" is used in two different ways:

A. The const Keyword (Variables)
As you noted, variables are immutable by default (let x = 5). So why do we have const MAX: i32 = 100?

let: A variable on the stack. It has a memory address. It is evaluated at runtime.

const: A compile-time constant. It has no memory address. The compiler literally replaces every instance of MAX with the value 100 in your code (like a #define).

B. The *const Type (Pointers)
In a raw pointer *const f64, the const means "I am not allowed to use this pointer to change the data it points to." It is exactly like C's const double*.

Even if the variable x was let mut x = 3.14;, if you cast it to a *const f64, you cannot use that specific pointer to change the value. You would need to cast it to a *mut f64 to do that.
