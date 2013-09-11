Title: Problem Set 1 Answers
Author: Zach Seid
zws3mb

Problem 1: Mozilla/5.0 (X11; Ubuntu; Linux i686;rv:23.0)Gecko/20100101 Firefox/23.0
Mozilla/5.0 refers to compatibility, i.e. that my browser is Mozilla compatible with Mozilla version 5.0.
X11 refers to X Window System, or the system-level software for GUIing via windows
Ubuntu is the operating system, running on 32-bit (i686) based Linux.
rv:23.0 apparently refers to the version of Gecko, where 20100101 is the build date (January 1st, 2010)

Problem 2: Speculate on why Rust thinks it is unsafe to modify a global variable like this:

Adjusting, or even having global variables is generally bad coding practice, since if one function can edit its value, so theoretically can any other. For this reason, global variable values are unknown and unstable.


