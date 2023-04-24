use anyhow::Result;
use std::time::Duration;

use ic_tests::driver::group::SystemTestGroup;
use ic_tests::nns_tests::sns_deployment::{
    add_one_participant, initiate_token_swap, sns_setup_fast,
};
use ic_tests::systest;

/// This is a non-interactive test:
/// 1. Install NNS and SNS
/// 2. Start the token sale
/// 3. Add one sale-participating user
fn main() -> Result<()> {
    SystemTestGroup::new()
        .with_overall_timeout(Duration::from_secs(15 * 60)) // 15 min
        .with_setup(sns_setup_fast)
        .add_test(systest!(initiate_token_swap))
        .add_test(systest!(add_one_participant))
        .execute_from_args()?;
    Ok(())
}