#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_works() {
        let access = unsafe {init_ryzenadj()};
        println!("RyzenAdj v{}.{}.{}", RYZENADJ_MAJOR_VER, RYZENADJ_MINIOR_VER, RYZENADJ_REVISION_VER);
        unsafe{ cleanup_ryzenadj(access) }
    }
}
