#![allow(dead_code)]

// CHECK: final labeling for static items:
// CHECK-DAG: "UNUSED": addr_of = UNIQUE
static UNUSED: usize = 2;
// CHECK-DAG: "UNUSED_MUT": addr_of = UNIQUE
static mut UNUSED_MUT: usize = 6;
// CHECK-DAG: "READ": addr_of = READ | UNIQUE
static READ: usize = 9;
// CHECK-DAG: "READ_MUT": addr_of = READ | UNIQUE
static mut READ_MUT: usize = 21;
// CHECK-DAG: "WRITTEN_MUT": addr_of = READ | WRITE | UNIQUE
static mut WRITTEN_MUT: usize = 3;

static mut oneshot_fdn: fdnode = fdnode { ctx: 0 as *mut u8 };

pub struct fdnode {
    pub ctx: *mut u8,
}

unsafe extern "C" fn server_free1() -> bool {
    oneshot_fdn.ctx.is_null()
}

unsafe extern "C" fn server_free2() -> () {
    &oneshot_fdn;
}

// CHECK: generated {{.*}} static rewrites:
// CHECK-DAG: static mut UNUS ...  = 6;: static $0
// CHECK-DAG: static mut READ ... = 21;: static $0

fn main() {
    let x = READ;
    if x > unsafe { READ_MUT } {
        unsafe {
            WRITTEN_MUT = 6000;
        }
    }
}

// CHECK:  ===== BEGIN
// CHECK: static UNUSED_MUT: usize = 6;
// CHECK: static READ_MUT: usize = 21;
// CHECK: static mut WRITTEN_MUT: usize = 3;
