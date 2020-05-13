(LOOP)        // User defined Label (l_command)
    @i        // User defined variable (a_command)
    D=M       // c_command
    @n        // User defined variable (a_command)
    D=D-M     // c_command
    @PRESSED  // User defined 
    D;JEQ     // c_command

    @address  // User defined variable (a_command)
    D=M       // c_command
    @i        // User defined varibale (a_command)
    A=D+M     // c_command
    M=-1      // c_command

    @i        // User defined variable (a_command)
    M=M+1     // c_command

    @LOOP     // Jump to previously defined label (a_command)
    0;JMP     // c_command
