<<<<<<< HEAD

pub struct Context{
    pub realm_request: realm::Request,
}

pub fn middleware(req: realm::Request) -> realm::Result {
    let ireq = Context {
=======
pub fn middleware(req: realm::Request) -> realm::Result{
    let ireq  = crate::in_::In{
>>>>>>> 8fe523b4da4067ef58c5c4e4a00123791facfda0
        realm_request: req,
    };
    crate::forward::magic(ireq)
}
