pub struct In{
    pub realm_request: realm::Request,
}

impl In{
    pub fn new(req: realm::Request) -> In{
        In{
             realm_request: req,
        }
    }
}