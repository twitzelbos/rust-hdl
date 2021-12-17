use crate::core::prelude::*;
use crate::widgets::dff::DFF;
use crate::widgets::soc::bus::LocalBusD;

#[derive(LogicBlock)]
pub struct MISOWidePort<const W: usize, const D: usize, const A: usize> {
    pub bus: LocalBusD<D, A>,
    pub port_in: Signal<In, Bits<W>>,
    pub strobe_in: Signal<In, Bit>,
    pub clock: Signal<In, Clock>,
    accum: DFF<Bits<W>>,
    address_active: DFF<Bit>,
    my_address: Constant<Bits<A>>,
    offset: Constant<Bits<W>>,
    shift: Constant<Bits<W>>,
    modulo: Constant<Bits<8>>,
    count: DFF<Bits<8>>,
    ready: DFF<Bit>,
}

impl<const W: usize, const D: usize, const A: usize> MISOWidePort<W, D, A> {
    pub fn new(addr: Bits<A>) -> Self {
        assert!(W > D);
        assert_eq!(W % D, 0);
        assert!(W / D < 256);
        Self {
            bus: Default::default(),
            port_in: Default::default(),
            strobe_in: Default::default(),
            clock: Default::default(),
            accum: Default::default(),
            address_active: Default::default(),
            my_address: Constant::new(addr),
            offset: Constant::new(D.into()),
            shift: Constant::new((W - D).into()),
            modulo: Constant::new((W / D).into()),
            count: Default::default(),
            ready: Default::default(),
        }
    }
}

impl<const W: usize, const D: usize, const A: usize> Logic for MISOWidePort<W, D, A> {
    #[hdl_gen]
    fn update(&mut self) {
        // Clock the internal flip flops
        self.accum.clk.next = self.clock.val();
        self.address_active.clk.next = self.clock.val();
        self.count.clk.next = self.clock.val();
        self.ready.clk.next = self.clock.val();
        // Latch prevention
        self.accum.d.next = self.accum.q.val();
        self.address_active.d.next = self.bus.addr.val() == self.my_address.val();
        self.count.d.next = self.count.q.val();
        self.bus.ready.next = false;
        // On the strobe in, load the new value into our accumulator
        if self.strobe_in.val() {
            self.accum.d.next = self.port_in.val();
            self.count.d.next = self.modulo.val();
        }
        self.bus.to_master.next = 0_usize.into();
        self.ready.d.next = self.count.q.val().any() & self.address_active.q.val();
        if self.address_active.q.val() {
            self.bus.to_master.next = self.accum.q.val().get_bits::<D>(self.shift.val().into());
            self.bus.ready.next = self.ready.q.val() & self.count.q.val().any();
            if self.bus.strobe.val() {
                self.accum.d.next = self.accum.q.val() << self.offset.val();
                self.count.d.next = self.count.q.val() - 1_usize;
            }
        }
    }
}

#[test]
fn test_local_in_wide_port_is_synthesizable() {
    let mut dev = MISOWidePort::<64, 16, 8>::new(53_u8.into());
    dev.bus.from_master.connect();
    dev.bus.addr.connect();
    dev.bus.strobe.connect();
    dev.clock.connect();
    dev.port_in.connect();
    dev.strobe_in.connect();
    dev.connect_all();
    let vlog = generate_verilog(&dev);
    println!("{}", vlog);
    yosys_validate("local_wide_in", &vlog).unwrap();
}
