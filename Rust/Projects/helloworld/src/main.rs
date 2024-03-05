fn main() {
    // Option
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // Result
    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), default);
}
