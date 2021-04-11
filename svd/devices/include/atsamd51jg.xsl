<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <!-- CCBUF & DITHERBUF wrong offset & size for CCBUF_DITH4_MODE -->
  <xsl:template match="/device/peripherals/peripheral[name='TCC0']/registers/register[name='CCBUF_DITH4_MODE[%s]']/fields/field[name='DITHERBUF']/bitOffset">
    <bitOffset>0</bitOffset>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='TCC0']/registers/register[name='CCBUF_DITH4_MODE[%s]']/fields/field[name='DITHERBUF']/bitWidth">
    <bitWidth>4</bitWidth>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='TCC0']/registers/register[name='CCBUF_DITH4_MODE[%s]']/fields/field[name='CCBUF']/bitOffset">
    <bitOffset>4</bitOffset>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='TCC0']/registers/register[name='CCBUF_DITH4_MODE[%s]']/fields/field[name='CCBUF']/bitWidth">
    <bitWidth>20</bitWidth>
  </xsl:template>
</xsl:stylesheet>
