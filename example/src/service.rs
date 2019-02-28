
trait Service {
    
    fn call(session: &mut Session, request: Request) -> Response;
}


struct ExampleService {}

impl Service for ExampleService {

    fn call(session: &mut Session, request: Request) -> Response {

        // forward to 
    }
}