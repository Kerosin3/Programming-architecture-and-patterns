fn main() {
    let proto = Proto2::AuthInfo {
        username: "Alex".to_string(),
        password: "12345".to_string(),
    };
    let p2 = Protocol2 {
        auth: proto,
        mountpoint: "root_volume".to_string(),
    };
    let d1 = Downloader {
        filename: "file1".to_string(),
        protocol: p2,
    };
    println!("mountpoint is {}", d1.get_mount_point());
    //-------------------------------
    let proto = Proto1::AuthInfo(); // no authinfo
    let p2 = Protocol1 { auth: proto };
    let d1 = Downloader {
        filename: "file2".to_string(),
        protocol: p2,
    };
    // no method get mount point here!
}

struct Downloader<P: Protocol> {
    filename: String,
    protocol: P,
}
//common impl
impl<P: Protocol> Downloader<P> {
    fn get_filename(&self) -> String {
        self.filename.to_owned()
    }
    fn get_auth_info(&self) -> P::AuthInfo {
        self.protocol.get_auth_info()
    }
}
//impl for specific generic
impl Downloader<Protocol2> {
    fn get_mount_point(&self) -> String {
        self.protocol.mountpoint.to_owned()
    }
}
pub mod Proto1 {
    #[derive(Debug, Clone)]
    pub struct AuthInfo(); // no auth
}
pub mod Proto2 {
    #[derive(Debug, Clone)]
    pub struct AuthInfo {
        pub username: String,
        pub password: String,
    }
}
// no auth info
struct Protocol1 {
    auth: Proto1::AuthInfo,
}
//with auth
struct Protocol2 {
    auth: Proto2::AuthInfo,
    mountpoint: String,
}

trait Protocol {
    type AuthInfo;
    fn get_auth_info(&self) -> Self::AuthInfo;
}

impl Protocol for Protocol1 {
    type AuthInfo = Proto1::AuthInfo;

    fn get_auth_info(&self) -> Self::AuthInfo {
        self.auth.clone()
    }
}

impl Protocol for Protocol2 {
    type AuthInfo = Proto2::AuthInfo;

    fn get_auth_info(&self) -> Self::AuthInfo {
        self.auth.clone()
    }
}
