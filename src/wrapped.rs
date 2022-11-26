use crate::raw::*;

#[derive(Debug, Clone, Copy)]
pub struct RyzenAdjVersion {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

impl std::fmt::Display for RyzenAdjVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "RyzenAdj v{}.{}.{}", self.major, self.minor, self.revision)
    }
}

pub const fn version() -> RyzenAdjVersion {
    RyzenAdjVersion {
        major: RYZENADJ_MAJOR_VER,
        minor: RYZENADJ_MINIOR_VER,
        revision: RYZENADJ_REVISION_VER,
    }
}

#[derive(Debug)]
pub enum RyzenAdjErr {
    Init,
    Table(i32),
    NaN,
    UnknownFamily(i32),
    FamilyUnsupported,
    SmuTimeout,
    SmuUnsupported,
    SmuRejected,
    MemoryAccess,
    Unknown(i32),
}

impl RyzenAdjErr {
    fn from_set_code(result: i32) -> Result<(), Self> {
        match result {
            0 => Ok(()),
            ADJ_ERR_FAM_UNSUPPORTED => Err(Self::FamilyUnsupported),
            ADJ_ERR_SMU_TIMEOUT => Err(Self::SmuTimeout),
            ADJ_ERR_SMU_UNSUPPORTED => Err(Self::SmuUnsupported),
            ADJ_ERR_SMU_REJECTED => Err(Self::SmuRejected),
            ADJ_ERR_MEMORY_ACCESS => Err(Self::MemoryAccess),
            res => Err(Self::Unknown(res))
        }
    }
}

impl std::fmt::Display for RyzenAdjErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Init => write!(f, "Init error"),
            Self::Table(i) => write!(f, "Table init error {}", i),
            Self::NaN => write!(f, "NaN returned"),
            Self::UnknownFamily(i) => write!(f, "Unknown CPU family {}", i),
            Self::FamilyUnsupported => write!(f, "CPU Family unsupported"),
            Self::SmuTimeout => write!(f, "SMU timeout"),
            Self::SmuUnsupported => write!(f, "SMU unsupported"),
            Self::SmuRejected => write!(f, "SMU rejected"),
            Self::MemoryAccess => write!(f, "Memory access error"),
            Self::Unknown(i) => write!(f, "Unknown error {}", i),
        }
    }
}

impl std::error::Error for RyzenAdjErr {}

pub struct RyzenAccess {
    raw_access: ryzen_access,
}

/// Macro for re-declaring value getters for raw_access in a more Rust-centric format
macro_rules! pub_get_raw {
    ($raw_fn:ident, $pretty_fn:ident, $result:ident) => {
        #[inline]
        pub fn $pretty_fn(&self) -> $result {
            unsafe {$raw_fn(self.raw_access)}
        }
    };
    ($raw_fn:ident, $pretty_fn:ident, $result:ident, $param0:ident) => {
        #[inline]
        pub fn $pretty_fn(&self, p0: $param0) -> $result {
            unsafe {$raw_fn(self.raw_access, p0)}
        }
    }
}

/// Macro for re-declaring value setters for raw_access in a more Rust-centric format
macro_rules! pub_set_raw {
    ($raw_fn:ident, $pretty_fn:ident) => {
        pub fn $pretty_fn(&self) -> Result<(), RyzenAdjErr> {
            RyzenAdjErr::from_set_code( unsafe {$raw_fn(self.raw_access)} )
        }
    };
    ($raw_fn:ident, $pretty_fn:ident, $value:ident) => {
        pub fn $pretty_fn(&self, val: $value) -> Result<(), RyzenAdjErr> {
            RyzenAdjErr::from_set_code( unsafe {$raw_fn(self.raw_access, val)})
        }
    }
}

impl RyzenAccess {

    pub fn new() -> Result<Self, RyzenAdjErr> {
        let access = unsafe { init_ryzenadj() };
        if access.is_null() {
            Err(RyzenAdjErr::Init)
        } else {
            let table_result = unsafe { init_table(access) };
            if table_result != 0 {
                Err(RyzenAdjErr::Table(table_result))
            } else {
                Ok(Self {
                    raw_access: access,
                })
            }
        }
    }

    pub_get_raw!{get_bios_if_ver, get_bios_if_ver, i32}

    pub_get_raw!{get_table_ver, get_table_ver, u32}
    pub_get_raw!{get_table_size, get_table_size, usize}

    pub unsafe fn get_table_values(&self) -> *mut f32 {
        get_table_values(self.raw_access)
    }

    pub fn refresh_table(&self) -> i32 {
        unsafe {refresh_table(self.raw_access)}
    }

    pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_get_raw!{get_stapm_value, get_stapm_value, f32}
    pub_set_raw!{set_stapm_limit, set_stapm_limit, u32}

    pub_get_raw!{get_fast_limit, get_fast_limit, f32}
    pub_get_raw!{get_fast_value, get_fast_value, f32}
    pub_set_raw!{set_fast_limit, set_fast_limit, u32}

    pub_get_raw!{get_slow_limit, get_slow_limit, f32}
    pub_get_raw!{get_slow_value, get_slow_value, f32}
    pub_set_raw!{set_slow_limit, set_slow_limit, u32}

    pub_get_raw!{get_slow_time, get_slow_time, f32}
    pub_set_raw!{set_slow_time, set_slow_time, u32}

    pub_get_raw!{get_stapm_time, get_stapm_time, f32}
    pub_set_raw!{set_stapm_time, set_stapm_time, u32}

    pub_get_raw!{get_tctl_temp, get_tctl_temp, f32}
    pub_get_raw!{get_tctl_temp_value, get_tctl_temp_value, f32}
    pub_set_raw!{set_tctl_temp, set_tctl_temp, u32}

    pub_get_raw!{get_vrm_current, get_vrm_current, f32}
    pub_get_raw!{get_vrm_current_value, get_vrm_current_value, f32}
    pub_set_raw!{set_vrm_current, set_vrm_current, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_vrmsoc_current, set_vrmsoc_current, u32}

    pub_get_raw!{get_vrmsoc_current, get_vrmsoc_current, f32}
    pub_get_raw!{get_vrmsoc_current_value, get_vrmsoc_current_value, f32}
    pub_set_raw!{set_vrmgfx_current, set_vrmgfx_current, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_vrmcvip_current, set_vrmcvip_current, u32}

    pub_get_raw!{get_vrmmax_current, get_vrmmax_current, f32}
    pub_get_raw!{get_vrmmax_current_value, get_vrmmax_current_value, f32}
    pub_set_raw!{set_vrmmax_current, set_vrmmax_current, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_vrmgfxmax_current, set_vrmgfxmax_current, u32}

    pub_get_raw!{get_vrmsocmax_current, get_vrmsocmax_current, f32}
    pub_get_raw!{get_vrmsocmax_current_value, get_vrmsocmax_current_value, f32}
    pub_set_raw!{set_vrmsocmax_current, set_vrmsocmax_current, u32}

    pub_get_raw!{get_psi0_current, get_psi0_current, f32}
    pub_set_raw!{set_psi0_current, set_psi0_current, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_psi3cpu_current, set_psi3cpu_current, u32}

    pub_get_raw!{get_psi0soc_current, get_psi0soc_current, f32}
    pub_set_raw!{set_psi0soc_current, set_psi0soc_current, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_psi3gfx_current, set_psi3gfx_current, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_max_gfxclk_freq, set_max_gfxclk_freq, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_min_gfxclk_freq, set_min_gfxclk_freq, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_max_socclk_freq, set_max_socclk_freq, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_min_socclk_freq, set_min_socclk_freq, u32}

    pub_get_raw!{get_cclk_setpoint, get_cclk_setpoint, f32}
    pub_get_raw!{get_cclk_busy_value, get_cclk_busy_value, f32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_max_fclk_freq, set_max_fclk_freq, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_min_fclk_freq, set_min_fclk_freq, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_max_vcn, set_max_vcn, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_min_vcn, set_min_vcn, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_max_lclk, set_max_lclk, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_min_lclk, set_min_lclk, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_prochot_deassertion_ramp, set_prochot_deassertion_ramp, u32}

    pub_get_raw!{get_apu_skin_temp_limit, get_apu_skin_temp_limit, f32}
    pub_get_raw!{get_apu_skin_temp_value, get_apu_skin_temp_value, f32}
    pub_set_raw!{set_apu_skin_temp_limit, set_apu_skin_temp_limit, u32}

    pub_get_raw!{get_dgpu_skin_temp_limit, get_dgpu_skin_temp_limit, f32}
    pub_get_raw!{get_dgpu_skin_temp_value, get_dgpu_skin_temp_value, f32}
    pub_set_raw!{set_dgpu_skin_temp_limit, set_dgpu_skin_temp_limit, u32}

    pub_get_raw!{get_apu_slow_limit, get_apu_slow_limit, f32}
    pub_get_raw!{get_apu_slow_value, get_apu_slow_value, f32}
    pub_set_raw!{set_apu_slow_limit, set_apu_slow_limit, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_skin_temp_power_limit, set_skin_temp_power_limit, u32}

    pub_get_raw!{get_gfx_clk, get_gfx_clk, f32}
    pub_set_raw!{set_gfx_clk, set_gfx_clk, u32}

    pub_get_raw!{get_gfx_temp, get_gfx_temp, f32}
    pub_get_raw!{get_gfx_volt, get_gfx_volt, f32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_oc_clk, set_oc_clk, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_per_core_oc_clk, set_per_core_oc_clk, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_oc_volt, set_oc_volt, u32}

    pub_set_raw!{set_disable_oc, set_disable_oc}

    pub_set_raw!{set_enable_oc, set_enable_oc}

    pub_set_raw!{set_power_saving, set_power_saving}

    pub_set_raw!{set_max_performance, set_max_performance}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_coall, set_coall, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_coper, set_coper, u32}

    //pub_get_raw!{get_stapm_limit, get_stapm_limit, f32}
    pub_set_raw!{set_cogfx, set_cogfx, u32}

    pub_get_raw!{get_core_clk, get_core_clk, f32, u32}
    pub_get_raw!{get_core_volt, get_core_volt, f32, u32}
    pub_get_raw!{get_core_power, get_core_power, f32, u32}
    pub_get_raw!{get_core_temp, get_core_temp, f32, u32}

    pub_get_raw!{get_l3_clk, get_l3_clk, f32}
    pub_get_raw!{get_l3_logic, get_l3_logic, f32}
    pub_get_raw!{get_l3_vddm, get_l3_vddm, f32}
    pub_get_raw!{get_l3_temp, get_l3_temp, f32}

    pub_get_raw!{get_mem_clk, get_mem_clk, f32}
    pub_get_raw!{get_fclk, get_fclk, f32}

    pub_get_raw!{get_soc_power, get_soc_power, f32}
    pub_get_raw!{get_soc_volt, get_soc_volt, f32}

    pub_get_raw!{get_socket_power, get_socket_power, f32}
}

impl std::ops::Drop for RyzenAccess {
    fn drop(&mut self) {
        unsafe {cleanup_ryzenadj(self.raw_access)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_works() {
        let access = RyzenAccess::new();
        println!("{}", version());
        drop(access);
    }
}
