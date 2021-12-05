use super::{
    program_config::{LockHandleResponse, Program},
    AdtError, AsReq, LockHandle, Request, SAPClient, Source,
};
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
        Method::GET
    }
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let res = client.request(&self.as_req()).await;

        // println!("{}", res.status());

        // self.source = res.text().await.ok();
        Ok(())
    }
}

pub struct LockStrategy<'a, T>
where
    T: LockHandle,
{
    body: String,
    path: String,
    method: Method,
    obj_to_lock: &'a T,
}
impl<'a, T> AsRef<LockStrategy<'a, T>> for LockStrategy<'a, T>
where
    T: LockHandle,
{
    fn as_ref(&self) -> &LockStrategy<'a, T> {
        self.as_ref()
    }
}
impl<'a, T> LockStrategy<'a, T>
where
    T: LockHandle,
{
    pub fn new(body: String, path: String, method: Method, obj_to_lock: &'a T) -> Self {
        LockStrategy {
            body,
            path,
            method,
            obj_to_lock,
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
impl<'a, T> Request for LockStrategy<'a, T>
where
    T: LockHandle + std::marker::Sync,
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
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        client.set_stateful(true);

        let lock_handle_res = client.lock(self.obj_to_lock).await;

        let xml = lock_handle_res.text().await.unwrap();
        // println!("{:?}", &xml);
        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        // self.obj_to_lock
        //     .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);

        let req = self.as_req();
        let res = client.request(&req).await;

        client.unlock(self.obj_to_lock);
        // println!("{}", res.text().await.unwrap());
        Ok(())
    }
}

pub struct CopyToSysStrategy<'a, T> {
    body: String,
    path: String,
    obj: &'a T,
    method: Method,
}
impl<'a, T> CopyToSysStrategy<'a, T>
where
    T: LockHandle,
{
    pub fn new(body: String, path: String, method: Method, obj: &'a T) -> Self {
        CopyToSysStrategy {
            body,
            path,
            method,
            obj,
        }
    }
}
#[async_trait]
impl<'a, T> Request for CopyToSysStrategy<'a, T>
where
    T: LockHandle + Source + Request + Sync + Send,
{
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
    fn get_method(&self) -> reqwest::Method {
        Method::GET
    }
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let lock_handle_res = client.lock(self.obj).await;

        let xml = lock_handle_res.text().await.unwrap();
        // println!("{:?}", &xml);
        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        // self.obj
        //     .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);
        // self.source = &self.obj.source();
        let get_source_req = self.obj.source();
        let ob = get_source_req.as_ref();
        let boxed = Box::new(ob);
        // let res = client.request(&boxed).await;

        client.unlock(self.obj).await;
        // println!("{}", res.text().await.unwrap());
        Ok(())
        // let mut conf_get_source = ConfigGetProgramSource::new(&self.prog_name);

        // conf_get_source.send_with(client).await?;

        // client.set_destination(&Destination {
        //     host: String::from("https://hamerpiea.zalaris.de"),
        //     port: 443,
        //     sys_id: String::from("IEA"),
        //     uname: String::from("PFRANK"),
        //     passwd: String::from("Start1234$"),
        //     mandt: String::from("200"),
        //     lang: String::from("DE"),
        // });

        // // println!("{}", &conf_get_source.get_source().unwrap());
        // client.set_stateful(true);
        // client.clear_session();
        // ConfigCreateProgram::new(&self.prog_name, None, None)
        //     .send_with(client)
        //     .await?;

        // ConfigUpdateProgramSource::new(&self.prog_name, &conf_get_source.get_source().unwrap())
        //     .send_with(client)
        //     .await?;

        // Ok(())
    }
}
