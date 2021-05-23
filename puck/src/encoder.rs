use std::io::Write;

use lunatic::net::TcpStream;

use crate::Response;

#[derive(Debug)]
pub struct Encoder {
    response: Response,
}

impl Encoder {
    pub fn new(response: Response) -> Self {
        Self { response }
    }
    pub fn write_tcp_stream(&mut self, mut stream: TcpStream) -> std::io::Result<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n",
            self.response.status, self.response.reason
        )?;
        let mut headers = self.response.headers.iter().collect::<Vec<_>>();
        headers.sort_unstable_by_key(|(h, _)| h.as_str());
        for (header, value) in headers {
            write!(stream, "{}: {}\r\n", header, value)?;
        }
        write!(stream, "\r\n")?;
        std::io::copy(&mut self.response.body, &mut stream)?;
        Ok(())
    }
}
