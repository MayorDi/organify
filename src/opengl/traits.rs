pub trait Build {
    fn build(&mut self) -> Result<(), String>;
}

pub trait GetId {
    fn id(&self) -> u32;
}

pub trait Status {
    type Output;
    fn status(&self) -> Self::Output;
}

pub trait Delete {
    fn delete(self);
}
