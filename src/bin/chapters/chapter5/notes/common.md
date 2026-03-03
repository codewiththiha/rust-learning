.unwarp() method means "I'm pretty sure this var isn't null (None)"

let first_char = s.chars().next().unwrap();

in that example .next() return an Option enum which includes Some and None
as we learnt earlier we need to implement and use match to works with enums to unpack
since it not sure the return value is which one , by using .unwarp we are promising
to the compiler that there will be Some value from .next()'s return Some and it remove the 
check for None.
