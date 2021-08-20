#[cfg(test)]
use futures::AsyncRead;
#[cfg(test)]
use async_std::io::ReadExt;

#[cfg(test)]
pub struct FakeReader {
    src: String,
    pos: usize,
    max_read: Option<usize>,
}


#[cfg(test)]
impl FakeReader {
    pub fn new_by_size(strng: String, size: usize) -> FakeReader {
        return FakeReader {
            src: strng,
            pos: 0,
            max_read: Option::Some(size),
        }
    }
    pub fn new(strng: String) -> FakeReader {
        return FakeReader {
            src: strng,
            pos: 0,
            max_read: Option::None,
        }
    }
}


#[cfg(test)]
impl AsyncRead for FakeReader {

    fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &mut [u8],
        ) -> std::task::Poll<std::io::Result<usize>> { 

            let mut to_read = self.src.len() - self.pos;

            if to_read > buf.len() {
                to_read = buf.len();
            }

            if to_read > self.max_read.unwrap_or(to_read) {
                to_read = self.max_read.unwrap_or(to_read);
            }

            if to_read == 0 {
                return std::task::Poll::Ready(Result::Ok(0));
            }

            for i in 0..to_read {
                buf[i] = self.src.as_bytes()[i + self.pos];
            }

            self.pos = self.pos + to_read;

            std::task::Poll::Ready(Result::Ok(to_read))
    }
}


#[test]
fn fake_reader_works() {

    async fn fake_reader_works_impl() {
        let mut fr = FakeReader::new("hi there".to_string());
        let mut buffer: [u8; 64] = [0; 64];

        let read_count = match fr.read(&mut buffer).await {
            Ok(r) => r,
            Err(e) => 0,
        };

        assert_eq!(read_count, 8);
        assert_eq!(buffer, "hi there".to_string().as_bytes());


    }

    fake_reader_works_impl();
}

