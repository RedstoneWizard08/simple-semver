#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChangeType {
    Alpha,
    Beta,
    None,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SemVerChangeID {
    pub r#type: ChangeType,
    pub id: i32,
}

unsafe impl Send for ChangeType {}
unsafe impl Sync for ChangeType {}

unsafe impl Send for SemVerChangeID {}
unsafe impl Sync for SemVerChangeID {}

impl ChangeType {
    pub fn to_string(&self) -> String {
        return match self.clone() {
            ChangeType::Alpha => String::from("alpha"),
            ChangeType::Beta => String::from("beta"),
            ChangeType::None => String::from(""),
        };
    }
}

impl SemVerChangeID {
    pub fn gt(&self, other: SemVerChangeID) -> bool {
        if other.r#type == ChangeType::None {
            return match self.r#type {
                ChangeType::None => self.id > other.id,
                _ => false,
            };
        } else if other.r#type == ChangeType::Alpha {
            return match self.r#type {
                ChangeType::Alpha => self.id > other.id,
                _ => true,
            };
        } else if other.r#type == ChangeType::Beta {
            return match self.r#type {
                ChangeType::Beta => self.id > other.id,
                ChangeType::Alpha => false,
                ChangeType::None => true,
            };
        }

        panic!("Unknown change type!");
    }

    pub fn lt(&self, other: SemVerChangeID) -> bool {
        if other.r#type == ChangeType::None {
            return match self.r#type {
                ChangeType::None => self.id < other.id,
                _ => true,
            };
        } else if other.r#type == ChangeType::Alpha {
            return match self.r#type {
                ChangeType::Alpha => self.id < other.id,
                _ => false,
            };
        } else if other.r#type == ChangeType::Beta {
            return match self.r#type {
                ChangeType::Beta => self.id < other.id,
                ChangeType::Alpha => true,
                ChangeType::None => false,
            };
        }

        panic!("Unknown change type!");
    }

    pub fn eq(&self, other: SemVerChangeID) -> bool {
        return other.r#type == self.r#type && other.id == self.id;
    }

    pub fn to_string(&self) -> String {
        if self.r#type == ChangeType::None {
            return self.id.to_string();
        }

        return format!("{}.{}", self.r#type.to_string(), self.id.to_string());
    }
}
