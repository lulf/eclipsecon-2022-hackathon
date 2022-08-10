#![no_std]
#![no_main]
#![macro_use]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use btmesh_device::{BluetoothMeshModel, BluetoothMeshModelContext};
use btmesh_macro::{device, element};
use btmesh_models::generic::onoff::{GenericOnOffClient, GenericOnOffMessage, GenericOnOffServer};
use btmesh_nrf_softdevice::*;
use core::{cell::RefCell, future::Future};
use embassy_executor::{
    executor::Spawner,
    time::{Delay, Duration, Ticker, Timer},
};
use embassy_microbit::*;
use embassy_nrf::{
    buffered_uarte::{BufferedUarte, State},
    config::Config,
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pull},
    interrupt,
    interrupt::Priority,
    peripherals::{TIMER0, UARTE0},
    uarte, Peripherals,
};
use embassy_util::{select, Either, Forever};
use heapless::Vec;
use nrf_softdevice::{
    ble::{gatt_server, peripheral, Connection},
    raw, temperature_celsius, Flash, Softdevice,
};

extern "C" {
    static __storage: u8;
}

use defmt_rtt as _;
use panic_probe as _;

// Application must run at a lower priority than softdevice
fn config() -> Config {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    config
}

#[embassy_executor::main(config = "config()")]
async fn main(s: Spawner, p: Peripherals) {
    let board = Microbit::new(p);

    let mut driver = Driver::new("drogue", unsafe { &__storage as *const u8 as u32 }, 100);

    let mut device = Device::new(board.btn_a, board.btn_b);
    driver.run(&mut device).await.unwrap();
}

#[device(cid = 0x0003, pid = 0x0001, vid = 0x0001)]
pub struct Device {
    zero: ElementZero,
}

#[element(location = "left")]
struct ElementZero {
    btn_a: ButtonOnOff,
    btn_b: ButtonOnOff,
    display: DisplayOnOff,
}

impl Device {
    pub fn new(btn_a: Button, btn_b: Button, display: LedMatrix) -> Self {
        Self {
            zero: ElementZero::new(btn_a, btn_b, display),
        }
    }
}

impl ElementZero {
    fn new(btn_a: Button, btn_b: Button, display: LedMatrix) -> Self {
        Self {
            btn_a: ButtonOnOff::new(btn_a),
            btn_b: ButtonOnOff::new(btn_b),
            display: DisplayOnOff::new(display),
        }
    }
}

struct ButtonOnOff {
    button: Input<'static, AnyPin>,
}

impl ButtonOnOff {
    fn new(button: Input<'static, AnyPin>) -> Self {
        Self { button }
    }
}

impl BluetoothMeshModel<GenericOnOffClient> for ButtonOnOff {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericOnOffClient> + 'f;

    #[allow(clippy::await_holding_refcell_ref)]
    fn run<'run, C: BluetoothMeshModelContext<GenericOnOffClient> + 'run>(
        &'run mut self,
        ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                self.button.wait_for_falling_edge().await;
                defmt::info!("** button pushed");
            }
        }
    }
}

struct DisplayOnOff {
    display: LedMatrix,
}

impl DisplayOnOff {
    fn new(display: LedMatrix) -> Self {
        Self { display }
    }
}

impl BluetoothMeshModel<GenericOnOffServer> for DisplayOnOff {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericOnOffServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericOnOffServer> + 'run>(
        &'run mut self,
        ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                let _ = ctx.
                self.button.wait_for_falling_edge().await;
                defmt::info!("** button pushed");
            }
        }
    }
}
