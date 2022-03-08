use std::collections::{BTreeMap, VecDeque};

use ndarray::ArcArray2;
use vidmod_node::{Frame, FrameKind, PullFrame, PullPort, PushFrame, PushPort, TickNode};

#[derive(Debug)]
pub struct SyncExtractor {}

impl SyncExtractor {
    pub fn new(_params: BTreeMap<String, String>) -> Self {
        Self {}
    }
}

impl PullFrame for SyncExtractor {
    fn pull_frame(&mut self, port: &PullPort, count: usize) -> Frame {
        assert_eq!(count, 1);
        match port.name() {
            "out" => Frame::U8x2(VecDeque::from(vec![ArcArray2::<u8>::from_shape_vec(
                (1, 1),
                vec![0],
            )
            .unwrap()])),
            _ => panic!("Unknown port {}", port.name()),
        }
    }

    fn test_pull_port(&self, name: &str) -> bool {
        name == "out"
    }

    fn pull_port_kind(&self, name: &str) -> FrameKind {
        match name {
            "out" => FrameKind::U8x2,
            _ => panic!("Unknown port {}", name),
        }
    }
    fn ready_to_pull(&self, port: &PullPort) -> usize {
        match port.name() {
            "out" => 1,
            _ => panic!("Unknown port {}", port.name()),
        }
    }
}

impl PushFrame for SyncExtractor {
    fn push_frame(&mut self, port: &PushPort, frame: Frame) {
        match port.name() {
            "in" => {
                if let Frame::U16(a) = frame {
                    assert!(a == vec![0]);
                } else {
                    panic!("Pushed frame wrong type");
                }
            }
            _ => panic!("Unknown port {}", port.name()),
        }
    }

    fn test_push_port(&self, name: &str) -> bool {
        name == "in"
    }

    fn push_port_kind(&self, name: &str) -> FrameKind {
        match name {
            "in" => FrameKind::U16,
            _ => panic!("Unknown port {}", name),
        }
    }
    fn ready_to_push(&self, port: &PushPort) -> usize {
        match port.name() {
            "in" => 1,
            _ => panic!("Unknown port {}", port.name()),
        }
    }
}

impl TickNode for SyncExtractor {}
