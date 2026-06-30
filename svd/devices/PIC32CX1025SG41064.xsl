<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:include href="include/common.xsl"/>
    <xsl:include href="include/pic32cxsg.xsl"/>

    <!-- SG41064 specifically is missing its EIC Config1 register, add it here -->
    <!-- Snippet copied from SG41080 SVD -->
    <xsl:template match="/device/peripherals/peripheral[name='EIC']/registers">
        <xsl:copy>
        <xsl:apply-templates select="@*|node()"/>
        <register>
            <name>CONFIG1</name>
            <description>External Interrupt Sense Configuration</description>
            <addressOffset>0x20</addressOffset>
            <fields>
                <field>
                    <name>SENSE8</name>
                    <description>Input Sense Configuration 8</description>
                    <bitOffset>0</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE8Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN8</name>
                    <description>Filter Enable 8</description>
                    <bitOffset>3</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
                <field>
                    <name>SENSE9</name>
                    <description>Input Sense Configuration 9</description>
                    <bitOffset>4</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE9Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN9</name>
                    <description>Filter Enable 9</description>
                    <bitOffset>7</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
                <field>
                    <name>SENSE10</name>
                    <description>Input Sense Configuration 10</description>
                    <bitOffset>8</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE10Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN10</name>
                    <description>Filter Enable 10</description>
                    <bitOffset>11</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
                <field>
                    <name>SENSE11</name>
                    <description>Input Sense Configuration 11</description>
                    <bitOffset>12</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE11Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN11</name>
                    <description>Filter Enable 11</description>
                    <bitOffset>15</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
                <field>
                    <name>SENSE12</name>
                    <description>Input Sense Configuration 12</description>
                    <bitOffset>16</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE12Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN12</name>
                    <description>Filter Enable 12</description>
                    <bitOffset>19</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
                <field>
                    <name>SENSE13</name>
                    <description>Input Sense Configuration 13</description>
                    <bitOffset>20</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE13Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN13</name>
                    <description>Filter Enable 13</description>
                    <bitOffset>23</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
                <field>
                    <name>SENSE14</name>
                    <description>Input Sense Configuration 14</description>
                    <bitOffset>24</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE14Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN14</name>
                    <description>Filter Enable 14</description>
                    <bitOffset>27</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
                <field>
                    <name>SENSE15</name>
                    <description>Input Sense Configuration 15</description>
                    <bitOffset>28</bitOffset>
                    <bitWidth>3</bitWidth>
                    <writeConstraint>
                    <useEnumeratedValues>true</useEnumeratedValues>
                    </writeConstraint>
                    <enumeratedValues>
                    <name>SENSE15Select</name>
                    <enumeratedValue>
                        <name>NONE</name>
                        <description>No detection</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>RISE</name>
                        <description>Rising edge detection</description>
                        <value>1</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>FALL</name>
                        <description>Falling edge detection</description>
                        <value>2</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>BOTH</name>
                        <description>Both edges detection</description>
                        <value>3</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>HIGH</name>
                        <description>High level detection</description>
                        <value>4</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>LOW</name>
                        <description>Low level detection</description>
                        <value>5</value>
                    </enumeratedValue>
                    </enumeratedValues>
                </field>
                <field>
                    <name>FILTEN15</name>
                    <description>Filter Enable 15</description>
                    <bitOffset>31</bitOffset>
                    <bitWidth>1</bitWidth>
                </field>
            </fields>
        </register>
        </xsl:copy>
    </xsl:template>
</xsl:stylesheet>
