use std::{cmp::min, collections::BTreeMap};

use ndarray::ArcArray1;
use vidmod_macros::*;
use vidmod_node::{FrameKind, FrameSingle, Node2MT, Node2T, PullPort, PushPort};

#[node_decl]
pub struct HSyncExtractor {
    threshold: u16,
    min_width: usize,
}

impl HSyncExtractor {
    #[node_new]
    pub fn new(params: BTreeMap<String, String>) -> Self {
        let threshold = params.get("threshold").unwrap().as_str().parse().unwrap();
        let min_width = params.get("min_width").unwrap().as_str().parse().unwrap();
        Self {
            threshold,
            min_width,
        }
    }
}

impl Node2T for HSyncExtractor {
    fn init(&mut self) {
        self.register_pushport("in", FrameKind::U16, self.min_width * 2);
        self.register_pullport("out", FrameKind::U16x1, 1);
    }

    fn tick(&mut self) -> bool {
        if self.inbuf_avail("in") == self.min_width * 2 {
            if self.outbuf_avail("out") == 1 {
                let buf = self.inbuf_peek("in", self.min_width * 2).unwrap_u16();
                let mut start = self.min_width;
                loop {
                    if let Some(idx) = buf.iter().skip(start).position(|x| *x < self.threshold) {
                        //println!("Testing sync {}->{}", start, start + idx);
                        if buf
                            .iter()
                            .skip(start + idx)
                            .take(100)
                            .filter(|x| **x < self.threshold)
                            .count()
                            > 90
                        {
                            //println!("Found sync!");
                            let line = self.inbuf_get("in", start + idx).unwrap_u16();
                            self.outbuf_put_single(
                                "out",
                                FrameSingle::U16x1(ArcArray1::from_iter(line.iter().copied())),
                            );
                            break;
                        } else {
                            start = start + idx + 1;
                        }
                    } else {
                        println!("Couldn't find sync- dumping entire line");
                        let line = self.inbuf_get("in", self.min_width * 2).unwrap_u16();
                        self.outbuf_put_single(
                            "out",
                            FrameSingle::U16x1(ArcArray1::from_iter(line.iter().copied())),
                        );
                        break;
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn finish(&mut self) -> bool {
        if self.inbuf_avail("in") > 0 {
            println!("Finishing up- dumping entire input buffer");
            let line = self.inbuf_get("in", self.inbuf_avail("in")).unwrap_u16();
            self.outbuf_put_single(
                "out",
                FrameSingle::U16x1(ArcArray1::from_iter(line.iter().copied())),
            );
        }
        // Now that we've dumped our buffer, there's no more work to do
        true
    }
}
