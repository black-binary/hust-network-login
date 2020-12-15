use std::fs;
use std::time::Duration;
use std::{io, thread};

fn login(username: &str, password: &str) -> io::Result<()> {
    let resp = minreq::get("http://www.baidu.com")
        .with_timeout(30)
        .send()
        .map_err(|e| {
            println!("baidu boom! {}", e);
            io::ErrorKind::ConnectionRefused
        })?;
    let resp = resp.as_str().map_err(|e| {
        println!("invalid resp format {}", e);
        io::ErrorKind::InvalidData
    })?;
    let begin_str = "<script>top.self.location.href='http://192.168.50.3:8080/eportal/index.jsp?";
    let end_str = "'</script>\r\n";

    let query_string = resp.strip_prefix(begin_str);
    if query_string.is_none() {
        println!("login ok");
        return Ok(());
    }
    let query_string = query_string.unwrap();
    let query_string = query_string.strip_suffix(end_str);
    if query_string.is_none() {
        println!("wtf?");
        return Err(io::ErrorKind::InvalidData.into());
    }

    let query_string = query_string.unwrap();
    println!("query_string: {}", query_string);
    let query_string = urlencoding::encode(&query_string);

    let body = format!(
        "userId={}&password={}&service=&queryString={}&passwordEncrypt=false",
        username, password, query_string
    );

    let resp = minreq::post("http://192.168.50.3:8080/eportal/InterFace.do?method=login")
        .with_body(body)
        .with_header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .with_header("Accept", "*/*")
        .with_header("User-Agent", "hust-network-login")
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

    if resp.find("success").is_some() {
        Ok(())
    } else {
        Err(io::ErrorKind::PermissionDenied.into())
    }
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
