<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Cards on a Board</name>
   <tag></tag>
   <elementGuidId>8b60da32-67a1-4c79-ab58-353c0b5c6563</elementGuidId>
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
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.trello.com/1/boards/${boardID}/cards?key=6d6aecb7550d295e5b135eae3efaf11e&amp;token=9b8f6a0f594437e6acb100e7e51678a66bae408b8fc6d44c28d4f576a355db4e</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>8f85c2bf-7ee6-4c9f-a36e-e2a38f6123cf</id>
      <name>Schema Validation</name>
      <type>AUTO_DETECT</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Include/schema/Get Cards on a Board Schema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>'64226450bcebc44b74ed3e2a'</defaultValue>
      <description></description>
      <id>60d66636-1b68-4126-8876-7e33d3bab15a</id>
      <masked>false</masked>
      <name>boardID</name>
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
