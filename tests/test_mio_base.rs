use std::thread;

use mio::{EventLoop, Handler};

struct BaseHandler;

impl Handler for BaseHandler {
    type Timeout = ();
    type Message = ();
}

#[test]
fn test_base() {
    let mut event_loop = EventLoop::<BaseHandler>::new().unwrap();
    event_loop.shutdown();
}

struct TimeoutHandler;

impl Handler for TimeoutHandler {
    type Timeout = u32;
    type Message = ();

    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, _: Self::Timeout) {
        event_loop.shutdown();
    }
}

#[test]
fn test_timeout() {
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.timeout_ms(100, 0).unwrap();
    event_loop.run(&mut TimeoutHandler).unwrap();
}

struct NotifyHandler;

impl Handler for NotifyHandler {
    type Timeout = ();
    type Message = u32;

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, _: Self::Message) {
        event_loop.shutdown();
    }
}

#[test]
fn test_notify() {
    let mut event_loop = EventLoop::new().unwrap();
    let sender = event_loop.channel();
    let child = thread::spawn(move || {
        sender.send(0).unwrap();
    });
    event_loop.run(&mut NotifyHandler).unwrap();
    child.join().unwrap();
}

struct TickHandler;

impl Handler for TickHandler {
    type Timeout = ();
    type Message = u32;

    fn tick(&mut self, event_loop: &mut EventLoop<Self>) {
        if !event_loop.is_running() {
            // Handle quit here.
            return;
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<Self>, _: Self::Message) {
        event_loop.shutdown();
    }
}

#[test]
fn test_tick() {
    let mut event_loop = EventLoop::new().unwrap();
    let sender = event_loop.channel();
    sender.send(0).unwrap();
    event_loop.run(&mut TickHandler).unwrap();
}
