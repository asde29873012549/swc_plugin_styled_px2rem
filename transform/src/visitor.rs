use crate::config::Config;

pub struct PxToRem {
    pub(crate) config: Config,
    pub(crate) px2rem_used: bool,
}