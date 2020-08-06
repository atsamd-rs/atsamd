<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- The NVMCTRL:PARAM:SEE field is specified as 1 bit wide, but several
    devices include much wider enumeration values that don't correspond to the
    actual values expected for this field. -->
  <xsl:template match="/device/peripherals/peripheral[name='NVMCTRL']/registers/register[name='PARAM']/fields/field[name='SEE']/enumeratedValues">
  </xsl:template>

  <!-- The SDHC0 peripheral on some devices contains reset values for certain
    fields that are wider than their corresponding storage. Replace the invalid
    values with the ones from the datasheet. -->
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='HC1R']/resetValue">
    0x00
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='HC1R_EMMC_MODE']/resetValue">
    0x00
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='SISR']/resetValue">
    0x00
  </xsl:template>

  <!-- The TCn::COUNTn::CTRLA::DMAOS fields are all missing. -->
  <xsl:template match="/device/peripherals/peripheral[name='TC0']/registers/cluster/register[name='CTRLA']/fields">
    <fields>
      <xsl:copy-of select="./field"/>
      <field>
        <name>DMAOS</name>
        <description>DMA One-Shot Trigger Mode</description>
        <bitOffset>15</bitOffset>
        <bitWidth>1</bitWidth>
      </field>
    </fields>
  </xsl:template>
</xsl:stylesheet>
