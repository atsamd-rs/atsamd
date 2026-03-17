<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:template match="/device/peripherals/peripheral[name='EVSYS']/registers/cluster[name='CHANNEL[%s]']/name">
    <name>CHANNELS[%s]</name>
  </xsl:template>

  <!--
  Since PIC32CXSG SVDs are essentially a rebrand of SAMx5x, and have the exact same peripherals, remove this field which
  ends up causing different code-gen with svd2rust
  -->
  <xsl:template match="@*|node()">
        <xsl:copy>
            <xsl:apply-templates select="@*|node()"/>
        </xsl:copy>
    </xsl:template>
    <xsl:template match="modifiedWriteValues[text()='oneToClear']"/>

    <!--
  EIC in SVD has Config0 and Config1, instead, convert it into an array of Config0, to match the layout of SAMx5x EIC
  -->
</xsl:stylesheet>
