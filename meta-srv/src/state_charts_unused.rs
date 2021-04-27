#[cfg(test)]
mod traffic_tests {
    use super::*;
    machine!(
      #[derive(Clone,Debug,PartialEq)]
      enum TrafficLight {
        Green { count: u8 },
        Orange,
        Red,
        BlinkingOrange,
      }
    );

    pub mod prefix {
      #[derive(Clone,Debug,PartialEq)]
      pub struct Advance;
    }

    #[derive(Clone,Debug,PartialEq)]
    pub struct PassCar<'a, T> { count: u8, name: &'a T }

    #[derive(Clone,Debug,PartialEq)]
    pub struct Toggle;

    transitions!(TrafficLight,
      [
        (Green, prefix::Advance) => Orange,
        (Orange, prefix::Advance) => Red,
        (Red, prefix::Advance) => Green,
        (Green, PassCar<'a, T>) => [Green, Orange],
        (Green, Toggle) => BlinkingOrange,
        (Orange, Toggle) => BlinkingOrange,
        (Red, Toggle) => BlinkingOrange,
        (BlinkingOrange, Toggle) => Red
      ]
    );

    methods!(TrafficLight,
      [
        Green => get count: u8,
        Green => set count: u8,
        Green, Orange, Red, BlinkingOrange => default(false) fn working(&self) -> bool
      ]
    );

    impl Green {
      pub fn on_advance(self, _: prefix::Advance) -> Orange {
        Orange {}
      }

      pub fn on_pass_car<'a, T>(self, input: PassCar<'a, T>) -> TrafficLight {
        let count = self.count + input.count;
        if count >= 10 {
          println!("reached max cars count: {}", count);
          TrafficLight::orange()
        } else {
          TrafficLight::green(count)
        }
      }

      pub fn on_toggle(self, _: Toggle) -> BlinkingOrange {
        BlinkingOrange{}
      }

      pub fn working(&self) -> bool {
        true
      }
    }

    impl Orange {
      pub fn on_advance(self, _: prefix::Advance) -> Red {
        Red {}
      }

      pub fn on_toggle(self, _: Toggle) -> BlinkingOrange {
        BlinkingOrange{}
      }

      pub fn working(&self) -> bool {
        true
      }
    }

    impl Red {
      pub fn on_advance(self, _: prefix::Advance) -> Green {
        Green {
          count: 0
        }
      }

      pub fn on_toggle(self, _: Toggle) -> BlinkingOrange {
        BlinkingOrange{}
      }

      pub fn working(&self) -> bool {
        true
      }
    }

    impl BlinkingOrange {
      pub fn on_toggle(self, _: Toggle) -> Red {
        Red{}
      }

      pub fn working(&self) -> bool {
        false
      }
    }

    #[test]
    fn traffic_works() {
      use prefix::Advance;

      let mut t = TrafficLight::Green(Green { count: 0 });
      t = t.on_pass_car(PassCar { count: 1, name: &"test".to_string() });
      t = t.on_pass_car(PassCar { count: 2, name: &"test".to_string() });
      assert_eq!(t, TrafficLight::green(3));
      t = t.on_advance(Advance);
      //println!("trace: {}", t.print_trace());
      assert_eq!(t, TrafficLight::orange());

      t = t.on_advance(Advance);
      assert_eq!(t, TrafficLight::red());

      t = t.on_advance(Advance);
      assert_eq!(t, TrafficLight::green(0));
      t = t.on_pass_car(PassCar { count: 5, name: &"test".to_string() });
      assert_eq!(t, TrafficLight::green(5));
      t = t.on_pass_car(PassCar { count: 7, name: &"test".to_string() });
      assert_eq!(t, TrafficLight::orange());
      t = t.on_advance(Advance);
      assert_eq!(t, TrafficLight::red());
      t = t.on_pass_car(PassCar { count: 7, name: &"test".to_string() });
      assert_eq!(t, TrafficLight::error());
      t = t.on_advance(Advance);
      assert_eq!(t, TrafficLight::error());
    }
}

#[cfg(test)]
mod hello_tests {
    use super::*;

    pub trait Transitions {
      fn next(&mut self);
    }

    machine!(
      #[derive(Clone,Debug,PartialEq)]
      enum State {
        Start { pub x:u8 },
        End { pub x: u8, y: bool },
      }
    );

    #[derive(Clone,Debug,PartialEq)]
    pub struct Msg1;

    #[derive(Clone,Debug,PartialEq)]
    pub struct Msg2;

    transitions!(State,
      [
      (Start, Msg1) => End,
      (End, Msg1) => End
      ]
    );

    impl Start {
      pub fn on_msg1(self, _input: Msg1) -> End {
        End {
          x: self.x,
          y: true,
        }
      }
    }

    impl End {
      pub fn on_msg1(self, _input: Msg1) -> End {
        End {
          x: self.x,
          y: !self.y,
        }
      }
    }

    #[test]
    fn hello() {
      let start = State::start(0);
      let _end  = State::end(1, true);
      let _err = State::error();

      assert_eq!(start, State::Start(Start { x: 0 }));
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    machine!(
        enum HttpRequest {
            Initial,
            HasRequestLine {
                request: RequestLine,
            },
            HasHost {
                request: RequestLine,
                host: String,
            },
            HasLength {
                request: RequestLine,
                length: LengthInfo,
            },
            HasHostAndLength {
                request: RequestLine,
                host: Host,
                length: LengthInfo,
            },
            Request {
                request: RequestLine,
                host: Host,
            },
            RequestWithBody {
                request: RequestLine,
                host: Host,
                remaining: usize,
            },
            RequestWithChunks {
                request: RequestLine,
                host: Host,
                chunk: ChunkState,
            },
        }
    );

    #[derive(Clone, Debug, PartialEq)]
    pub struct RequestLine;

    #[derive(Clone, Debug, PartialEq)]
    pub struct HostHeader(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct LengthHeader(LengthInfo);

    #[derive(Clone, Debug, PartialEq)]
    pub struct HeaderEnd;

    pub type Host = String;

    #[derive(Clone, Debug, PartialEq)]
    pub enum LengthInfo {
        Length(usize),
        Chunked,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ChunkState;

    transitions!(HttpRequest,
      [
        (Initial, RequestLine) => HasRequestLine,
        (HasRequestLine, HostHeader) => HasHost,
        (HasRequestLine, LengthHeader) => HasLength,
        (HasHost, LengthHeader) => HasHostAndLength,
        (HasLength, HostHeader) => HasHostAndLength,
        (HasHost, HeaderEnd) => Request,
        (HasHostAndLength, HeaderEnd) => [RequestWithBody, RequestWithChunks]
      ]
    );

    methods!(HttpRequest,
      [
        HasHost, HasHostAndLength, Request,
          RequestWithBody, RequestWithChunks => get host: str
      ]
    );

    impl Initial {
      pub fn on_request_line(self, request: RequestLine) -> HasRequestLine {
        HasRequestLine { request }
      }
    }

    impl HasRequestLine {
      pub fn on_host_header(self, h: HostHeader) -> HasHost {
        let HostHeader(host) = h;

        HasHost {
          request: self.request,
          host,
        }
      }

      pub fn on_length_header(self, h: LengthHeader) -> HasLength {
        let LengthHeader(length) = h;

        HasLength {
          request: self.request,
          length,
        }
      }
    }

    impl HasHost {
      pub fn on_length_header(self, h: LengthHeader) -> HasHostAndLength {
        let LengthHeader(length) = h;

        HasHostAndLength {
          request: self.request,
          host: self.host,
          length,
        }
      }

      pub fn on_header_end(self, _: HeaderEnd) -> Request {
        Request {
          request: self.request,
          host: self.host,
        }
      }
    }

    impl HasLength {
      pub fn on_host_header(self, h: HostHeader) -> HasHostAndLength {
        let HostHeader(host) = h;

        HasHostAndLength {
          request: self.request,
          length: self.length,
          host,
        }
      }
    }

    impl HasHostAndLength {
      pub fn on_header_end(self, _: HeaderEnd) -> HttpRequest {
        match self.length {
          LengthInfo::Length(remaining) => HttpRequest::RequestWithBody(RequestWithBody {
            request: self.request,
            host: self.host,
            remaining
          }),
          LengthInfo::Chunked => {
            HttpRequest::RequestWithChunks(RequestWithChunks {
              request: self.request,
              host: self.host,
              chunk: ChunkState,
            })
          }
        }
      }
    }
}



