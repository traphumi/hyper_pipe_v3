use std::{fs::File, io::{Read, Seek, SeekFrom}, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Serving on 8000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();

        let req = String::from_utf8_lossy(&buf);
        let range = req.lines()
            .find(|l| l.starts_with("Range:"))
            .and_then(|l| l.split("=").nth(1))
            .and_then(|r| r.split('-').next())
            .and_then(|n| n.parse::<u64>().ok());

        let mut file = File::open("test.bin").unwrap();
        let size = file.metadata().unwrap().len();

        if let Some(start) = range {
            file.seek(SeekFrom::Start(start)).unwrap();
            let mut chunk = vec![0; 1024*1024];
            let n = file.read(&mut chunk).unwrap();
            let header = format!(
                "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\n\r\n",
                n
            );
            stream.write_all(header.as_bytes()).unwrap();
            stream.write_all(&chunk[..n]).unwrap();
        } else {
            let mut all = Vec::new();
            file.read_to_end(&mut all).unwrap();
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            stream.write_all(&all).unwrap();
        }
    }
}
