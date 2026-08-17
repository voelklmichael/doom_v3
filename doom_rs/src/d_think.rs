pub type actionf_v = Option<unsafe extern "C" fn()>;

pub type actionf_p1 = Option<unsafe extern "C" fn(*mut std::ffi::c_void)>;

pub type actionf_p2 = Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub union actionf_t {
    pub acp1: actionf_p1,
    pub acv: actionf_v,
    pub acp2: actionf_p2,
}

pub type think_t = actionf_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thinker_t {
    pub prev: *mut thinker_s,
    pub next: *mut thinker_s,
    pub function: think_t,
}

pub type thinker_s = thinker_t;
