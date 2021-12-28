<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:include href="./ptc.xsl"/>

  <!-- Add enumeratedValue for PTC in GCLK CLKCTRL register's ID field -->
  <xsl:template match="/device/peripherals/peripheral[name='GCLK']/registers/register[name='CLKCTRL']/fields/field[name='ID']/enumeratedValues">
    <xsl:call-template name="PTC_GCLK_ID">
      <xsl:with-param name="PTC_GCLK_ID_NAME">PTC</xsl:with-param>
      <xsl:with-param name="PTC_GCLK_ID_DESC">PTC</xsl:with-param>
      <xsl:with-param name="PTC_GCLK_ID_VALUE">0x17</xsl:with-param>
    </xsl:call-template>
  </xsl:template>

  <!-- Add PTC peripheral -->
  <xsl:template match="/device/peripherals">
    <xsl:call-template name="PTC_PERIPHERAL">
      <xsl:with-param name="PTC_PERIPHERAL_BASEADDRESS">0x42004C00</xsl:with-param>
      <xsl:with-param name="PTC_PERIPHERAL_INTERRUPT_VALUE">18</xsl:with-param>
    </xsl:call-template>
  </xsl:template>
</xsl:stylesheet>