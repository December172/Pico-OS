use crate::Kernel::Services::Service::Service;

pub struct ServiceRegistry {
    loadedServices : [Option<&'a dyn Service>; 32],
    count : u8
}

impl ServerRegistry {
    fn new() -> Self {
        Self {
            count : 0,
            loadedServices : [None; 32]
        }
    }

    fn load(&mut self, serviceDescriptor: &'a ServiceDescriptor) {
        self.loadedServices[count] = Some(serviceDescriptor);
        self.count += 1;
    }

    fn unload(&mut self, serviceID: ServiceID) {
        for (index, service) in self.loadedServices.iter().enumerate() {
            if (service.id == serviceID) {
                self.loadedServices[index] = self.loadedServices[count - 1];
                self.loadedServices[count - 1] = None;
                break;
            }
        }
    }

    fn require(&mut self, serviceID: ServiceID) -> &'a dyn Service {
        for (_, service) in self.loadedServices.iter().enumerate() {
            if (service.unwrap().id == serviceID) {
                return service.unwrap().instance;
            }
        }
    }
}