#[no_mangle]
pub extern fn default_handler() {
    loop {}
}

extern {
    fn __stack_top();
    fn reset_handler();
    fn nmi_handler();
    fn hardfault_handler();
    fn memmanage_handler();
    fn busfault_handler();
    fn usagefault_handler();
    fn svc_handler();
    fn debugmon_handler();
    fn pendsv_handler();
    fn systick_handler();
    fn wwdg_irqhandler();
    fn pvd_irqhandler();
    fn tamp_stamp_irqhandler();
    fn rtc_wkup_irqhandler();
    fn flash_irqhandler();
    fn rcc_irqhandler();
    fn exti0_irqhandler();
    fn exti1_irqhandler();
    fn exti2_irqhandler();
    fn exti3_irqhandler();
    fn exti4_irqhandler();
    fn dma1_stream0_irqhandler();
    fn dma1_stream1_irqhandler();
    fn dma1_stream2_irqhandler();
    fn dma1_stream3_irqhandler();
    fn dma1_stream4_irqhandler();
    fn dma1_stream5_irqhandler();
    fn dma1_stream6_irqhandler();
    fn adc_irqhandler();
    fn can1_tx_irqhandler();
    fn can1_rx0_irqhandler();
    fn can1_rx1_irqhandler();
    fn can1_sce_irqhandler();
    fn exti9_5_irqhandler();
    fn tim1_brk_tim9_irqhandler();
    fn tim1_up_tim10_irqhandler();
    fn tim1_trg_com_tim11_irqhandler();
    fn tim1_cc_irqhandler();
    fn tim2_irqhandler();
    fn tim3_irqhandler();
    fn tim4_irqhandler();
    fn i2c1_ev_irqhandler();
    fn i2c1_er_irqhandler();
    fn i2c2_ev_irqhandler();
    fn i2c2_er_irqhandler();
    fn spi1_irqhandler();
    fn spi2_irqhandler();
    fn usart1_irqhandler();
    fn usart2_irqhandler();
    fn usart3_irqhandler();
    fn exti15_10_irqhandler();
    fn rtc_alarm_irqhandler();
    fn otg_fs_wkup_irqhandler();
    fn tim8_brk_tim12_irqhandler();
    fn tim8_up_tim13_irqhandler();
    fn tim8_trg_com_tim14_irqhandler();
    fn tim8_cc_irqhandler();
    fn dma1_stream7_irqhandler();
    fn fmc_irqhandler();
    fn sdio_irqhandler();
    fn tim5_irqhandler();
    fn spi3_irqhandler();
    fn uart4_irqhandler();
    fn uart5_irqhandler();
    fn tim6_dac_irqhandler();
    fn tim7_irqhandler();
    fn dma2_stream0_irqhandler();
    fn dma2_stream1_irqhandler();
    fn dma2_stream2_irqhandler();
    fn dma2_stream3_irqhandler();
    fn dma2_stream4_irqhandler();
    fn eth_irqhandler();
    fn eth_wkup_irqhandler();
    fn can2_tx_irqhandler();
    fn can2_rx0_irqhandler();
    fn can2_rx1_irqhandler();
    fn can2_sce_irqhandler();
    fn otg_fs_irqhandler();
    fn dma2_stream5_irqhandler();
    fn dma2_stream6_irqhandler();
    fn dma2_stream7_irqhandler();
    fn usart6_irqhandler();
    fn i2c3_ev_irqhandler();
    fn i2c3_er_irqhandler();
    fn otg_hs_ep1_out_irqhandler();
    fn otg_hs_ep1_in_irqhandler();
    fn otg_hs_wkup_irqhandler();
    fn otg_hs_irqhandler();
    fn dcmi_irqhandler();
    fn hash_rng_irqhandler();
    fn fpu_irqhandler();
    fn uart7_irqhandler();
    fn uart8_irqhandler();
    fn spi4_irqhandler();
    fn spi5_irqhandler();
    fn spi6_irqhandler();
    fn sai1_irqhandler();
    fn ltdc_irqhandler();
    fn ltdc_er_irqhandler();
    fn dma2d_irqhandler();
}

#[no_mangle]
#[link_section = ".vectors"]
pub static VECTOR_TABLE: [Option<unsafe extern fn()>; 107] =
    [Some(__stack_top),
     Some(reset_handler),
     Some(nmi_handler),
     Some(hardfault_handler),
     Some(memmanage_handler),
     Some(busfault_handler),
     Some(usagefault_handler),
     None,
     None,
     None,
     None,
     Some(svc_handler),
     Some(debugmon_handler),
     None,
     Some(pendsv_handler),
     Some(systick_handler),
     // external interrupts
     Some(wwdg_irqhandler),
     Some(pvd_irqhandler),
     Some(tamp_stamp_irqhandler),
     Some(rtc_wkup_irqhandler),
     Some(flash_irqhandler),
     Some(rcc_irqhandler),
     Some(exti0_irqhandler),
     Some(exti1_irqhandler),
     Some(exti2_irqhandler),
     Some(exti3_irqhandler),
     Some(exti4_irqhandler),
     Some(dma1_stream0_irqhandler),
     Some(dma1_stream1_irqhandler),
     Some(dma1_stream2_irqhandler),
     Some(dma1_stream3_irqhandler),
     Some(dma1_stream4_irqhandler),
     Some(dma1_stream5_irqhandler),
     Some(dma1_stream6_irqhandler),
     Some(adc_irqhandler),
     Some(can1_tx_irqhandler),
     Some(can1_rx0_irqhandler),
     Some(can1_rx1_irqhandler),
     Some(can1_sce_irqhandler),
     Some(exti9_5_irqhandler),
     Some(tim1_brk_tim9_irqhandler),
     Some(tim1_up_tim10_irqhandler),
     Some(tim1_trg_com_tim11_irqhandler),
     Some(tim1_cc_irqhandler),
     Some(tim2_irqhandler),
     Some(tim3_irqhandler),
     Some(tim4_irqhandler),
     Some(i2c1_ev_irqhandler),
     Some(i2c1_er_irqhandler),
     Some(i2c2_ev_irqhandler),
     Some(i2c2_er_irqhandler),
     Some(spi1_irqhandler),
     Some(spi2_irqhandler),
     Some(usart1_irqhandler),
     Some(usart2_irqhandler),
     Some(usart3_irqhandler),
     Some(exti15_10_irqhandler),
     Some(rtc_alarm_irqhandler),
     Some(otg_fs_wkup_irqhandler),
     Some(tim8_brk_tim12_irqhandler),
     Some(tim8_up_tim13_irqhandler),
     Some(tim8_trg_com_tim14_irqhandler),
     Some(tim8_cc_irqhandler),
     Some(dma1_stream7_irqhandler),
     Some(fmc_irqhandler),
     Some(sdio_irqhandler),
     Some(tim5_irqhandler),
     Some(spi3_irqhandler),
     Some(uart4_irqhandler),
     Some(uart5_irqhandler),
     Some(tim6_dac_irqhandler),
     Some(tim7_irqhandler),
     Some(dma2_stream0_irqhandler),
     Some(dma2_stream1_irqhandler),
     Some(dma2_stream2_irqhandler),
     Some(dma2_stream3_irqhandler),
     Some(dma2_stream4_irqhandler),
     Some(eth_irqhandler),
     Some(eth_wkup_irqhandler),
     Some(can2_tx_irqhandler),
     Some(can2_rx0_irqhandler),
     Some(can2_rx1_irqhandler),
     Some(can2_sce_irqhandler),
     Some(otg_fs_irqhandler),
     Some(dma2_stream5_irqhandler),
     Some(dma2_stream6_irqhandler),
     Some(dma2_stream7_irqhandler),
     Some(usart6_irqhandler),
     Some(i2c3_ev_irqhandler),
     Some(i2c3_er_irqhandler),
     Some(otg_hs_ep1_out_irqhandler),
     Some(otg_hs_ep1_in_irqhandler),
     Some(otg_hs_wkup_irqhandler),
     Some(otg_hs_irqhandler),
     Some(dcmi_irqhandler),
     None,
     Some(hash_rng_irqhandler),
     Some(fpu_irqhandler),
     Some(uart7_irqhandler),
     Some(uart8_irqhandler),
     Some(spi4_irqhandler),
     Some(spi5_irqhandler),
     Some(spi6_irqhandler),
     Some(sai1_irqhandler),
     Some(ltdc_irqhandler),
     Some(ltdc_er_irqhandler),
     Some(dma2d_irqhandler)];
