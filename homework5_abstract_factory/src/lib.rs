// Abstract factory
pub trait CpuBoundTask<T: Sorting> {
    fn instantiate_sorting(&self) -> T;
}
// sorting methods
pub trait Sorting {
    fn assign_data(&mut self, vec: &[i32]);
    fn do_task(&mut self);
    fn validate_data(&self, vec: &[i32]) -> Option<()>;
    fn printout(&self);
    fn get_data(&self) -> Vec<i32>;
    fn perform(&mut self, data: &[i32]) -> Vec<i32> {
        self.assign_data(data);
        self.do_task();
        self.get_data()
    }
}
