<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <!-- Some register bits are erroneously write-only in the
  original SVD, remove the access field that marks it as such -->
  <xsl:template match="/device/peripherals/peripheral[name='PORT']/registers/register[name='PINCFG0_%s']/fields/field[name='DRVSTR']/access" />
  <xsl:template match="/device/peripherals/peripheral[name='RTC']/registers/cluster/register[name='CTRL']/fields/field[name='SWRST']/access" />

</xsl:stylesheet>
