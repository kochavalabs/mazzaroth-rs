
pub trait Contract {
    fn execute(&mut self);
}

pub fn dispatch(mut contract: Box<Contract>) {
    contract.execute();
}