
Consider the string "abbcdefffgh"

You can see 3 repetitions here: "bb" (once), "ff" (twice). 

Problem is to write a program which will count such repetitions.

This is a good demonstration of idiomatic coding in Rust which
makes heavy use of iterator chains.

How to do it
--------------

Let's count the number of repetitions in this string:

a b b c d d e

There are two repetitions here.

Let's write down the original string and the same string with the first
character missing; one below the other:

a b b c d d e

b b c d d e 

Let's now "zip" them:

[(a, b), (b, b), (b, c), (c, d), (d, d), (d, e)]


Now, let us run a for loop which will take out each tuple from the sequence -
if the two elements of the tuple are equal, increment a count.

The total count gives us the result.

Note: It is possible to avoid the for loop and write in a "pure iterator" style.
