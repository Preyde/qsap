use super::{
    behavior::{Create, Details, Source},
    AdtError, AsReq, LockHandle, LockHandleResponse, LockObject, Request, Response, Responses,
    SAPClient, SendWith, TryFromAsync,
};
use crate::net::Destination;
use async_trait::async_trait;
use reqwest::Method;
pub struct DefaultStrategy<T>
where
    T: Response + TryFromAsync<reqwest::Response>,
{
    body: String,
    path: String,
    method: Method,
    res: Option<T>,
}
impl<T> DefaultStrategy<T>
where
    T: Response + TryFromAsync<reqwest::Response>,
{
    pub fn new(body: String, path: String, method: Method) -> Self {
        DefaultStrategy {
            body,
            path,
            method,
            res: None,
        }
    }
}

impl<T> Request for DefaultStrategy<T>
where
    T: Response + TryFromAsync<reqwest::Response> + Sync + Send,
{
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
    fn get_method(&self) -> reqwest::Method {
        self.method.clone()
    }
}
#[async_trait]
impl<T> SendWith for DefaultStrategy<T>
where
    T: Response + TryFromAsync<reqwest::Response> + Sync + Send,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<Box<dyn Response>, AdtError> {
        let config = self.as_req();
        let res = client.request(config).await;
        if let Ok(result) = T::try_from_async(res).await {
            Ok(result)
        } else {
            Err(AdtError {
                details: String::new(),
            })
        }
    }
}

trait IntoSendWith<T> {
    fn into_send_with(self) -> Box<dyn SendWith>;
}
impl<T> IntoSendWith<T> for DefaultStrategy<T>
where
    T: Response + TryFromAsync<reqwest::Response> + 'static,
    // U: Sync + Send + 'static,
{
    fn into_send_with(self) -> Box<dyn SendWith> {
        Box::new(self) as Box<dyn SendWith>
    }
}

pub struct LockStrategy<T> {
    body: String,
    method: Method,
    lock_obj: LockObject,
    x: Option<T>,
}
impl<T> AsRef<LockStrategy<T>> for LockStrategy<T> {
    fn as_ref(&self) -> &LockStrategy<T> {
        self.as_ref()
    }
}
impl<T> LockStrategy<T> {
    pub fn new(body: String, method: Method, lock_obj: LockObject) -> Self {
        LockStrategy {
            body,
            method,
            lock_obj,
            x: None,
        }
    }
}

#[async_trait]
impl<T> Request for LockStrategy<T>
where
    T: Response + TryFromAsync<reqwest::Response> + Sync + Send,
{
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.lock_obj.get_path().unwrap()
    }
    fn get_method(&self) -> reqwest::Method {
        self.method.clone()
    }
}
#[async_trait]
impl<T> SendWith for LockStrategy<T>
where
    T: Response + TryFromAsync<reqwest::Response> + Sync + Send,
    Self: AsReq + Sync + Send,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<Box<dyn Response>, AdtError> {
        client.set_stateful(true);

        let lock_handle_res = client.lock(&self.lock_obj).await;

        let xml = lock_handle_res.text().await.unwrap();

        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        self.lock_obj
            .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);

        let req = self.as_req();
        let res = client.request(req).await;

        client.unlock(&self.lock_obj).await;

        let x = T::try_from_async(res).await;
        x
    }
}
pub struct CopyToSysStrategy<'a, T, U>
where
    T: Source + Create + AsRef<T> + Send + Sync,
{
    obj: &'a T,
    data: Option<U>,
    destination: Destination,
}
impl<'a, T, U> CopyToSysStrategy<'a, T, U>
where
    T: Create + Source + AsMut<T> + AsRef<T> + Sync + Send,
    U: Response + TryFromAsync<reqwest::Response>,
{
    pub fn new(obj: &'a T, destination: Destination) -> CopyToSysStrategy<'a, T, U> {
        CopyToSysStrategy {
            obj,
            data: None,
            destination,
        }
    }
}

#[async_trait]
impl<'a, T, U> SendWith for CopyToSysStrategy<'a, T, U>
where
    T: Source + Create + AsRef<T> + Sync + Send,
    U: Response + TryFromAsync<reqwest::Response>,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<Box<dyn Response>, AdtError> {
        let mut get_source_req = self.obj.source();
        let res = get_source_req.send_with(client).await?;

        let text = &res.get_text();

        client.set_destination(&self.destination);
        client.clear_session();
        client.set_stateful(true);
        let mut x = self.obj.create();

        let res = self.obj.create().send_with(client).await?;

        let x = self.obj.update_source(&text).send_with(client).await;
        x
    }
}

pub struct CopyTabToSysStrategy<'a, T, U>
where
    T: Details,
    U: Response + TryFromAsync<reqwest::Response>,
{
    obj: &'a T,
    data: Option<U>,
    destination: Destination,
}
impl<'a, T, U> CopyTabToSysStrategy<'a, T, U>
where
    T: Details,
    U: Response + TryFromAsync<reqwest::Response>,
{
    pub fn new(obj: &'a T, destination: &Destination) -> CopyTabToSysStrategy<'a, T, U> {
        CopyTabToSysStrategy {
            obj,
            data: None,
            destination: destination.clone(),
        }
    }
}
#[async_trait]
impl<'a, T, U> SendWith for CopyTabToSysStrategy<'a, T, U>
where
    T: Details + Sync + Send,
    U: Response + TryFromAsync<reqwest::Response>,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<Box<dyn Response>, AdtError> {
        client.set_content_type("text/xml");
        let res = self.obj.details().send_with(client).await.unwrap();
        let soap_res = res.get_value();

        if let Responses::Detail(detail) = soap_res {
            let detail_res = detail.body.response;
            client.set_stateful(true);
            client.clear_session();
            client.set_destination(&self.destination);

            Ok(self
                .obj
                .update_details(&detail_res.dd02v, &detail_res.dd09l, &detail_res.fields)
                .send_with(client)
                .await?)
        } else {
            Err(AdtError::new("Could not get table details from system"))
        }
    }
}
