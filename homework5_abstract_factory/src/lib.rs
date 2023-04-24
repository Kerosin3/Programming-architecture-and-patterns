// Abstract factory
pub trait CpuBoundTask<T: Sorting> {
    fn instantiate_sorting(&self) -> T;
}
// sorting methods
pub trait Sorting {
    fn assign_data(&mut self, vec: &[i32]);
    fn do_task(&mut self);
}
