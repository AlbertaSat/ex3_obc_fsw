pub mod ports {
    pub const SIM_DFGM_PORT: u16 = 1802;
    pub const SIM_ADCS_PORT: u16 = 1803;
    pub const SIM_EPS_PORT: u16 = 1804;
    pub const SIM_COMMS_PORT: u16 = 1805;
    pub const SIM_IRIS_PORT: u16 = 1806;

    pub const DFGM_HANDLER_DISPATCHER_PORT: u16 = 1900;
    pub const SCHEDULER_DISPATCHER_PORT: u16 = 1901;
    pub const SUBSYSTEM_MONITOR_DISPATCHER_PORT: u16 = 1902;
    pub const BULK_MSG_HANDLER_DISPATCHER_PORT: u16 = 1903;
}

pub mod dest_ids {
    pub const OBC: u8 = 0;
    pub const EPS: u8 = 1;
    pub const ADCS: u8 = 2;
    pub const DFGM: u8 = 3;
    pub const IRIS: u8 = 4;
    pub const GPS: u8 = 5;
    pub const DEPLOYABLES: u8 = 6;
    pub const GROUND_STATION: u8 = 7;
}