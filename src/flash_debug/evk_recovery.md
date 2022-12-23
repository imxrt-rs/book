# Recovering an EVK

Sometimes, your debug probe fails to connect to the i.MX RT processor on your
NXP EVK. Simple troubleshooting approaches, like resetting power, cannot resolve
the issue. As a result, you're unable to flash a known, good program to recover
the board.

If you're experiencing this problem, try these steps to recover your board. This
guide assumes

-   you're using a NXP EVK, or equivalent hardware, that has switches to control
    the processor boot order.
-   your processor typically executes out of on-board flash.
-   you have a known, good program to load onto your board's flash.

## Recovery steps

1.  Power off your board.
2.  Locate the boot mode switches on your board. Consult your board's
    documentation for the switch locations.
3.  Use the switches to change the boot mode to "serial downloader." Consult
    your board's documentation to understand possible boot mode configurations.
4.  Power on your board. Observe that the program stored in the board's flash is
    not executing.
5.  Use your normal process to flash a known, good program. This step should now
    succeed.
6.  Power off your board.
7.  Revert the boot mode switches back to their previous configuration.
8.  Power on your board. Observe that the known, good program executes from
    flash.

After flashing the known, good program, you should be able to reprogram the
board without changing the boot mode.

## Discussion

By changing the boot mode to "serial downloader," we use the serial bootloader,
stored in the processor's boot ROM, as an ephemeral, known, good program. Your
probe should have better luck connecting to the processor when the processor is
simply waiting for commands over LPUART / USB.

If these steps are not sufficient to recover your board, your hardware may be in
a more troublesome state. Refer to your board's documentation for additional
troubleshooting steps.
