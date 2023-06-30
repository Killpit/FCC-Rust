/*
Borrowing:

-Way of temporarily access data without taking ownership of it.
-When borrowing, you're taking a reference (pointer) to the data, not the data itself.
-Prevention of dangling pointers and data races 
-Data can be borrowed immutably and mutably
-There are certain rules when borrowing which we have to comply with, otherwise, the program won't compile */

