#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn t1() {
    assert_eq!(device_count(), 1);
}
pub fn device_count() -> i64 {
    use cudaGetDeviceCount;
    let mut x = 0;
    let err;
    unsafe {
        err = cudaGetDeviceCount(&mut x);
    }
    return x as i64;
}
