use rust_hdl::core::prelude::*;
use rust_hdl::widgets::prelude::*;

#[derive(LogicBlock)]
struct SPITestMultiMaster {
    clock: Signal<In, Clock>,
    masters: [SPIMaster<64>; 3],
    addr: Signal<In, Bits<3>>,
    mux: MuxMasters<3, 3>,
    slave: SPISlave<64>,
}

impl SPITestMultiMaster {
    pub fn new(config: SPIConfig) -> Self {
        Self {
            clock: Default::default(),
            masters: array_init::array_init(|_| SPIMaster::new(config)),
            addr: Default::default(),
            mux: Default::default(),
            slave: SPISlave::new(config),
        }
    }
}

impl Logic for SPITestMultiMaster {
    #[hdl_gen]
    fn update(&mut self) {
        self.masters[0].clock.next = self.clock.val();
        SPIWiresMaster::join(&mut self.masters[0].wires, &mut self.mux.from_masters[0]);
        self.masters[1].clock.next = self.clock.val();
        SPIWiresMaster::join(&mut self.masters[1].wires, &mut self.mux.from_masters[1]);
        self.masters[2].clock.next = self.clock.val();
        SPIWiresMaster::join(&mut self.masters[2].wires, &mut self.mux.from_masters[2]);
        SPIWiresMaster::join(&mut self.mux.to_bus, &mut self.slave.wires);
        self.slave.clock.next = self.clock.val();
    }
}

#[test]
fn test_spi_mux() {
    let config = SPIConfig {
        clock_speed: 48_000_000,
        cs_off: true,
        mosi_off: true,
        speed_hz: 1_000_000,
        cpha: true,
        cpol: true,
    };
    let mut uut = TopWrap::new(SPITestMultiMaster::new(config));
    uut.uut.clock.connect();
    for i in 0..3 {
        uut.uut.masters[i].continued_transaction.connect();
        uut.uut.masters[i].start_send.connect();
        uut.uut.masters[i].data_outbound.connect();
        uut.uut.masters[i].bits_outbound.connect();
    }
    uut.uut.slave.data_outbound.connect();
    uut.uut.slave.start_send.connect();
    uut.uut.slave.continued_transaction.connect();
    uut.uut.slave.disabled.connect();
    uut.uut.slave.bits.connect();
    uut.connect_all();
    yosys_validate("spi_mux_multi_master", &generate_verilog(&uut)).unwrap();
}
