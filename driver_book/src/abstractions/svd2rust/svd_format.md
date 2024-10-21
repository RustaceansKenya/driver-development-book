# SVD




alternative :  
```yaml
device:
  name: ACME1234
  architecture: ARM Cortex-M4
  clock:
    main_oscillator: 16MHz
    pll:
      input: main_oscillator
      multiplier: 6
      divider: 2
  peripherals:
    - name: UART1
      base_address: 0x40001000
      interrupts:
        - name: UART1_RX
          number: 20
          priority: 2
        - name: UART1_TX
          number: 21
          priority: 2
      registers:
        - name: DATA
          offset: 0x00
          size: 32
          access: read-write
          fields:
            - name: RX_TX_DATA
              bits: [0-7]
            - name: PARITY_ERROR
              bits: [8]
              description: "0 = No error, 1 = Parity error"
        # More registers...
      timing_constraints:
        - description: "Wait at least 1 us after enabling UART before first transmission"
        - description: "Maximum baud rate change frequency: 10 kHz"
      dependencies:
        - "Changing system clock frequency affects UART baud rate"
        - "DMA channel 2 can be used for automatic data transfer"
  power_modes:
    - name: SLEEP
      wakeup_sources: [UART1_RX, RTC_ALARM]
    - name: DEEP_SLEEP
      wakeup_sources: [RTC_ALARM]
  # More device-specific details...
```