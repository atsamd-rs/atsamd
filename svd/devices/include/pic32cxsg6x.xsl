<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

    <!-- Sample count enumeration names are inconsistent on 6x series -->
    <xsl:template match="/device/peripherals/peripheral[starts-with(name, 'ADC')]/registers/register[name='AVGCTRL']/fields/field[name='SAMPLENUM']/enumeratedValues">
    <enumeratedValues>
    <name>SAMPLENUMSelect</name>
    <enumeratedValue>
        <name>1</name>
        <description>1 sample</description>
        <value>0x0</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>2</name>
        <description>2 samples</description>
        <value>0x1</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>4</name>
        <description>4 samples</description>
        <value>0x2</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>8</name>
        <description>8 samples</description>
        <value>0x3</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>16</name>
        <description>16 samples</description>
        <value>0x4</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>32</name>
        <description>32 samples</description>
        <value>0x5</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>64</name>
        <description>64 samples</description>
        <value>0x6</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>128</name>
        <description>128 samples</description>
        <value>0x7</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>256</name>
        <description>256 samples</description>
        <value>0x8</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>512</name>
        <description>512 samples</description>
        <value>0x9</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>1024</name>
        <description>1024 samples</description>
        <value>0xA</value>
    </enumeratedValue>
    </enumeratedValues>
    </xsl:template>

    <!-- PSZ field name is inconsistent on 6x series -->
    <xsl:template match="/device/peripherals/peripheral[name='NVMCTRL']/registers/register[name='PARAM']/fields/field[name='PSZ']/enumeratedValues">
    <enumeratedValues>
    <name>PSZSelect</name>
    <enumeratedValue>
        <name>8</name>
        <description>8 bytes</description>
        <value>0x0</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>16</name>
        <description>16 bytes</description>
        <value>0x1</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>32</name>
        <description>32 bytes</description>
        <value>0x2</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>64</name>
        <description>64 bytes</description>
        <value>0x3</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>128</name>
        <description>128 bytes</description>
        <value>0x4</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>256</name>
        <description>256 bytes</description>
        <value>0x5</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>512</name>
        <description>512 bytes</description>
        <value>0x6</value>
    </enumeratedValue>
    <enumeratedValue>
        <name>1024</name>
        <description>1024 bytes</description>
        <value>0x7</value>
    </enumeratedValue>
    </enumeratedValues>
    </xsl:template>
</xsl:stylesheet>
