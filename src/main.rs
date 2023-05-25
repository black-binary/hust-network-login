mod encrypt;
use std::fs;
use std::time::Duration;
use std::{io, thread};

fn extract<'a>(text: &'a str, prefix: &'a str, suffix: &'a str) -> io::Result<&'a str> {
    let left = text.find(prefix);
    let right = text.find(suffix);
    if let (Some(l), Some(r)) = (left, right) {
        if l + prefix.len() < r {
            return Ok(&text[l + prefix.len()..r]);
        }
    }
    Err(io::ErrorKind::InvalidData.into())
}

fn login(username: &str, password: &str) -> io::Result<()> {
    let resp = minreq::get("http://www.baidu.com")
        .with_timeout(10)
        .send()
        .map_err(|e| {
            println!("baidu boom! {}", e);
            io::ErrorKind::ConnectionRefused
        })?;
    let resp = resp.as_str().map_err(|e| {
        println!("invalid resp format {}", e);
        io::ErrorKind::InvalidData
    })?;

    if !resp.contains("/eportal/index.jsp")
        && !resp.contains("<script>top.self.location.href='http://")
    {
        return Ok(());
    }

    let portal_ip = extract(
        resp,
        "<script>top.self.location.href='http://",
        "/eportal/index.jsp",
    )?;
    println!("portal ip: {}", portal_ip);

    let mac = extract(resp, "mac=", "&t=")?;
    println!("mac: {}", mac);

    let encrypt_pass = encrypt::encrypt_pass(format!("{}>{}", password, mac));

    let query_string = extract(resp, "/eportal/index.jsp?", "'</script>\r\n")?;
    println!("query_string: {}", query_string);

    let query_string = urlencoding::encode(query_string);

    let body = format!(
        "userId={}&password={}&service=&queryString={}&passwordEncrypt=true",
        username, encrypt_pass, query_string
    );

    let login_url = format!("http://{}/eportal/InterFace.do?method=login", portal_ip);

    let resp = minreq::post(login_url)
        .with_body(body)
        .with_header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .with_header("Accept", "*/*")
        .with_header("User-Agent", "hust-network-login")
        .with_timeout(10)
        .send()
        .map_err(|e| {
            println!("portal boom! {}", e);
            io::ErrorKind::ConnectionRefused
        })?;

    let resp = resp.as_str().map_err(|e| {
        println!("invalid login resp format {}", e);
        io::ErrorKind::InvalidData
    })?;

    println!("login resp: {}", resp);

    if resp.contains("success") {
        Ok(())
    } else {
        Err(io::ErrorKind::PermissionDenied.into())
    }
}

#[test]
fn login_test() {
    let _ = login("username", "password");
}

fn main() {
    let args = std::env::args();
    if args.len() <= 1 {
        panic!("give me your config filename, you idiot")
    }
    let path = args.last().unwrap();
    let s = String::from_utf8(fs::read(&path).unwrap()).unwrap();
    let mut lines = s.lines();
    let username = lines.next().unwrap().to_owned();
    let password = lines.next().unwrap().to_owned();
    loop {
        match login(&username, &password) {
            Ok(_) => {
                println!("login ok. awaiting...");
                thread::sleep(Duration::from_secs(15));
            }
            Err(e) => {
                println!("error! {}", e);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
