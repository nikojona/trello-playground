<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Lists on a Board</name>
   <tag></tag>
   <elementGuidId>503972cb-6418-4d6d-9824-4f4230467d64</elementGuidId>
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
   <restUrl>https://api.trello.com/1/boards/${boardID}/lists?key=6d6aecb7550d295e5b135eae3efaf11e&amp;token=9b8f6a0f594437e6acb100e7e51678a66bae408b8fc6d44c28d4f576a355db4e</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>0f9c5357-8023-4e75-b50d-0bdcd1c42d9f</id>
      <name>Schema Validation</name>
      <type>AUTO_DETECT</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Include/schema/Get Lists on a Board Schema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>'64226450bcebc44b74ed3e2a'</defaultValue>
      <description></description>
      <id>9bcd6cd9-cfd4-4aa6-b19a-fac934a00715</id>
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
