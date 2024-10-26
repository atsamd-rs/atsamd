<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <!-- The GMAC::TIDM::ENIDn fields are all missing. -->
  <xsl:template match="/device/peripherals/peripheral[name='GMAC']/registers/register[name='TIDM[%s]']/fields">
    <fields>
      <xsl:copy-of select="./field"/>
      <field>
        <name>ENID</name>
        <description>Enable Copying of TID Matched Frames</description>
        <bitOffset>31</bitOffset>
        <bitWidth>1</bitWidth>
      </field>
    </fields>
  </xsl:template>
</xsl:stylesheet>
