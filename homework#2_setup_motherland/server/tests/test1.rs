#[cfg(test)]
#[mockall::automock]
trait DatabaseExecutor {
    fn send_query(&mut self, q: String);
    fn send_queryV2(&mut self, q: String) -> anyhow::Result<()>;
}

fn send_some_query_to_db(mut db: Box<dyn DatabaseExecutor>, id: i32) -> anyhow::Result<()> {
    let q = format!("query {}", id);
    db.send_query(q);
    Ok(())
}

fn send_query_v2(mut db: Box<dyn DatabaseExecutor>, id: i32) -> anyhow::Result<u16> {
    let q = format!("queryv2 {}", id);
    db.send_queryV2(q)?;
    Ok(13_u16)
}

mod test1 {
    use crate::*;
    use anyhow::*;
    use lib_game_mechanics::run_me;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::*;
    #[test]
    ///
    fn some_test() {
        let x = true;
        run_me();
        assert!(x);
    }
    #[rstest]
    #[case(42, format!("query {}",42_i32))]
    #[case(555, format!("query {}",555_i32))]
    fn test_initialize(#[case] input: i32, #[case] expected: String) {
        let mut mockdb = Box::new(MockDatabaseExecutor::new());
        mockdb
            .expect_send_query()
            .with(eq(expected))
            .times(1)
            .returning(|_x| ());
        // оно настроило и ожидает, что будет вызвана квери с ид 22
        assert!(send_some_query_to_db(mockdb, input).is_ok());
    }
}
