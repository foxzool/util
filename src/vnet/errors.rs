use crate::Error;

lazy_static! {
    pub static ref ERR_OBS_CANNOT_BE_NIL: Error = Error::new("obs cannot be nil".to_owned());
    pub static ref ERR_USE_CLOSED_NETWORK_CONN: Error =
        Error::new("use of closed network connection".to_owned());
    pub static ref ERR_ADDR_NOT_UDPADDR: Error = Error::new("addr is not a net.UDPAddr".to_owned());
    pub static ref ERR_LOC_ADDR: Error = Error::new("something went wrong with locAddr".to_owned());
    pub static ref ERR_ALREADY_CLOSED: Error = Error::new("already closed".to_owned());
    pub static ref ERR_NO_REM_ADDR: Error = Error::new("no remAddr defined".to_owned());
}
