<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Cards in a list</name>
   <tag></tag>
   <elementGuidId>d831b68c-1aa7-4cfb-b897-c16aea0243b8</elementGuidId>
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
   <restUrl>https://api.trello.com/1/lists/${listID}/cards?key=6d6aecb7550d295e5b135eae3efaf11e&amp;token=9b8f6a0f594437e6acb100e7e51678a66bae408b8fc6d44c28d4f576a355db4e</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>b8053ea4-0274-4bad-912a-bf4402f65554</id>
      <name>Schma Validation</name>
      <type>AUTO_DETECT</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Include/schema/Get Cards in a List Schema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>'6422646fcfadbc7ab61d7970'</defaultValue>
      <description></description>
      <id>139b7797-3d79-44b1-9bc7-4adcc17efca4</id>
      <masked>false</masked>
      <name>listID</name>
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

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, '[1].cover.idUploadedBackground', &quot;My First Card&quot;)
WS.verifyElementPropertyValue(response, '[0].subscribed', &quot;Demo Card&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
