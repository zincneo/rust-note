mod m01_read_write;
mod m02_in_out_err;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_read_write() {
        assert_eq!(m01_read_write::f01_read_write_trait(), ());
        assert_eq!(m01_read_write::f02_seek_bufread(), ());
        assert_eq!(m01_read_write::f03_io_result(), ());
    }

    #[test]
    fn test02_in_out_err() {
        assert_eq!(m02_in_out_err::f01_stdout(), ());
    }
}
