use rust_hdl_bsp_ok_xem6010::XEM6010;
use rust_hdl_core::prelude::*;
use rust_hdl_test_ok_common::prelude::*;
use rust_hdl_test_core::target_path;

#[test]
fn test_opalkelly_xem_6010_synth_wave() {
    let mut uut = OpalKellyWave::new::<XEM6010>();
    uut.hi.sig_in.connect();
    uut.hi.sig_out.connect();
    uut.hi.sig_inout.connect();
    uut.hi.sig_aa.connect();
    uut.connect_all();
    rust_hdl_bsp_ok_xem6010::synth::synth_obj(uut, target_path!("xem_6010/wave"));
}
