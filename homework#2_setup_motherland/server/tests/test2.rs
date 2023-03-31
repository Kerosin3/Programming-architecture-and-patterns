#[cfg(test)]
// mocking
#[mockall::automock]
#[tonic::async_trait]
trait TonicExecutor {
    fn accept_client(&mut self, q: String);
    //     fn send_queryV2(&mut self, q: String) -> anyhow::Result<()>;
}
fn try_accept_client(mut db: Box<dyn TonicExecutor>, id: i32) -> anyhow::Result<()> {
    let q = format!("query {}", id);
    db.accept_client(q);
    Ok(())
}

/*
fn send_query_v2(mut db: Box<dyn DatabaseExecutor>, id: i32) -> anyhow::Result<u16> {
    let q = format!("queryv2 {}", id);
    db.send_queryV2(q)?;
    Ok(13_u16)
}
*/
mod test1 {
    use crate::*;
    use anyhow::*;
    //    use lib_game_mechanics::run_me;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::*;
    #[rstest]
    #[case(42, format!("query {}",42_i32))]
    fn test_initializev2(#[case] input: i32, #[case] expected: String) {
        let mut mock_server = Box::new(MockTonicExecutor::new());
        mock_server
            .expect_accept_client()
            .with(eq(expected))
            .times(1)
            .returning(|_x| ());
        // оно настроило и ожидает, что будет вызвана квери с ид 22
        assert!(try_accept_client(mock_server, input).is_ok());
    }
}
