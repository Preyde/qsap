use std::{future::Future, ops::Deref};

use super::{
    program_config::{LockHandleResponse, Program},
    AdtError, AsReq, Create, DefaultResponse, LockHandle, LockObject, Request, Response, SAPClient,
    Source, TryFromAsync,
};
use crate::{config::SendWith, net::Destination};
use async_trait::async_trait;
use reqwest::Method;
pub struct DefaultStrategy<T, U>
where
    T: Response<U> + TryFromAsync<reqwest::Response>,
{
    body: String,
    path: String,
    method: Method,
    res: Option<T>,
    x: Option<U>, // res: T,
}
impl<T, U> DefaultStrategy<T, U>
where
    T: Response<U> + TryFromAsync<reqwest::Response>,
{
    pub fn new(body: String, path: String, method: Method) -> Self {
        DefaultStrategy {
            body,
            path,
            method,
            res: None, // res: None,
            x: None,
        }
    }
}

impl<T, U> Request for DefaultStrategy<T, U>
where
    T: Response<U> + TryFromAsync<reqwest::Response> + Sync + Send,
    U: Sync + Send,
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
impl<T, U> SendWith<T> for DefaultStrategy<T, U>
where
    T: Response<U> + TryFromAsync<reqwest::Response> + Sync + Send,
    Self: AsReq + Sync + Send, // where
                               //                                                                     // T: LockHandle + Source + AsMut<T> + AsRef<T> + Sync + Send
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<T, AdtError> {
        let config = self.as_req();
        let res = client.request(config).await;
        if let Ok(result) = T::try_from_async(res).await {
            Ok(result)
        } else {
            Err(AdtError {
                details: String::new(),
            })
        }
        // println!("{}", res.status());
        // println!("{}", res.text().await.unwrap());
        // println!("{}", res.status());

        // self.source = res.text().await.ok();
        // Ok(())
    }
}

trait IntoSendWith<T> {
    fn into_send_with(self) -> Box<dyn SendWith<T>>;
}
impl<T, U> IntoSendWith<T> for DefaultStrategy<T, U>
where
    T: Response<U> + TryFromAsync<reqwest::Response> + 'static,
    U: Sync + Send + 'static,
{
    fn into_send_with(self) -> Box<dyn SendWith<T>> {
        Box::new(self) as Box<dyn SendWith<T>>
    }
}

pub struct LockStrategy<T, U> {
    body: String,
    method: Method,
    lock_obj: LockObject,
    res: Option<Box<dyn Response<T>>>,
    x: Option<U>,
}
impl<T, U> AsRef<LockStrategy<T, U>> for LockStrategy<T, U>
// where
//     T: LockHandle,
{
    fn as_ref(&self) -> &LockStrategy<T, U> {
        self.as_ref()
    }
}
impl<T, U> LockStrategy<T, U>
// where
//     T: LockHandle,
{
    pub fn new(body: String, method: Method, lock_obj: LockObject) -> Self {
        LockStrategy {
            body,
            method,
            lock_obj,
            res: None,
            x: None,
        }
    }
}
// impl<'a, T> LockStrategy<'a, T>
// where
//     T: LockHandle + Sync,
// {
//     fn as_req(&self) -> Box<&dyn Request> {
//         Box::new(self as &dyn Request)
//     }
// }
#[async_trait]
impl<T, U> Request for LockStrategy<T, U>
where
    T: Response<U> + TryFromAsync<reqwest::Response> + Sync + Send,
    U: Sync + Send,
    // where
    //     T: LockHandle + std::marker::Sync + Send,
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
impl<T, U> SendWith<T> for LockStrategy<T, U>
where
    T: Response<U> + TryFromAsync<reqwest::Response> + Sync + Send, // where
    Self: AsReq + Sync + Send, //     T: LockHandle + Source<'a> + AsMut<T> + AsRef<T> + Sync + Send
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<T, AdtError> {
        client.set_stateful(true);

        let lock_handle_res = client.lock(&self.lock_obj).await;

        let xml = lock_handle_res.text().await.unwrap();
        println!("{:?}", &xml);
        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        self.lock_obj
            .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);

        let req = self.as_req();
        let res = client.request(req).await;
        // println!("{}", res.status());
        // println!("{}", res.text().await.unwrap());
        client.unlock(&self.lock_obj).await;

        let x = T::try_from_async(res).await;
        x
        // println!("{}", res.text().await.unwrap());
        // Ok(())
    }
}
pub struct CopyToSysStrategy<'a, T, U, V>
where
    T: Send + Sync,
    // U: SendWith<'a> + Send + Sync,
{
    // lock_object: LockObject,
    obj: &'a T,
    res: Option<Box<dyn Response<U>>>,
    data: Option<V>,
    destination: Destination, // update_source_req: Option<Box<(dyn SendWith<'a> + 'a)>>, // update_source_req: Option<Box<dyn SendWith<'a>>>, // xxx: Box<dyn SendWith<'a>>,
}
impl<'a, T, U, V> CopyToSysStrategy<'a, T, U, V>
where
    T: Source<U, V> + AsMut<T> + AsRef<T> + Sync + Send,
    U: Response<V> + TryFromAsync<reqwest::Response>, // U: SendWith<'a> + Send + Sync + 'a,
{
    pub fn new(
        // lock_object: LockObject,
        obj: &'a T,
        destination: Destination,
    ) -> CopyToSysStrategy<'a, T, U, V> {
        CopyToSysStrategy {
            obj,
            res: None,
            data: None,
            // lock_object,
            destination, // update_source_req: None, // update_source_req: None,
        }
    }
}

#[async_trait]
impl<'a, T, U, V> SendWith<U> for CopyToSysStrategy<'a, T, U, V>
where
    T: Source<U, V> + Create<U, V> + AsRef<T> + Sync + Send,
    U: Response<V> + TryFromAsync<reqwest::Response> + Sync + Send,
    V: Sync + Send,
    // U: SendWith<'a> + 'a,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<U, AdtError>
// where
    //     T: LockHandle + Source + AsMut<T> + AsRef<T> + Sync + Send,
    {
        let mut get_source_req = self.obj.source();
        let res = get_source_req.send_with(client).await?;
        // let res = client.request(Box::new(get_source_req.as_req())).await;
        let text = &res.get_text();
        // println!("{}", text);
        // println!("{:?}", &self.destination);
        client.set_destination(&self.destination);
        client.clear_session();
        client.set_stateful(true);

        let res = self.obj.create().send_with(client).await?;

        // let res = client.request(self.obj.create().as_req()).await;
        // println!("{}", res.status());
        // println!("{}", res.text().await.unwrap());
        self.obj.update_source(&text).send_with(client).await
        // U::try_from_async(res.).await
        // Ok(())
    }
}

// #[async_trait]
// impl<'a, T, U, V> Request for CopyToSysStrategy<'a, T, U, V>
// where
//     T: LockHandle + Source<U, V> + AsMut<T> + AsRef<T> + Sync + Send,
//     U: Response<V> + TryFromAsync<reqwest::Response>,
//     V: Sync + Send,
// {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
//     fn get_method(&self) -> reqwest::Method {
//         Method::GET
//     }
// }
//////////////////////////////////////////////////////////////////////////////////////////////
// async fn send_with(&'a mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//     // let lock_handle_res = client.lock(self.obj).await;

//     // let xml = lock_handle_res.text().await.unwrap();

//     // let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

//     let obj = self.obj.as_ref();
//     // self.obj
//     //     .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);
//     // self.source = &self.obj.source();
//     let get_source_req = obj.source();
//     let ob = get_source_req.as_ref();
//     let boxed = Box::new(ob);

//     let res = client.request(&boxed).await;

//     obj.update_source(&res.text().await.unwrap())
//         .send_with(client);

//     // client.unlock(self.obj).await;
//     // println!("{}", res.text().await.unwrap());
//     Ok(())
//     // let mut conf_get_source = ConfigGetProgramSource::new(&self.prog_name);

//     // conf_get_source.send_with(client).await?;

//     // client.set_destination(&Destination {
//     //     host: String::from("https://hamerpiea.zalaris.de"),
//     //     port: 443,
//     //     sys_id: String::from("IEA"),
//     //     uname: String::from("PFRANK"),
//     //     passwd: String::from("Start1234$"),
//     //     mandt: String::from("200"),
//     //     lang: String::from("DE"),
//     // });

//     // // println!("{}", &conf_get_source.get_source().unwrap());
//     // client.set_stateful(true);
//     // client.clear_session();
//     // ConfigCreateProgram::new(&self.prog_name, None, None)
//     //     .send_with(client)
//     //     .await?;

//     // ConfigUpdateProgramSource::new(&self.prog_name, &conf_get_source.get_source().unwrap())
//     //     .send_with(client)
//     //     .await?;

//     // Ok(())
// }
