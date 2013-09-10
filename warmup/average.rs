use std::{os, float};

fn main() {
let mut sum = 0.0;
let mut n =0;
let args: ~[~str] = os::args();
for args.slice(1, args.len()).iter().advance |s| {
match float::from_str(*s) {
Some(num) =>
{
sum+=num;
n=n+1;
}
None =>{
 println (fmt!("%s is bad input!", *s))
}

}
}
println(fmt!("Average:%f",sum/(n as float)));
}
