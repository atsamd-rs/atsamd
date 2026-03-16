<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:template match="/device/peripherals/peripheral[name='EVSYS']/registers/cluster[name='CHANNEL[%s]']/name">
    <name>CHANNELS[%s]</name>
  </xsl:template>
</xsl:stylesheet>
