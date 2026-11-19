use crate::{
    client::Senviok,
    error::SenviokError,
    models::{SendEmailRequest, SendMessageResponse, UploadedEmailAttachment},
};
use reqwest::Method;

pub struct Emails<'a> {
    pub(crate) client: &'a Senviok,
}

impl Emails<'_> {
    pub async fn upload_attachment(
        &self,
        filename: &str,
        content: Vec<u8>,
        content_type: &str,
    ) -> Result<UploadedEmailAttachment, SenviokError> {
        self.client
            .request_multipart(
                "/v1/emails/attachments",
                filename,
                content_type,
                content,
            )
            .await
    }

    pub async fn send(
        &self,
        request: SendEmailRequest,
    ) -> Result<SendMessageResponse, SenviokError> {
        self.client
            .request(Method::POST, "/v1/emails", Some(&request), None::<&()>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SendEmailRequest;
    use std::sync::{Arc, Mutex};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    async fn read_request(stream: &mut tokio::net::TcpStream) -> Vec<u8> {
        let mut request = Vec::new();
        let mut chunk = [0_u8; 4096];
        loop {
            let read = stream.read(&mut chunk).await.unwrap();
            assert!(read > 0, "connection closed before request completed");
            request.extend_from_slice(&chunk[..read]);
            if let Some(headers_end) = request
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
            {
                let headers = String::from_utf8_lossy(&request[..headers_end]);
                let length = headers
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .unwrap_or(0);
                if request.len() >= headers_end + 4 + length { break; }
            }
        }
        request
    }

    #[tokio::test]
    async fn uploads_then_sends_attachment_id() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let captured = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let captured_server = captured.clone();
        let server = tokio::spawn(async move {
            for response_body in [
                r#"{"id":"att_1","filename":"invoice.pdf","contentType":"application/pdf","sizeBytes":3,"bucket":"test","key":"test/att_1"}"#,
                r#"{"id":"msg_1"}"#,
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let request = read_request(&mut stream).await;
                captured_server.lock().unwrap().push(request);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    response_body.len(), response_body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        });

        let client = Senviok::new("svk_test_key").with_base_url(format!("http://{address}"));
        let uploaded = client
            .emails()
            .upload_attachment("invoice.pdf", b"pdf".to_vec(), "application/pdf")
            .await
            .unwrap();
        let sent = client
            .emails()
            .send(SendEmailRequest {
                from: "billing@example.com".into(),
                to: "customer@example.com".into(),
                subject: "Invoice".into(),
                attachment_ids: Some(vec![uploaded.id]),
                ..Default::default()
            })
            .await
            .unwrap();
        server.await.unwrap();

        assert_eq!(sent.id, "msg_1");
        let requests = captured.lock().unwrap();
        let upload = String::from_utf8_lossy(&requests[0]);
        assert!(upload.starts_with("POST /v1/emails/attachments "));
        assert!(upload.to_ascii_lowercase().contains("multipart/form-data"));
        assert!(upload.contains("name=\"file\""));
        assert!(upload.contains("\r\n\r\npdf\r\n--"));
        let send = String::from_utf8_lossy(&requests[1]);
        assert!(send.starts_with("POST /v1/emails "));
        assert!(send.contains(r#""attachmentIds":["att_1"]"#));
    }
}
