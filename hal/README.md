# Datasheet? TL;DR 

Block diagram on Page 23 / Section 4

* Pins are GPIO by default but can be assigned to peripheral functions A-H via PINCFG and PMUX.  Table showing combinations on Page 33 / Section 7.
* Configurable bus matrix links various peripherals. *AHB-APB* Bridges A-C are controlled by PAC instances 0-2.  It can write-protect a peripheral.
* DSU provides debugger related services.  Probably don't care about this for now.
* Clocked peripherals need to be pointed to a clock multiplexer which is in turn pointed to a clock source connected to a running clock.
* GCLK has 8 generators and m multiplexers

