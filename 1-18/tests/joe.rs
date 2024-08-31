#![cfg(feature = "test-sbf")]

use solana_program_test::*;

#[tokio::test]
async fn test_me() {
    let program_test = ProgramTest::default();
    let mut context = program_test.start_with_context().await;

    // 1.18:
    // This will cause a program cache error on `assign_program()` for both
    // Loader v4 and ZK Token Proof.
    context.warp_to_slot(100).unwrap();
}
