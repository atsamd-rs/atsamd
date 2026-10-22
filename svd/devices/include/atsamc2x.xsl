<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <!-- The GCLK:SYNCBUSY:GENCTRL field is specified as 1 bit wide, but several
      devices include much wider enumeration values that don't correspond to the
      actual values expected for this field. -->
    <xsl:template match="/device/peripherals/peripheral[name='GCLK']/registers/register[name='SYNCBUSY']/fields/field[starts-with(name, 'GENCTRL')]/enumeratedValues"/>
</xsl:stylesheet>
