use tonic::{Request, Response, Status};
use crate::grpc::traits::test::test_service_server::TestService;
use crate::grpc::traits::test::{TestRequest, TestResponse};

#[derive(Debug, Default)]
pub struct MyTestService;

#[tonic::async_trait]
impl TestService for MyTestService {
    async fn test(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestResponse>, Status> {
        let req = request.into_inner();
        let reply = TestResponse {
            testres: format!("Hello, {}!", req.testreq),
        };
        Ok(Response::new(reply))
    }
}
