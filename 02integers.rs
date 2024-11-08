//INTEGERS 
/**
 * Integers are whole numbers
 * Integers are signed or unsigned
 * signed integers can be positive or negative
 * unsigned integers are always positive
 * Integers are either 8, 16, 32, 64, 128 bits or arch dependent
 * arch dependent means that the size of the integer depends on the architecture of the computer
 * 
 * 
 * Numbers can be written in decimal, hexadecimal, octal, or binary,
 * but the default is FLOAT f64 and INTEGERS are i32, if we do not specify the type,
 * 
 * 
 * BINARY
 *  Explanation:
 */

 DATA TYPE | SIZE | RANGE
 i8      | 1 byte | -128 to 127
 i16     | 2 bytes | -32,768 to 32,767
 i32     | 4 bytes | -2,147,483,648 to 2,147,483,647
 i64     | 8 bytes | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
 i128    | 16 bytes | -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727
 isize   | arch dependent | -2,147,483,648 to 2,147,483,647

 DATA TYPE | SIZE | RANGE
 u8      | 1 byte | 0 to 255
 u16     | 2 bytes | 0 to 65,535
 u32     | 4 bytes | 0 to 4,294,967,295
 u64     | 8 bytes | 0 to 18,446,744,073,709,551,615
 u128    | 16 bytes | 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
 usize   | arch dependent | 0 to 18,446,744,073,709,551,615

//Note that the range of values for the integer types are not the same as the range of values for the floating point types.
//Unsigned integers never go below 0`, and signed integers can go below `0`.


 /*Arch dependent means that the size of the integer depends on the architecture of the computer.
 For example, on a 32-bit computer, an integer is 4 bytes, and on a 64-bit computer, an integer is 8 bytes.*/

 Address | Data (1 byte)
 0 x 0001 | 0000 0001
 0 x 0002 | 0000 0010
 0 x 0003 | 0000 0011
 0 x 0004 | 0000 0100
 0 x 0005 | 0000 0101
 0 x 0006 | 0000 0111
 0 x 0007 | 0000 1000
 0 x 0008 | 0000 1001
 0 x 0009 | 0000 1010
 
 
 
 //Note that the if we do not specify the type, the default is FLOAT f64 and INTEGERS are i32
 //Note that we cannot assign a variable of a type to another of different type
