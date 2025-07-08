fn main() {
    let v_0 = vec![0_u8, 1, 2, 3, 4, 5];
    let v_1 = v_0.clone(); // 调用clone的时候堆上的数据也会被复制一份
    let v_2 = &v_0;
    println!("{:p}", v_0.as_ptr());
    println!("{:p}", v_1.as_ptr()); // v1 对堆上数据产生了完全复制
    println!("{:p}", v_2.as_ptr()); // v2 v0指向同一块内存空间

    use bytes::Bytes; // Bytes内部是通过Arc实现的

    let b_0 = Bytes::from_static(b"hello");
    let b_1 = b_0.clone(); // Arc的clone并不会导致堆上数据复制
    println!("{:p}", b_0.as_ptr());
    println!("{:p}", b_1.as_ptr());
}
