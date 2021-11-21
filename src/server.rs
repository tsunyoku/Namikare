use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::utils::handle_whitespace;
use std::collections::HashMap;
use std::str::FromStr;
use std::io::Result;
use std::sync::Arc;

struct Router;
impl Router {

}

pub struct Request {
    _type: Option<String>,
    path: Option<String>,
    url: Option<String>,
    ver: Option<String>,
    body: Vec<u8>,

    // TODO: extras, elapsed

    code: Option<i16>,

    // TODO: fix key,val types of hashmaps
    headers: Option<HashMap<String, String>>, // TODO: case-insensitive
    args: Option<HashMap<String, String>>,
    resp_headers: Option<HashMap<String, String>>,
    files: Option<HashMap<String, String>>,

    // cus im stupid
    body_len: i32,
}

impl Request { // note: the code quality of this will suck ASS
    fn new(buffer: [u8; 1024]) -> Self {
        return Self {
            _type: Some("GET".to_owned()),
            path: Some("/".to_owned()),
            url: Some("".to_owned()),
            ver: Some("HTTP/1.1".to_owned()),
            body: buffer.to_vec(),
            code: Some(404.to_owned()),
            headers: None,
            args: None,
            resp_headers: None,
            files: None,
            body_len: buffer.len() as i32,
        }
    }

    fn handle_headers(mut self) -> Vec<HashMap<String, String>> { // actually handles args too trolley
        let headers = String::from_utf8_lossy(&self.body);

        let mut split = headers.split("\n");
        let char_sep = split.next().unwrap();
        let mut term = char_sep.split(" ");

        self._type = Some(term.next().unwrap().to_owned());
        self.path = Some(term.next().unwrap().to_owned());
        
        let ver = term.next().unwrap().to_owned();
        let mut var_split = ver.split("/");
        let mut http_prefix = var_split.next().unwrap().to_owned();
        let http_ver = var_split.next().unwrap().to_owned();

        // cursed.
        http_prefix.push_str("/");
        http_prefix.push_str(&http_ver);
        self.ver = Some(http_prefix.to_owned());

        let mut args_dict = HashMap::new();
        let mut unwrapped_ver =  self.ver.unwrap();
        let mut unwrapped_path = self.path.unwrap();
        if unwrapped_path.contains("?") {
            let mut terminator = unwrapped_path.split("?");

            self.path = Some(terminator.next().unwrap().to_owned());
            let mut args = terminator.next().unwrap().to_owned();

            let mut args_list = args.split("&");
            for arg in args_list {
                let mut kp = arg.split("=");

                args_dict.insert(
                    kp.next().unwrap().to_string(),
                    kp.next().unwrap().to_string().replace(" ", "")
                );
            }
        }

        let mut header_split = headers.split("\n");
        header_split.next();
        
        let mut header_dict = HashMap::new();
        for hp in header_split {
            let mut kp: Vec<String> = hp.split(":").map(|s| s.to_string()).collect();

            if kp.len() == 2 {
                let element_1 = kp[0].clone();
                let element_2 = kp[1].clone();

                header_dict.insert(
                    element_1,
                    handle_whitespace(element_2)
                );
            }
        }

        let mut new_args = Some(args_dict).clone().unwrap();
        let mut new_dict = Some(header_dict).clone().unwrap();

        self.args = Some(new_args);
        self.headers = Some(new_dict);

        self.body.drain(0..4);
        self.body_len = self.body.len() as i32;

        return vec![self.args.clone().unwrap(), self.headers.clone().unwrap()];
    }

    async fn parse_request(self) -> String {
        let ha_vec = self.handle_headers().to_owned();

        let args = ha_vec[0].clone();
        let headers = ha_vec[1].clone();

        let hardcoded_msg = "temporary hardcode";
        return format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            hardcoded_msg.len(),
            hardcoded_msg
        ).to_owned();
    }
}

// eek i am too used to OOP
pub struct Namikare {
    address: String,

    // TODO: decorators?
    before_serve: Option<fn()>,
    after_serve: Option<fn()>,
    
    // hm.
    routers: Vec<Router>,
}

impl Namikare {
    pub fn new(address: String) -> Arc<Self> {
        return Arc::new(Self { // ahahahah these nones i'm such a python user
            address: address,
            before_serve: None,
            after_serve: None,
            routers: Vec::new(),
        })
    }

    async fn handle_request(&self, buf: [u8; 1024]) -> String {
        let mut request = Request::new(buf);

        return request.parse_request().await;
    }

    pub async fn start(self: &Arc<Self>) {
        let listener = TcpListener::bind(&self.address).await.unwrap();
        println!("Taking requests on Namikare @ {}", self.address.clone());

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();

            let instance = self.clone();
            tokio::spawn(async move {
                let mut buffer = [0; 1024];

                loop {
                    let conn = match socket.read(&mut buffer).await {
                        Ok(0) => return,
                        Ok(conn) => conn,
                        Err(err) => {
                            println!("Namikare error: {}", err);
                            return;
                        }
                    };

                    // fine to handle request
                    let resp = instance.handle_request(buffer).await;
                    socket.write(resp.as_bytes()).await;
                    socket.flush().await;
                }
            });
        }
    }
}