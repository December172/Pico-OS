use crate::Kernel::Services::ServiceID::ServiceID;

pub trait Service {
    fn getID(&self) -> ServiceID;
}
