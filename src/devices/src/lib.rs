// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the THIRD-PARTY file.

//! Emulates virtual and hardware devices.
extern crate epoll;
extern crate libc;

extern crate dumbo;
#[macro_use]
extern crate logger;
extern crate net_gen;
extern crate polly;
extern crate rate_limiter;
extern crate virtio_gen;
extern crate vm_memory;

use rate_limiter::Error as RateLimiterError;
use std::io;

mod bus;
pub mod legacy;
pub mod virtio;

pub use self::bus::{Bus, BusDevice, Error as BusError};
use logger::{Metric, METRICS};
use virtio::AsAny;

pub type DeviceEventT = u16;

type Result<T> = std::result::Result<T, Error>;

pub trait EpollHandler: AsAny + Send {
    fn handle_event(&mut self, device_event: DeviceEventT, evset: epoll::Events) -> Result<()>;
}

// Function used for reporting error in terms of logging
// but also in terms of METRICS net event fails.
pub(crate) fn report_net_event_fail(err: Error) {
    error!("{:?}", err);
    METRICS.net.event_fails.inc();
}

#[derive(Debug)]
pub enum Error {
    FailedReadingQueue {
        event_type: &'static str,
        underlying: io::Error,
    },
    FailedReadTap,
    FailedSignalingUsedQueue(io::Error),
    RateLimited(RateLimiterError),
    PayloadExpected,
    UnknownEvent {
        device: &'static str,
        event: DeviceEventT,
    },
    IoError(io::Error),
    NoAvailBuffers,
}
