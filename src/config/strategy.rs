use std::{future::Future, ops::Deref};

use super::{
    program_config::{LockHandleResponse, Program},
    AdtError, AsReq, Create, LockHandle, LockObject, Request, SAPClient, Source,
};
use crate::{config::SendWith, net::Destination};
use async_trait::async_trait;
use reqwest::Method;
pub struct DefaultStrategy {
    body: String,
    path: String,
    method: Method,
}
impl DefaultStrategy {
    pub fn new(body: String, path: String, method: Method) -> Self {
        DefaultStrategy { body, path, method }
    }
}
#[async_trait]
impl Request for DefaultStrategy {
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
impl SendWith for DefaultStrategy
// where
// T: LockHandle + Source + AsMut<T> + AsRef<T> + Sync + Send,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let config = self.as_req();
        let res = client.request(config).await;
        println!("{}", res.status());
        println!("{}", res.text().await.unwrap());
        // println!("{}", res.status());

        // self.source = res.text().await.ok();
        Ok(())
    }
}

pub struct LockStrategy {
    body: String,
    method: Method,
    lock_obj: LockObject,
}
impl AsRef<LockStrategy> for LockStrategy
// where
//     T: LockHandle,
{
    fn as_ref(&self) -> &LockStrategy {
        self.as_ref()
    }
}
impl LockStrategy
// where
//     T: LockHandle,
{
    pub fn new(body: String, method: Method, lock_obj: LockObject) -> Self {
        LockStrategy {
            body,
            method,
            lock_obj,
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
impl Request for LockStrategy
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
impl SendWith for LockStrategy
// where
//     T: LockHandle + Source<'a> + AsMut<T> + AsRef<T> + Sync + Send,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        client.set_stateful(true);

        let lock_handle_res = client.lock(&self.lock_obj).await;

        let xml = lock_handle_res.text().await.unwrap();
        // println!("{:?}", &xml);
        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        self.lock_obj
            .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);

        let req = self.as_req();
        let res = client.request(req).await;
        println!("{}", res.status());
        println!("{}", res.text().await.unwrap());
        client.unlock(&self.lock_obj).await;
        // println!("{}", res.text().await.unwrap());
        Ok(())
    }
}
pub struct CopyToSysStrategy<'a, T>
// where
//     T: Send + Sync,
// U: SendWith<'a> + Send + Sync,
{
    lock_object: LockObject,
    obj: &'a T,
    destination: Destination, // update_source_req: Option<Box<(dyn SendWith<'a> + 'a)>>, // update_source_req: Option<Box<dyn SendWith<'a>>>, // xxx: Box<dyn SendWith<'a>>,
}
impl<'a, T> CopyToSysStrategy<'a, T>
where
    T: Source + AsMut<T> + AsRef<T> + Sync + Send,
    // U: SendWith<'a> + Send + Sync + 'a,
{
    pub fn new(
        lock_object: LockObject,
        obj: &'a T,
        destination: Destination,
    ) -> CopyToSysStrategy<'a, T> {
        CopyToSysStrategy {
            obj,
            lock_object,
            destination, // update_source_req: None, // update_source_req: None,
        }
    }
}

#[async_trait]
impl<'a, T> SendWith for CopyToSysStrategy<'a, T>
where
    T: Source + Create + AsMut<T> + AsRef<T> + Sync + Send,
    // U: SendWith<'a> + 'a,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>
// where
    //     T: LockHandle + Source + AsMut<T> + AsRef<T> + Sync + Send,
    {
        // let lock_handle_res = client.lock(self.obj).await;

        // let xml = lock_handle_res.text().await.unwrap();

        // let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        let obj = self.obj.as_ref();
        // self.obj
        //     .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);
        // self.source = &self.obj.source();
        let get_source_req = obj.source();
        let ob = get_source_req.as_ref();
        let boxed = Box::new(ob);

        let res = client.request(boxed).await;
        let text = &res.text().await.unwrap();
        println!("{}", text);
        println!("{:?}", &self.destination);
        client.set_destination(&self.destination);
        client.clear_session();
        client.set_stateful(true);
        let res = client.request(self.obj.create().as_req()).await;
        println!("{}", res.status());
        println!("{}", res.text().await.unwrap());
        // let mut req: Box<dyn SendWith<'a> + 'a> = self.obj.update_source(text);
        self.obj.update_source(&text).send_with(client).await;
        // &Some(&self.obj.update_source(&res.text().await).unwrap()));
        // let val = self.update_source_req.unwrap();
        // self.update_source_req.unwrap().send_with(client);
        // self.update_source_req
        //     .as_mut()
        //     .unwrap()
        //     .send_with(client)
        //     .await?;
        // client.unlock(self.obj).await;
        // println!("{}", res.text().await.unwrap());
        Ok(())
    }
}

// #[async_trait]
// impl<'a, T> Request for CopyToSysStrategy<'a, T>
// where
//     T: LockHandle + Source + AsMut<T> + AsRef<T> + Sync + Send,
//     // U: SendWith<'a> + Sync + Send,
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
