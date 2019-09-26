use crate::handlers::VitessPupperQueriesHandler;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetPupperQuery, GetRandomPupperQuery, GetTopTenPuppersQuery};
use doggo_core::dtos::Pupper;
use mysql;

const RETRY_COUNT: i32 = 30;

pub struct VitessQueryRetryWrapper {
    handler: VitessPupperQueriesHandler
}

impl VitessQueryRetryWrapper {
    pub fn new(handler: VitessPupperQueriesHandler) -> VitessQueryRetryWrapper {
        VitessQueryRetryWrapper {
            handler,
        }
    }
}

impl HandlesQuery<GetPupperQuery> for VitessQueryRetryWrapper {
    type Result = Result<Option<Pupper>, mysql::Error>;

    fn handle(&mut self, query: GetPupperQuery) -> Self::Result {
        for _ in 0..RETRY_COUNT {
            if let Ok(result) = self.handler.handle(&query) {
                return Ok(result);
            }
        }
        // Failed all retries so return result which likely re-occurring mysql error.
        self.handler.handle(&query)
    }
}

impl HandlesQuery<GetRandomPupperQuery> for VitessQueryRetryWrapper {
    type Result = Result<Option<Pupper>, mysql::Error>;

    fn handle(&mut self, query: GetRandomPupperQuery) -> Self::Result {
        for _ in 0..RETRY_COUNT {
            if let Ok(result) = self.handler.handle(&query) {
                return Ok(result);
            }
        }
        // Failed all retries so return result which likely re-occurring mysql error.
        self.handler.handle(&query)
    }
}

impl HandlesQuery<GetTopTenPuppersQuery> for VitessQueryRetryWrapper {
    type Result = Result<Option<Vec<Pupper>>, mysql::Error>;

    fn handle(&mut self, query: GetTopTenPuppersQuery) -> Self::Result {
        for _ in 0..RETRY_COUNT {
            if let Ok(result) = self.handler.handle(&query) {
                return Ok(result);
            }
        }
        // Failed all retries so return result which likely re-occurring mysql error.
        self.handler.handle(&query)
    }
}
