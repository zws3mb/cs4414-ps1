use std::{os, uint};

fn main() {
let args:~[~str] = os::args();
for args.slice(1, args.len()).iter().advance |s| {
print(fmt!("%s ",*s));
}
println("");
}
