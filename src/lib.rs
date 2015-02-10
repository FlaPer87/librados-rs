#![feature(unsafe_destructor)]
#![feature(libc)]

extern crate libc;
extern crate "rados-sys" as rados_sys;

#[macro_use]
extern crate log;

use std::ptr;

fn connect() {
    let mut cluster: rados_sys::rados_t = ptr::null_mut();

    unsafe {
        let cfg = "/home/flaper87/workspace/ceph/src/ceph.conf".as_bytes().as_ptr();
        let rados = rados_sys::rados_create(&mut cluster, ptr::null());
        let err = rados_sys::rados_conf_read_file(cluster, cfg as *const i8);
        if (err < 0) {
            panic!("YOOOOO ERROR READING CONFIG");
        }

        let err = rados_sys::rados_connect(cluster);
        if (err < 0) {
            panic!("YOOOOO ERROR CONNECTINGb");
        }

        let id = rados_sys::rados_get_instance_id(cluster);
        println!("Instance id is: {:?}", id);
    }
}

#[test]
fn test_connect() {
    connect();
}
