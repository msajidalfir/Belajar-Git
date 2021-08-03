<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>03.Update Employee Detail</name>
   <tag></tag>
   <elementGuidId>78fc095f-aea5-4899-b27c-eb8be4124acd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\t\&quot;id\&quot;: \&quot;4\&quot;,\n        \&quot;firstName\&quot;: \&quot;Nania\&quot;,\n        \&quot;middleName\&quot;: \&quot;Janes\&quot;,\n         \&quot;lastName\&quot;: \&quot;Lewise\&quot;,\n         \&quot;code\&quot;: \&quot;0011\&quot;,\n         \&quot;dob\&quot;: \&quot;2016-05-04\&quot;,\n         \&quot;licenceNumber\&quot; : \&quot;444555124223\&quot;,\n         \&quot;licenseNumberExpDate\&quot; : \&quot;2017-02-09\&quot;,\n         \&quot;maritalStatus\&quot;: \&quot;Married\&quot;,\n         \&quot;gender\&quot;: \&quot;2\&quot;,\n         \&quot;otherId\&quot;: \&quot;4646522\&quot;,\n         \&quot;nationality\&quot;: \&quot;British\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.AksesToken}</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://qa.cilsy.id/symfony/web/index.php/api/v1/employee/9</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
