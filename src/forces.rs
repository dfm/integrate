pub mod gravity;

pub use gravity::Gravity;

pub trait Force {
    fn accumulate_accelerations(
        &self,
        configuration: &crate::system::Configuration,
        workspace: &mut crate::system::Workspace,
    );
}
