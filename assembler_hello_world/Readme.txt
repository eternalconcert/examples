Debugging assembler with gdb
============================

$ gdb hello
(gdb) list              # Prints the whole program to the screen
(gdb) break _start      # A breakpoint at the start point is needed. Also possible: (gdb) break 5 => Creates a breakpoint at the fifth line
(gdb) run               # Runs the program to the breakpoint
(gdb) info registers    # Print the current register contet to the screen
(gdb) stepi             # Execute the next line

Continue with info_registers and stepi to check register content at each step of execution.
