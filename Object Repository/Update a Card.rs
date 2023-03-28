<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update a Card</name>
   <tag></tag>
   <elementGuidId>06ec8496-0450-47ec-8f0e-d8c9ff3b6939</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://api.trello.com/1/cards/${cardID}?key=6d6aecb7550d295e5b135eae3efaf11e&amp;token=9b8f6a0f594437e6acb100e7e51678a66bae408b8fc6d44c28d4f576a355db4e&amp;desc=Update 04</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>d657910c-5c98-4cdc-806c-ec46a92cb33c</id>
      <name>Schema Validation</name>
      <type>AUTO_DETECT</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Include/schema/Update a Card Schema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>'64226b8f75c9c6e2de87c33a'</defaultValue>
      <description></description>
      <id>994678b0-20da-4802-9371-53790452925f</id>
      <masked>false</masked>
      <name>cardID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
