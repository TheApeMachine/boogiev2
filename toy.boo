/*
Define a Calculator object to perform various operations.
*/
Calculator =>
    digits <inputs []int|output string>

    /*
    Method to perform basic arithmetic operations.
    */
    arithmetic digits =>
        // Applying operations across a collection of inputs.
        sum = inputs.each <= +  // Test addition.
        difference = inputs.each <= -  // Test subtraction.
        product = inputs.each <= *  // Test multiplication.
        quotient = inputs.each <= /  // Test division.
        modulus = inputs.each <= %  // Test modulus.
        "Results: {sum}, {difference}, {product}, {quotient}, {modulus}"
    ;

    /*
    Method to test logical operations.
    */
    logical =>
        andResult = true and false  // Test AND operation.
        orResult = true or false  // Test OR operation.
        notResult = not true  // Test NOT operation.
        xorResult = true xor false  // Test XOR operation.
        "Logical Results: {andResult}, {orResult}, {notResult}, {xorResult}"
    ;

    /*
    Method to test input and output.
    */
    inputOutput =>
        userInput = input "Enter a number: "  // Get input from the user.
        doubleValue = userInput * 2  // Multiply input by 2.
        "You entered {userInput}, double of which is {doubleValue}"
    ;

    /*
    Method to test comparison and control flow.
    */
    comparison =>
        // Using match directly on the comparison result.
        match 8 > 5 =>
            | true => "8 is greater than 5"
            | false => "8 is not greater than 5"
        ;
    ;

    /*
    Execute all operations to test language basics.
    Capitalize the first letter to make it public.
    */
    TestAll =>
        stdout <= [
            arithmetic [1, 1, 2, 3, 5, 8],  // Output arithmetic results.
            logical,  // Output logical operation results.
            inputOutput,  // Handle input and output.
            comparison  // Test comparisons and control flow.
        ] <= join "\n"
    ;
; <= TestAll

