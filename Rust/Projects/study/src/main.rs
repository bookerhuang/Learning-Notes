fn main() {
    // 数值类型的转换
    assert_eq!(10i8 as u16, 10u16);
    assert_eq!(123u16 as i16, 123i16);
    // bool类型转换
    assert_eq!(false as u32, 0u32);
    assert_eq!(true as i8, 1i8);
    // char类型相关转换
    assert_eq!('我' as i32, 25105i32);  // char转换到i32
    assert_eq!('是' as u8, 47u8);       // char转换到u8，会被截断
    assert_eq!(97u8 as char, 'a');      // u8转换到char
    assert_eq!(std::char::from_u32(0x2764).unwrap(), '❤');
    println!("char to digit: {:?}", std::char::from_u32(0x2764));
    assert_eq!(std::char::from_digit(4, 10).unwrap(), '4');
}
