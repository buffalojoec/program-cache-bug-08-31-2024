#![cfg(feature = "test-sbf")]

use solana_program_test::*;

#[tokio::test]
async fn test_me() {
    let program_test = ProgramTest::default();
    let mut context = program_test.start_with_context().await;

    // 2.0:
    // This will _not_ cause a program cache error on `assign_program()`.
    context.warp_to_slot(100).unwrap();
}
