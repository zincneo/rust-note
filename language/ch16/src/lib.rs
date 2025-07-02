mod m01_thread;
mod m02_mpsc;
mod m03_mutex_rwlock;
mod m04_condvar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_thread() {
        assert_eq!(m01_thread::f01_thread(), ());
        assert_eq!(m01_thread::f02_barrier(), ());
    }

    #[test]
    fn test02_mpsc() {
        assert_eq!(m02_mpsc::f01_async_mpsc(), ());
        assert_eq!(m02_mpsc::f02_try_recv(), ());
        assert_eq!(m02_mpsc::f03_sync_channel(), ());
    }

    #[test]
    fn test03_mutex_rwlock() {
        assert_eq!(m03_mutex_rwlock::f01_mutext(), ());
        assert_eq!(m03_mutex_rwlock::f02_rwlock(), ());
    }

    #[test]
    fn test04_condvar() {
        assert_eq!(m04_condvar::f01_condvar(), ());
    }
}
