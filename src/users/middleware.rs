use std::net::SocketAddr;
use std::task::{Context, Poll};
use axum::extract::{ConnectInfo, Request};
use axum::http::HeaderValue;
use axum::response::Response;
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::info;

//tower
#[derive(Debug,Clone)]
pub struct MyLayer;
//S->a是一个服务//实现服务
impl <S>Layer<S> for MyLayer{
    type Service=MyMiddleware<S>; //我自己的middleware
    fn layer(&self,inner:S)->Self::Service{
        MyMiddleware{inner}
    }

}
//request->md1->md2->m3->md2->md1->response
#[derive(Debug,Clone)]
pub struct MyMiddleware<S>{
    inner:S
}

//实现trait
impl <S>Service<Request>for MyMiddleware<S>
where
    S:Service<Request,Response=Response>+Send+Clone+'static,
    S::Future:Send+'static,
{
    type Response=Response;
    type Error=S::Error;
    type Future=BoxFuture<'static,Result<Self::Response,Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, req: Request) -> Self::Future {
        let mut inner=self.inner.clone();

        let uri=req.uri().path().to_string();

        let method=req.method().to_string();

       if let Some(ConnectInfo(conn))= req.extensions().get::<ConnectInfo<SocketAddr>>(){
        info!("{}-{}:{}\n",conn,uri,method);
       }else{

        info!("uri:{},method:{}",uri,method);
        }

        Box::pin(async move {
            let mut response=inner.call(req).await?;
            response.headers_mut().append("X-custom-Header",HeaderValue::from_static("hello form custom header"));
                Ok(response)
        })
    }
}