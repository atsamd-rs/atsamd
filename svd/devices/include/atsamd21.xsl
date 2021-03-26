<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- The DMAC trigger sources in the original SVD only have the 0=disabled
  enumeration value -->
  <xsl:template match="/device/peripherals/peripheral[name='DMAC']/registers/register[name='CHCTRLB']/fields/field[name='TRIGSRC']/enumeratedValues">
    <enumeratedValues>
      <xsl:copy-of select="./name"/>
      <xsl:copy-of select="./enumeratedValue"/>
      <enumeratedValue>
        <name>SERCOM0_RX</name>
        <description>SERCOM0 RX Trigger</description>
        <value>0x01</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM0_TX</name>
        <description>SERCOM0 TX Trigger</description>
        <value>0x02</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_RX</name>
        <description>SERCOM1 RX Trigger</description>
        <value>0x03</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_TX</name>
        <description>SERCOM1 TX Trigger</description>
        <value>0x04</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_RX</name>
        <description>SERCOM2 RX Trigger</description>
        <value>0x05</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_TX</name>
        <description>SERCOM2 TX Trigger</description>
        <value>0x06</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_RX</name>
        <description>SERCOM3 RX Trigger</description>
        <value>0x07</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_TX</name>
        <description>SERCOM3 TX Trigger</description>
        <value>0x08</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_RX</name>
        <description>SERCOM4 RX Trigger</description>
        <value>0x09</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_TX</name>
        <description>SERCOM4 TX Trigger</description>
        <value>0x0A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_RX</name>
        <description>SERCOM5 RX Trigger</description>
        <value>0x0B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_TX</name>
        <description>SERCOM5 TX Trigger</description>
        <value>0x0C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_OVF</name>
        <description>TCC0 Overflow Trigger</description>
        <value>0x0D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC0</name>
        <description>TCC0 Match/Compare 0 Trigger</description>
        <value>0x0E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC1</name>
        <description>TCC0 Match/Compare 1 Trigger</description>
        <value>0x0F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC2</name>
        <description>TCC0 Match/Compare 2 Trigger</description>
        <value>0x10</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC3</name>
        <description>TCC0 Match/Compare 3 Trigger</description>
        <value>0x11</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_OVF</name>
        <description>TCC1 Overflow Trigger</description>
        <value>0x12</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC0</name>
        <description>TCC1 Match/Compare 0 Trigger</description>
        <value>0x13</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC1</name>
        <description>TCC1 Match/Compare 1 Trigger</description>
        <value>0x14</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_OVF</name>
        <description>TCC2 Overflow Trigger</description>
        <value>0x15</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC0</name>
        <description>TCC2 Match/Compare 0 Trigger</description>
        <value>0x16</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC1</name>
        <description>TCC2 Match/Compare 1 Trigger</description>
        <value>0x17</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_OVF</name>
        <description>TC3 Overflow Trigger</description>
        <value>0x18</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC0</name>
        <description>TC3 Match/Compare 0 Trigger</description>
        <value>0x19</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC1</name>
        <description>TC3 Match/Compare 1 Trigger</description>
        <value>0x1A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_OVF</name>
        <description>TC4 Overflow Trigger</description>
        <value>0x1B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC0</name>
        <description>TC4 Match/Compare 0 Trigger</description>
        <value>0x1C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC1</name>
        <description>TC4 Match/Compare 1 Trigger</description>
        <value>0x1D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_OVF</name>
        <description>TC5 Overflow Trigger</description>
        <value>0x1E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC0</name>
        <description>TC5 Match/Compare 0 Trigger</description>
        <value>0x1F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC1</name>
        <description>TC5 Match/Compare 1 Trigger</description>
        <value>0x20</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_OVF</name>
        <description>TC6 Overflow Trigger</description>
        <value>0x21</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC0</name>
        <description>TC6 Match/Compare 0 Trigger</description>
        <value>0x22</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC1</name>
        <description>TC6 Match/Compare 1 Trigger</description>
        <value>0x23</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_OVF</name>
        <description>TC7 Overflow Trigger</description>
        <value>0x24</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC0</name>
        <description>TC7 Match/Compare 0 Trigger</description>
        <value>0x25</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC1</name>
        <description>TC7 Match/Compare 1 Trigger</description>
        <value>0x26</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC_RESRDY</name>
        <description>ADC Result Ready Trigger</description>
        <value>0x27</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_EMPTY</name>
        <description>DAC Empty Trigger</description>
        <value>0x28</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_RX_0</name>
        <description>I2S RX 0 Trigger</description>
        <value>0x29</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_RX_1</name>
        <description>I2S RX 1 Trigger</description>
        <value>0x2A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_TX_0</name>
        <description>I2S TX 0 Trigger</description>
        <value>0x2B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_TX_1</name>
        <description>I2S TX 1 Trigger</description>
        <value>0x2C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_OVF</name>
        <description>TCC3 Overflow Trigger</description>
        <value>0x2D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC0</name>
        <description>TCC3 Match/Compare 0 Trigger</description>
        <value>0x2E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC1</name>
        <description>TCC3 Match/Compare 1 Trigger</description>
        <value>0x2F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC2</name>
        <description>Match/Compare 2 Trigger</description>
        <value>0x30</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC3</name>
        <description>Match/Compare 3 Trigger</description>
        <value>0x31</value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>

  <!-- Some register bits are erroneously write-only in the
  original SVD, remove the access field that marks it as such -->
  <xsl:template match="/device/peripherals/peripheral[name='PORT']/registers/register[name='PINCFG0_%s']/fields/field[name='DRVSTR']/access" />
  <xsl:template match="/device/peripherals/peripheral[name='RTC']/registers/cluster/register[name='CTRL']/fields/field[name='SWRST']/access" />

  <!-- Add PTCReserved to the Generic Clock Selection ID enum on the GCLK peripheral -->
  <xsl:template match="/device/peripherals/peripheral[name='GCLK']/registers/register[name='CLKCTRL']/fields/field[name='ID']/enumeratedValues">
    <enumeratedValues>
      <xsl:for-each select="./enumeratedValue">
        <xsl:copy-of select="."/>
      </xsl:for-each>
      <enumeratedValue>
        <name>PTC</name>
        <description>PTCReserved</description>
        <value>0x22</value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>

  <!-- Add PTC peripheral. This is a best guess based off Adafruit's FreeTouch repo. -->
  <xsl:template match="/device/peripherals">
    <peripherals>
      <xsl:for-each select="./peripheral">
        <xsl:apply-templates select="."/>
      </xsl:for-each>
      <peripheral>
        <name>PTC</name>
        <version>1.0.0</version>
        <description>Peripheral Touch Controller</description>
        <groupName>PTC</groupName>
        <prependToName>PTC_</prependToName>
        <baseAddress>0x42004C00</baseAddress>
        <addressBlock>
          <offset>0</offset>
          <size>0x28</size>
          <usage>registers</usage>
        </addressBlock>
        <interrupt>
          <name>PTC</name>
          <value>26</value>
        </interrupt>
        <registers>
          <register>
            <name>CTRLA</name>
            <description>Control A</description>
            <addressOffset>0x00</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>SWRST</name>
                <description>Software Reset</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>ENABLE</name>
                <description>Enable</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>RUNSTDBY</name>
                <description>Run in Standby</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>CTRLB</name>
            <description>Control B</description>
            <addressOffset>0x01</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>SYNCFLAG</name>
                <description>Synchronisation flag</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <!-- Maybe this is like REFCTRL in ADC? -->
            <name>UNK4C04</name>
            <description>Unknown Register 0x42004C04</description>
            <addressOffset>0x04</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>CTRLC</name>
            <description>Control C</description>
            <addressOffset>0x05</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>INIT</name>
                <description>Initialize</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTENCLR</name>
            <description>Interrupt Enable Clear</description>
            <addressOffset>0x08</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>EOC</name>
                <description>Interrupt on end of comparison</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>WCO</name>
                <description>Watch crystal oscillator interrupt</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTENSET</name>
            <description>Interrupt Enable Set</description>
            <addressOffset>0x09</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>EOC</name>
                <description>Interrupt on end of comparison</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>WCO</name>
                <description>Watch crystal oscillator interrupt</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTFLAG</name>
            <description>Interrupt Flag Status and Clear</description>
            <addressOffset>0x0A</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>EOC</name>
                <description>Interrupt end of comparison</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>WCO</name>
                <description>Watch crystal oscillator interrupt</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>FREQCTRL</name>
            <description>Frequency Control</description>
            <addressOffset>0x0C</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>SAMPLEDELAY</name>
                <description>Sample delay</description>
                <bitOffset>0</bitOffset>
                <bitWidth>4</bitWidth>
                <enumeratedValues>
                  <name>FREQHOP</name>
                  <enumeratedValue>
                    <name>FREQHOP1</name>
                    <value>0x0</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP2</name>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP3</name>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP4</name>
                    <value>0x3</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP5</name>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP6</name>
                    <value>0x5</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP7</name>
                    <value>0x6</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP8</name>
                    <value>0x7</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP9</name>
                    <value>0x8</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP10</name>
                    <value>0x9</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP11</name>
                    <value>0xA</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP12</name>
                    <value>0xB</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP13</name>
                    <value>0xC</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP14</name>
                    <value>0xD</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP15</name>
                    <value>0xE</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP16</name>
                    <value>0xF</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
              <field>
                <name>FREQSPREADEN</name>
                <description>Enable frequency spread</description>
                <bitOffset>4</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>CONVCTRL</name>
            <description>Conversion control</description>
            <addressOffset>0x0D</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>ADCACCUM</name>
                <description>ADC Accumulator</description>
                <bitOffset>0</bitOffset>
                <bitWidth>3</bitWidth>
                <enumeratedValues>
                  <name>OVERSAMPLECOUNT</name>
                  <enumeratedValue>
                    <name>OVERSAMPLE1</name>
                    <description>1 sample per report</description>
                    <value>0x0</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE2</name>
                    <description>2 samples per report</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE4</name>
                    <description>4 samples per report</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE8</name>
                    <description>8 samples per report</description>
                    <value>0x3</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE16</name>
                    <description>16 samples per report</description>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE32</name>
                    <description>32 samples per report</description>
                    <value>0x5</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE64</name>
                    <description>64 samples per report</description>
                    <value>0x6</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
              <field>
                <name>CONVERT</name>
                <description>Start conversion</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>YSELECT</name>
            <description>Select Y line</description>
            <addressOffset>0x10</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>YMUX</name>
                <description>Y line selection MUX</description>
                <bitOffset>0</bitOffset>
                <bitWidth>16</bitWidth>
                <enumeratedValues>
                  <name>YMUXSelect</name>
                  <enumeratedValue>
                    <name>Y0</name>
                    <description>PTC Y0 Pin</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y1</name>
                    <description>PTC Y1 Pin</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y2</name>
                    <description>PTC Y2 Pin</description>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y3</name>
                    <description>PTC Y3 Pin</description>
                    <value>0x8</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y4</name>
                    <description>PTC Y4 Pin</description>
                    <value>0x10</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y5</name>
                    <description>PTC Y5 Pin</description>
                    <value>0x20</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y6</name>
                    <description>PTC Y6 Pin</description>
                    <value>0x40</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y7</name>
                    <description>PTC Y7 Pin</description>
                    <value>0x80</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y8</name>
                    <description>PTC Y8 Pin</description>
                    <value>0x100</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y9</name>
                    <description>PTC Y9 Pin</description>
                    <value>0x200</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y10</name>
                    <description>PTC Y10 Pin</description>
                    <value>0x400</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y11</name>
                    <description>PTC Y11 Pin</description>
                    <value>0x800</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y12</name>
                    <description>PTC Y12 Pin</description>
                    <value>0x1000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y13</name>
                    <description>PTC Y13 Pin</description>
                    <value>0x2000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y14</name>
                    <description>PTC Y14 Pin</description>
                    <value>0x4000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y15</name>
                    <description>PTC Y15 Pin</description>
                    <value>0x8000</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
            </fields>
          </register>
          <register>
            <name>XSELECT</name>
            <description>Select X line</description>
            <addressOffset>0x12</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>XMUX</name>
                <description>X line selection MUX</description>
                <bitOffset>0</bitOffset>
                <bitWidth>16</bitWidth>
                <enumeratedValues>
                  <name>XMUXSelect</name>
                  <enumeratedValue>
                    <name>X0</name>
                    <description>PTC X0 Pin</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X1</name>
                    <description>PTC X1 Pin</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X2</name>
                    <description>PTC X2 Pin</description>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X3</name>
                    <description>PTC X3 Pin</description>
                    <value>0x8</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X4</name>
                    <description>PTC X4 Pin</description>
                    <value>0x10</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X5</name>
                    <description>PTC X5 Pin</description>
                    <value>0x20</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X6</name>
                    <description>PTC X6 Pin</description>
                    <value>0x40</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X7</name>
                    <description>PTC X7 Pin</description>
                    <value>0x80</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X8</name>
                    <description>PTC X8 Pin</description>
                    <value>0x100</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X9</name>
                    <description>PTC X9 Pin</description>
                    <value>0x200</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X10</name>
                    <description>PTC X10 Pin</description>
                    <value>0x400</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X11</name>
                    <description>PTC X11 Pin</description>
                    <value>0x800</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X12</name>
                    <description>PTC X12 Pin</description>
                    <value>0x1000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X13</name>
                    <description>PTC X13 Pin</description>
                    <value>0x2000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X14</name>
                    <description>PTC X14 Pin</description>
                    <value>0x4000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X15</name>
                    <description>PTC X15 Pin</description>
                    <value>0x8000</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
            </fields>
          </register>
          <register>
            <name>YSELECTEN</name>
            <description>Enable Y lines</description>
            <addressOffset>0x14</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>Y0EN</name>
                <description>PTC Y0 pin enable</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y1EN</name>
                <description>PTC Y1 pin enable</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y2EN</name>
                <description>PTC Y2 pin enable</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y3EN</name>
                <description>PTC Y3 pin enable</description>
                <bitOffset>3</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y4EN</name>
                <description>PTC Y4 pin enable</description>
                <bitOffset>4</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y5EN</name>
                <description>PTC Y5 pin enable</description>
                <bitOffset>5</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y6EN</name>
                <description>PTC Y6 pin enable</description>
                <bitOffset>6</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y7EN</name>
                <description>PTC Y7 pin enable</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y8EN</name>
                <description>PTC Y8 pin enable</description>
                <bitOffset>8</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y9EN</name>
                <description>PTC Y9 pin enable</description>
                <bitOffset>9</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y10EN</name>
                <description>PTC Y10 pin enable</description>
                <bitOffset>10</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y11EN</name>
                <description>PTC Y11 pin enable</description>
                <bitOffset>11</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y12EN</name>
                <description>PTC Y12 pin enable</description>
                <bitOffset>12</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y13EN</name>
                <description>PTC Y13 pin enable</description>
                <bitOffset>13</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y14EN</name>
                <description>PTC Y14 pin enable</description>
                <bitOffset>14</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y15EN</name>
                <description>PTC Y15 pin enable</description>
                <bitOffset>15</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>XSELECTEN</name>
            <description>Enable X lines</description>
            <addressOffset>0x16</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>X0EN</name>
                <description>PTC X0 pin enable</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X1EN</name>
                <description>PTC X1 pin enable</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X2EN</name>
                <description>PTC X2 pin enable</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X3EN</name>
                <description>PTC X3 pin enable</description>
                <bitOffset>3</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X4EN</name>
                <description>PTC X4 pin enable</description>
                <bitOffset>4</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X5EN</name>
                <description>PTC X5 pin enable</description>
                <bitOffset>5</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X6EN</name>
                <description>PTC X6 pin enable</description>
                <bitOffset>6</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X7EN</name>
                <description>PTC X7 pin enable</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X8EN</name>
                <description>PTC X8 pin enable</description>
                <bitOffset>8</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X9EN</name>
                <description>PTC X9 pin enable</description>
                <bitOffset>9</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X10EN</name>
                <description>PTC X10 pin enable</description>
                <bitOffset>10</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X11EN</name>
                <description>PTC X11 pin enable</description>
                <bitOffset>11</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X12EN</name>
                <description>PTC X12 pin enable</description>
                <bitOffset>12</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X13EN</name>
                <description>PTC X13 pin enable</description>
                <bitOffset>13</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X14EN</name>
                <description>PTC X14 pin enable</description>
                <bitOffset>14</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X15EN</name>
                <description>PTC X15 pin enable</description>
                <bitOffset>15</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>COMPCAP</name>
            <description>Compensation capacitor value</description>
            <addressOffset>0x18</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>VALUE</name>
                <description>Value</description>
                <bitOffset>0</bitOffset>
                <bitWidth>14</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTCAP</name>
            <description>Internal capacitor value</description>
            <addressOffset>0x1A</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>VALUE</name>
                <description>Value</description>
                <bitOffset>0</bitOffset>
                <bitWidth>6</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>SERRES</name>
            <description>Series resistor for PTC measurements</description>
            <addressOffset>0x1B</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>RESISTOR</name>
                <description>Resistor value</description>
                <bitOffset>0</bitOffset>
                <bitWidth>2</bitWidth>
                <enumeratedValues>
                  <name>RESVALUE</name>
                  <enumeratedValue>
                    <name>RES0</name>
                    <description>No series resistor</description>
                    <value>0x0</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>RES20K</name>
                    <description>20 kiloohm series resistor</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>RES50K</name>
                    <description>50 kiloohm series resistor</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>RES100K</name>
                    <description>100 kiloohm series resistor</description>
                    <value>0x3</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
            </fields>
          </register>
          <register>
            <name>RESULT</name>
            <description>Conversion result</description>
            <addressOffset>0x1C</addressOffset>
            <size>16</size>
            <access>read-only</access>
            <fields>
              <field>
                <name>RESULT</name>
                <description>Result of conversion</description>
                <bitOffset>0</bitOffset>
                <bitWidth>16</bitWidth>
                <access>read-only</access>
              </field>
            </fields>
          </register>
          <register>
            <name>BURSTMODE</name>
            <description>Enable burst or clear to send low power mode</description>
            <addressOffset>0x20</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>CTSLOWPOWEREN</name>
                <description>Enable clear to send low power mode</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>BURSTMODE</name>
                <description>Burst mode setting</description>
                <bitOffset>4</bitOffset>
                <bitWidth>4</bitWidth>
              </field>
            </fields>
          </register>
          <!-- The rest of these are unused or have unknown uses, but are included for the sake of completeness. -->
          <register>
            <name>WCOMODE</name>
            <description>Set WCO mode</description>
            <addressOffset>0x21</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>MODE</name>
                <description>Set WCO mode</description>
                <bitOffset>0</bitOffset>
                <bitWidth>3</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>WCOTHRESHOLDAL</name>
            <description>Set lower WCO threshold for port A</description>
            <addressOffset>0x24</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>WCOTHRESHOLDAH</name>
            <description>Set upper WCO threshold for port A</description>
            <addressOffset>0x25</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>WCOTHRESHOLDBL</name>
            <description>Set lower WCO threshold for port B</description>
            <addressOffset>0x26</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>WCOTHRESHOLDBH</name>
            <description>Set upper WCO threshold for port B</description>
            <addressOffset>0x27</addressOffset>
            <size>8</size>
          </register>
        </registers>
      </peripheral>
    </peripherals>
  </xsl:template>
</xsl:stylesheet>
