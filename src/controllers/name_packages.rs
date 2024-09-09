use tokio::runtime::Runtime;
use crate::service::fetch_multiple_packages;

pub fn name_packages(packages: Vec<String>, rt: &Runtime) {
    rt.block_on(fetch_multiple_packages(packages));
}
