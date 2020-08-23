<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:include href="include/common.xsl"/>

  <xsl:template match="/device/peripherals/peripheral[name='USB']/registers/cluster[name='DEVICE']/register[name='FSMSTATUS']/fields/field[name='FSMSTATE']/bitWidth">
    <bitWidth>7</bitWidth>
  </xsl:template>
</xsl:stylesheet>
