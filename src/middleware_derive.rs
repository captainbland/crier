//use iron::{Request, Response, IronResult,
//           middleware::{
//               BeforeMiddleware,
//           },
//           typemap::Key,
//           request::*
//};
//
//impl<T> AroundMiddleware for T {
//    fn around(self, handler: Box<Handler>) -> Box<Handler> {
//        Box::new(move |req: &mut Request| -> IronResult<Response> {
//            let s = self.backend.from_request(req);
//            req.extensions.insert::<SessionKey>(Session::new(Box::new(s)));
//            let mut res = handler.handle(req);
//            let s = req.extensions.remove::<SessionKey>().unwrap();
//            if s.has_changed {
//                match res {
//                    Ok(ref mut r) => try!(s.inner.write(r)),
//                    Err(ref mut e) => try!(s.inner.write(&mut e.response))
//                }
//            };
//            res
//        })
//    }
//}