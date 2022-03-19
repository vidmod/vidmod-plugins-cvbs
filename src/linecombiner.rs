use std::collections::BTreeMap;

use ndarray::ArcArray2;
use vidmod_macros::*;
use vidmod_node::{
    frame::{FrameKind, FrameSingle},
    Node2MT, Node2T, PullPort, PushPort,
};

#[node_decl]
pub struct LineCombiner {
    max_width: usize,
    count:     usize,
}

impl LineCombiner {
    #[node_new]
    pub fn new(params: BTreeMap<String, String>) -> Self {
        let max_width = params.get("max_width").unwrap().as_str().parse().unwrap();
        let count = params.get("count").unwrap().as_str().parse().unwrap();

        Self { max_width, count }
    }
}

impl Node2T for LineCombiner {
    fn init(&mut self) {
        self.register_pushport("in", FrameKind::U16x1, self.count);
        self.register_pullport("out", FrameKind::U16x2, 1);
    }

    fn tick(&mut self) -> bool {
        if self.inbuf_avail("in") == self.count {
            if self.outbuf_avail("out") == 1 {
                let mut out = ArcArray2::zeros((self.count, self.max_width));
                for i in 0..self.count {
                    let line = self.inbuf_get_single("in").unwrap_u16x1();
                    out.slice_mut(ndarray::s![i, ..line.len()])
                        .assign(&line.slice(ndarray::s![..]));
                }
                println!("Put frame out");
                self.outbuf_put_single("out", FrameSingle::U16x2(out));
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
            let mut out = ArcArray2::zeros((self.inbuf_avail("in"), self.max_width));
            for i in 0..self.inbuf_avail("in") {
                let line = self.inbuf_get_single("in").unwrap_u16x1();
                out.slice_mut(ndarray::s![i, ..line.len()])
                    .assign(&line.slice(ndarray::s![..]));
            }
            println!("Put frame out");
            self.outbuf_put_single("out", FrameSingle::U16x2(out));
        }
        // Now that we've dumped our buffer, there's no more work to do
        true
    }
}
