#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserId {
    #[prost(int64, tag = "1")]
    pub user_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nothing {
    #[prost(bool, tag = "1")]
    pub dummy: bool,
}
#[doc = r" Generated client implementations."]
pub mod auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " grpc-сервис проверки авторизации"]
    pub struct AuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuthServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::UserId>,
        ) -> Result<tonic::Response<super::Token>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/auth.AuthService/Create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::Token>,
        ) -> Result<tonic::Response<super::UserId>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/auth.AuthService/Check");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::Token>,
        ) -> Result<tonic::Response<super::Nothing>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/auth.AuthService/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AuthServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AuthServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AuthServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod auth_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AuthServiceServer."]
    #[async_trait]
    pub trait AuthService: Send + Sync + 'static {
        async fn create(
            &self,
            request: tonic::Request<super::UserId>,
        ) -> Result<tonic::Response<super::Token>, tonic::Status>;
        async fn check(
            &self,
            request: tonic::Request<super::Token>,
        ) -> Result<tonic::Response<super::UserId>, tonic::Status>;
        async fn delete(
            &self,
            request: tonic::Request<super::Token>,
        ) -> Result<tonic::Response<super::Nothing>, tonic::Status>;
    }
    #[doc = " grpc-сервис проверки авторизации"]
    #[derive(Debug)]
    pub struct AuthServiceServer<T: AuthService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AuthService> AuthServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for AuthServiceServer<T>
    where
        T: AuthService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/auth.AuthService/Create" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSvc<T: AuthService>(pub Arc<T>);
                    impl<T: AuthService> tonic::server::UnaryService<super::UserId> for CreateSvc<T> {
                        type Response = super::Token;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::UserId>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth.AuthService/Check" => {
                    #[allow(non_camel_case_types)]
                    struct CheckSvc<T: AuthService>(pub Arc<T>);
                    impl<T: AuthService> tonic::server::UnaryService<super::Token> for CheckSvc<T> {
                        type Response = super::UserId;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Token>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth.AuthService/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: AuthService>(pub Arc<T>);
                    impl<T: AuthService> tonic::server::UnaryService<super::Token> for DeleteSvc<T> {
                        type Response = super::Nothing;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Token>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: AuthService> Clone for AuthServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AuthService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthService> tonic::transport::NamedService for AuthServiceServer<T> {
        const NAME: &'static str = "auth.AuthService";
    }
}
