use log::trace;
use lunatic::net::TcpStream;

use super::{frame::Frame, message::Message, send::send_frame};

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct WebSocket {
    #[derivative(Debug = "ignore")]
    stream: TcpStream,
    state: WebSocketState,
}

#[derive(Debug, Copy, Clone)]
pub enum WebSocketState {
    Open,
    Closed,
}

impl WebSocket {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: WebSocketState::Open,
        }
    }

    pub fn next(&mut self) -> Result<Message, NextMessageError> {
        match self.state {
            WebSocketState::Open => match Message::next(self.stream.clone()) {
                Ok(msg) => {
                    match msg {
                        Message::Ping(ref payload) => {
                            send_frame(
                                self.stream.clone(),
                                Frame {
                                    fin: true,
                                    rsv1: false,
                                    rsv2: false,
                                    rsv3: false,
                                    op_code: super::frame::OpCode::Pong,
                                    decoded: payload.clone().unwrap_or_default(),
                                },
                            )
                            .expect("failed to send pong");
                        }
                        _ => {}
                    }
                    Ok(msg)
                }
                Err(e) => match e {
                    super::message::DecodeMessageError::ClientProtocolViolationError => {
                        Err(NextMessageError::ClientError)
                    }
                    super::message::DecodeMessageError::ClientSentCloseFrame => {
                        self.state = WebSocketState::Closed;
                        send_close_frame(self.stream.clone());
                        Err(NextMessageError::ConnectionClosed)
                    }
                },
            },
            WebSocketState::Closed => Err(NextMessageError::ConnectionClosed),
        }
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        match self.state {
            WebSocketState::Open => {
                send_close_frame(self.stream.clone());
            }
            WebSocketState::Closed => {}
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NextMessageError {
    #[error("malformed client")]
    ClientError,
    #[error("the connection has been closed")]
    ConnectionClosed,
}

fn send_close_frame(stream: TcpStream) {
    trace!("Sending close frame");
    send_frame(
        stream,
        Frame {
            fin: true,
            rsv1: false,
            rsv2: false,
            rsv3: false,
            op_code: super::frame::OpCode::Terminate,
            decoded: vec![],
        },
    )
    .expect("failed to send close frame");
}
