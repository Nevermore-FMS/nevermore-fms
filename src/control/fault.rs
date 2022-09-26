#[derive(Clone)]
pub struct Fault {
    code: String,
    name: String,
    description: String,
    active: bool,
}
impl Fault {
    pub fn new(code: String, name: String, description: String, active: bool) -> Self {
        Self { code, name, description, active }
    }
    pub fn code(&self) -> String {
        self.code.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn clear(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}