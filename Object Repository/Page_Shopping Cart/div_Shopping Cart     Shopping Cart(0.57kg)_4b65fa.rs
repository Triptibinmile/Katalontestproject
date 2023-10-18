<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Shopping Cart     Shopping Cart(0.57kg)_4b65fa</name>
   <tag></tag>
   <elementGuidId>d55be257-ecb3-4e18-9861-0d64288f9332</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='checkout-cart']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#checkout-cart</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>9937d311-4915-43a2-a6aa-af89dfff276c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>checkout-cart</value>
      <webElementGuid>2b9b2ff0-34b0-4e78-9d30-ec638a3491e3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>container content pb-5</value>
      <webElementGuid>004af52a-d5fc-4661-8c3e-08a387ad101f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>    Shopping Cart     Shopping Cart
 (0.57kg)
      Image Product Name Model Quantity Unit Price Total        HTC Touch HD  Reward Points: 1200  Product 1          $146.00 $438.00      Palm Treo Pro  Product 2          $337.99 $337.99      Nikon D300  Product 4          $98.00 $294.00         What would you like to do next? Choose if you have a discount code or reward points you want to use or would like to estimate your delivery cost.   Use Coupon Code    Apply Coupon   &lt;!--
$('#button-coupon').on('click', function() {
$.ajax({
url: 'index.php?route=extension/total/coupon/coupon',
type: 'post',
data: 'coupon=' + encodeURIComponent($('input[name=\'coupon\']').val()),
dataType: 'json',
beforeSend: function() {
$('#button-coupon').text('Loading...');
},
complete: function() {
$('#button-coupon').text('Apply Coupon');
},
success: function(json) {
$('.alert-dismissible').remove();
if (json['error']) {
$('#collapse-coupon .card-body').prepend('&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> ' + json['error'] + ' &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>');
}
if (json['redirect'] &amp;&amp; !$('#maza-checkout').length) {
location = json['redirect'];
}
if($('#maza-checkout').length){
$.getJSON(&quot;index.php?route=extension/maza/checkout/total/update&quot;, function(json) {
if (json['redirect']) location = json['redirect'];
$(&quot;#checkout-total&quot;).replaceWith(json.total);
$(&quot;#payment-method&quot;).replaceWith(json.payment_method);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//-->   Estimate Shipping &amp; Taxes     Country    --- Please Select ---  Aaland Islands Afghanistan Albania Algeria American Samoa Andorra Angola Anguilla Antarctica Antigua and Barbuda Argentina Armenia Aruba Ascension Island (British) Australia Austria Azerbaijan Bahamas Bahrain Bangladesh Barbados Belarus Belgium Belize Benin Bermuda Bhutan Bolivia Bonaire, Sint Eustatius and Saba Bosnia and Herzegovina Botswana Bouvet Island Brazil British Indian Ocean Territory Brunei Darussalam Bulgaria Burkina Faso Burundi Cambodia Cameroon Canada Canary Islands Cape Verde Cayman Islands Central African Republic Chad Chile China Christmas Island Cocos (Keeling) Islands Colombia Comoros Congo Cook Islands Costa Rica Cote D'Ivoire Croatia Cuba Curacao Cyprus Czech Republic Democratic Republic of Congo Denmark Djibouti Dominica Dominican Republic East Timor Ecuador Egypt El Salvador Equatorial Guinea Eritrea Estonia Ethiopia Falkland Islands (Malvinas) Faroe Islands Fiji Finland France, Metropolitan French Guiana French Polynesia French Southern Territories FYROM Gabon Gambia Georgia Germany Ghana Gibraltar Greece Greenland Grenada Guadeloupe Guam Guatemala Guernsey Guinea Guinea-Bissau Guyana Haiti Heard and Mc Donald Islands Honduras Hong Kong Hungary Iceland India Indonesia Iran (Islamic Republic of) Iraq Ireland Isle of Man Israel Italy Jamaica Japan Jersey Jordan Kazakhstan Kenya Kiribati Kosovo, Republic of Kuwait Kyrgyzstan Lao People's Democratic Republic Latvia Lebanon Lesotho Liberia Libyan Arab Jamahiriya Liechtenstein Lithuania Luxembourg Macau Madagascar Malawi Malaysia Maldives Mali Malta Marshall Islands Martinique Mauritania Mauritius Mayotte Mexico Micronesia, Federated States of Moldova, Republic of Monaco Mongolia Montenegro Montserrat Morocco Mozambique Myanmar Namibia Nauru Nepal Netherlands Netherlands Antilles New Caledonia New Zealand Nicaragua Niger Nigeria Niue Norfolk Island North Korea Northern Mariana Islands Norway Oman Pakistan Palau Palestinian Territory, Occupied Panama Papua New Guinea Paraguay Peru Philippines Pitcairn Poland Portugal Puerto Rico Qatar Reunion Romania Russian Federation Rwanda Saint Kitts and Nevis Saint Lucia Saint Vincent and the Grenadines Samoa San Marino Sao Tome and Principe Saudi Arabia Senegal Serbia Seychelles Sierra Leone Singapore Slovak Republic Slovenia Solomon Islands Somalia South Africa South Georgia &amp; South Sandwich Islands South Korea South Sudan Spain Sri Lanka St. Barthelemy St. Helena St. Martin (French part) St. Pierre and Miquelon Sudan Suriname Svalbard and Jan Mayen Islands Swaziland Sweden Switzerland Syrian Arab Republic Taiwan Tajikistan Tanzania, United Republic of Thailand Togo Tokelau Tonga Trinidad and Tobago Tristan da Cunha Tunisia Turkey Turkmenistan Turks and Caicos Islands Tuvalu Uganda Ukraine United Arab Emirates United Kingdom United States United States Minor Outlying Islands Uruguay Uzbekistan Vanuatu Vatican City State (Holy See) Venezuela Viet Nam Virgin Islands (British) Virgin Islands (U.S.) Wallis and Futuna Islands Western Sahara Yemen Zambia Zimbabwe  Please select a country!    Region / State   --- Please Select --- AberdeenAberdeenshireAngleseyAngusArgyll and ButeBedfordshireBerkshireBlaenau GwentBridgendBristolBuckinghamshireCaerphillyCambridgeshireCardiffCarmarthenshireCeredigionCheshireClackmannanshireConwyCornwallCounty AntrimCounty ArmaghCounty DownCounty FermanaghCounty LondonderryCounty TyroneCumbriaDenbighshireDerbyshireDevonDorsetDumfries and GallowayDundeeDurhamEast AyrshireEast DunbartonshireEast LothianEast RenfrewshireEast Riding of YorkshireEast SussexEdinburghEssexFalkirkFifeFlintshireGlasgowGloucestershireGreater LondonGreater ManchesterGwyneddHampshireHerefordshireHertfordshireHighlandsInverclydeIsle of WightKentLancashireLeicestershireLincolnshireMerseysideMerthyr TydfilMidlothianMonmouthshireMorayNeath Port TalbotNewportNorfolkNorth AyrshireNorth LanarkshireNorth YorkshireNorthamptonshireNorthumberlandNottinghamshireOrkney IslandsOxfordshirePembrokeshirePerth and KinrossPowysRenfrewshireRhondda Cynon TaffRutlandScottish BordersShetland IslandsShropshireSomersetSouth AyrshireSouth LanarkshireSouth YorkshireStaffordshireStirlingSuffolkSurreySwanseaTorfaenTyne and WearVale of GlamorganWarwickshireWest DunbartonshireWest LothianWest MidlandsWest SussexWest YorkshireWestern IslesWiltshireWorcestershireWrexham Please select a region / state!    Post Code   Postcode must be between 2 and 10 characters!     Get Quotes     &lt;!--
$('#button-quote').on('click', function() {
$.ajax({
url: 'index.php?route=extension/total/shipping/quote',
type: 'post',
data: 'country_id=' + $('select[name=\'country_id\']').val() + '&amp;zone_id=' + $('select[name=\'zone_id\']').val() + '&amp;postcode=' + encodeURIComponent($('input[name=\'postcode\']').val()),
dataType: 'json',
beforeSend: function() {
$('#button-quote').text('Loading...');
},
complete: function() {
$('#button-quote').text('Get Quotes');
},
success: function(json) {
$('.alert-dismissible, .text-danger').remove();
if (json['error']) {
if (json['error']['warning']) {
$('#collapse-shipping .card-body').prepend('&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> ' + json['error']['warning'] + ' &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>');
}
if (json['error']['country']) {
$('select[name=\'country_id\']').addClass('is-invalid');
} else {
$('select[name=\'country_id\']').removeClass('is-invalid');
}
if (json['error']['zone']) {
$('select[name=\'zone_id\']').addClass('is-invalid');
} else {
$('select[name=\'zone_id\']').removeClass('is-invalid');
}
if (json['error']['postcode']) {
$('input[name=\'postcode\']').addClass('is-invalid');
} else {
$('input[name=\'postcode\']').removeClass('is-invalid');
}
}
if (json['shipping_method']) {
$('#modal-shipping').remove();
html = '&lt;div id=&quot;modal-shipping&quot; class=&quot;modal&quot;>';
html += ' &lt;div class=&quot;modal-dialog&quot;>';
html += ' &lt;div class=&quot;modal-content&quot;>';
html += ' &lt;div class=&quot;modal-header&quot;>';
html += ' &lt;h5 class=&quot;modal-title&quot;>Please select the preferred shipping method to use on this order.&lt;/h5>';
html += ' &lt;/div>';
html += ' &lt;div class=&quot;modal-body&quot;>';
for (i in json['shipping_method']) {
html += '&lt;p>&lt;strong>' + json['shipping_method'][i]['title'] + '&lt;/strong>&lt;/p>';
if (!json['shipping_method'][i]['error']) {
for (j in json['shipping_method'][i]['quote']) {
html += '&lt;div class=&quot;form-check&quot;>';
html += ' &lt;label>';
if (json['shipping_method'][i]['quote'][j]['code'] == '') {
html += '&lt;input type=&quot;radio&quot; name=&quot;shipping_method&quot; value=&quot;' + json['shipping_method'][i]['quote'][j]['code'] + '&quot; checked=&quot;checked&quot; /> ';
} else {
html += '&lt;input type=&quot;radio&quot; name=&quot;shipping_method&quot; value=&quot;' + json['shipping_method'][i]['quote'][j]['code'] + '&quot; /> ';
}
html += json['shipping_method'][i]['quote'][j]['title'] + ' - ' + json['shipping_method'][i]['quote'][j]['text'] + '&lt;/label>&lt;/div>';
}
} else {
html += '&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>' + json['shipping_method'][i]['error'] + '&lt;/div>';
}
}
html += ' &lt;/div>';
html += ' &lt;div class=&quot;modal-footer&quot;>';
html += ' &lt;button type=&quot;button&quot; class=&quot;btn btn-light&quot; data-dismiss=&quot;modal&quot;>Cancel&lt;/button>';
html += ' &lt;button type=&quot;button&quot; id=&quot;button-shipping&quot; data-loading-text=&quot;Loading...&quot; disabled=&quot;disabled&quot; class=&quot;btn btn-primary&quot;>Apply Shipping&lt;/button>';
html += ' &lt;/div>';
html += ' &lt;/div>';
html += ' &lt;/div>';
html += '&lt;/div> ';
$('body').append(html);
$('#modal-shipping').modal('show');
$('input[name=\'shipping_method\']').on('change', function() {
$('#button-shipping').prop('disabled', false);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
$(document).delegate('#button-shipping', 'click', function() {
$.ajax({
url: 'index.php?route=extension/total/shipping/shipping',
type: 'post',
data: 'shipping_method=' + encodeURIComponent($('input[name=\'shipping_method\']:checked').val()),
dataType: 'json',
beforeSend: function() {
$('#button-shipping').text('Loading...');
},
complete: function() {
$('#button-shipping').text('Apply Shipping');
},
success: function(json) {
$('.alert-dismissible').remove();
if (json['error']) {
$('.breadcrumb').after('&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> ' + json['error'] + '&lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>');
$('html, body').animate({scrollTop: 0}, 'slow');
}
if (json['redirect']) {
location = json['redirect'];
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//--> &lt;!--
$('select[name=\'country_id\']').on('change', function() {
$.ajax({
url: 'index.php?route=extension/total/shipping/country&amp;country_id=' + this.value,
dataType: 'json',
beforeSend: function() {
$('select[name=\'country_id\']').prop('disabled', true);
},
complete: function() {
$('select[name=\'country_id\']').prop('disabled', false);
},
success: function(json) {
if (json['postcode_required'] == '1') {
$('input[name=\'postcode\']').parent().parent().addClass('required');
} else {
$('input[name=\'postcode\']').parent().parent().removeClass('required');
}
html = '&lt;option value=&quot;&quot;> --- Please Select --- &lt;/option>';
if (json['zone'] &amp;&amp; json['zone'] != '') {
for (i = 0; i &lt; json['zone'].length; i++) {
html += '&lt;option value=&quot;' + json['zone'][i]['zone_id'] + '&quot;';
if (json['zone'][i]['zone_id'] == '') {
html += ' selected=&quot;selected&quot;';
}
html += '>' + json['zone'][i]['name'] + '&lt;/option>';
}
} else {
html += '&lt;option value=&quot;0&quot; selected=&quot;selected&quot;> --- None --- &lt;/option>';
}
$('select[name=\'zone_id\']').html(html);
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
$('select[name=\'country_id\']').trigger('change');
//-->  Use Gift Certificate       &lt;!--
$('#button-voucher').on('click', function() {
$.ajax({
url: 'index.php?route=extension/total/voucher/voucher',
type: 'post',
data: 'voucher=' + encodeURIComponent($('input[name=\'voucher\']').val()),
dataType: 'json',
beforeSend: function() {
$('#button-voucher').val('Loading...');
},
complete: function() {
$('#button-voucher').val('Apply Gift Certificate');
},
success: function(json) {
$('.alert-dismissible').remove();
if (json['error']) {
$('#collapse-voucher .card-body').prepend('&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> ' + json['error'] + ' &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>');
}
if (json['redirect'] &amp;&amp; !$('#maza-checkout').length) {
location = json['redirect'];
}
if($('#maza-checkout').length){
$.getJSON(&quot;index.php?route=extension/maza/checkout/total/update&quot;, function(json) {
if (json['redirect']) location = json['redirect'];
$(&quot;#checkout-total&quot;).replaceWith(json.total);
$(&quot;#payment-method&quot;).replaceWith(json.payment_method);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//-->      Sub-Total: $879.99   Eco Tax (-2.00): $14.00   VAT (20%): $176.00   Total: $1,069.99      Continue Shopping Checkout   </value>
      <webElementGuid>872b0358-4dba-43e3-9ab3-9297269312f4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;checkout-cart&quot;)</value>
      <webElementGuid>768ee1e9-af83-4306-a4aa-d31a2f751214</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='checkout-cart']</value>
      <webElementGuid>813b0210-06c6-4bfc-b8d8-87ac2a6ac879</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='container']/div</value>
      <webElementGuid>27ac2616-e343-48c1-a431-399997071e73</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='SAVE60'])[1]/following::div[1]</value>
      <webElementGuid>89ca6f86-c5c4-41fb-a0d1-edfa3e624c2f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Upto 60% Off'])[1]/following::div[1]</value>
      <webElementGuid>22f6e72f-19d6-4deb-8b88-30d65a5a2f36</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[5]/div</value>
      <webElementGuid>65bd804e-0302-4d6a-850a-1e14438fcddc</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'checkout-cart' and (text() = concat(&quot;    Shopping Cart     Shopping Cart
 (0.57kg)
      Image Product Name Model Quantity Unit Price Total        HTC Touch HD  Reward Points: 1200  Product 1          $146.00 $438.00      Palm Treo Pro  Product 2          $337.99 $337.99      Nikon D300  Product 4          $98.00 $294.00         What would you like to do next? Choose if you have a discount code or reward points you want to use or would like to estimate your delivery cost.   Use Coupon Code    Apply Coupon   &lt;!--
$(&quot; , &quot;'&quot; , &quot;#button-coupon&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/coupon/coupon&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;coupon=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;coupon\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-coupon&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-coupon&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Apply Coupon&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#collapse-coupon .card-body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;] &amp;&amp; !$(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length) {
location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
}
if($(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length){
$.getJSON(&quot;index.php?route=extension/maza/checkout/total/update&quot;, function(json) {
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;]) location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
$(&quot;#checkout-total&quot;).replaceWith(json.total);
$(&quot;#payment-method&quot;).replaceWith(json.payment_method);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//-->   Estimate Shipping &amp; Taxes     Country    --- Please Select ---  Aaland Islands Afghanistan Albania Algeria American Samoa Andorra Angola Anguilla Antarctica Antigua and Barbuda Argentina Armenia Aruba Ascension Island (British) Australia Austria Azerbaijan Bahamas Bahrain Bangladesh Barbados Belarus Belgium Belize Benin Bermuda Bhutan Bolivia Bonaire, Sint Eustatius and Saba Bosnia and Herzegovina Botswana Bouvet Island Brazil British Indian Ocean Territory Brunei Darussalam Bulgaria Burkina Faso Burundi Cambodia Cameroon Canada Canary Islands Cape Verde Cayman Islands Central African Republic Chad Chile China Christmas Island Cocos (Keeling) Islands Colombia Comoros Congo Cook Islands Costa Rica Cote D&quot; , &quot;'&quot; , &quot;Ivoire Croatia Cuba Curacao Cyprus Czech Republic Democratic Republic of Congo Denmark Djibouti Dominica Dominican Republic East Timor Ecuador Egypt El Salvador Equatorial Guinea Eritrea Estonia Ethiopia Falkland Islands (Malvinas) Faroe Islands Fiji Finland France, Metropolitan French Guiana French Polynesia French Southern Territories FYROM Gabon Gambia Georgia Germany Ghana Gibraltar Greece Greenland Grenada Guadeloupe Guam Guatemala Guernsey Guinea Guinea-Bissau Guyana Haiti Heard and Mc Donald Islands Honduras Hong Kong Hungary Iceland India Indonesia Iran (Islamic Republic of) Iraq Ireland Isle of Man Israel Italy Jamaica Japan Jersey Jordan Kazakhstan Kenya Kiribati Kosovo, Republic of Kuwait Kyrgyzstan Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic Latvia Lebanon Lesotho Liberia Libyan Arab Jamahiriya Liechtenstein Lithuania Luxembourg Macau Madagascar Malawi Malaysia Maldives Mali Malta Marshall Islands Martinique Mauritania Mauritius Mayotte Mexico Micronesia, Federated States of Moldova, Republic of Monaco Mongolia Montenegro Montserrat Morocco Mozambique Myanmar Namibia Nauru Nepal Netherlands Netherlands Antilles New Caledonia New Zealand Nicaragua Niger Nigeria Niue Norfolk Island North Korea Northern Mariana Islands Norway Oman Pakistan Palau Palestinian Territory, Occupied Panama Papua New Guinea Paraguay Peru Philippines Pitcairn Poland Portugal Puerto Rico Qatar Reunion Romania Russian Federation Rwanda Saint Kitts and Nevis Saint Lucia Saint Vincent and the Grenadines Samoa San Marino Sao Tome and Principe Saudi Arabia Senegal Serbia Seychelles Sierra Leone Singapore Slovak Republic Slovenia Solomon Islands Somalia South Africa South Georgia &amp; South Sandwich Islands South Korea South Sudan Spain Sri Lanka St. Barthelemy St. Helena St. Martin (French part) St. Pierre and Miquelon Sudan Suriname Svalbard and Jan Mayen Islands Swaziland Sweden Switzerland Syrian Arab Republic Taiwan Tajikistan Tanzania, United Republic of Thailand Togo Tokelau Tonga Trinidad and Tobago Tristan da Cunha Tunisia Turkey Turkmenistan Turks and Caicos Islands Tuvalu Uganda Ukraine United Arab Emirates United Kingdom United States United States Minor Outlying Islands Uruguay Uzbekistan Vanuatu Vatican City State (Holy See) Venezuela Viet Nam Virgin Islands (British) Virgin Islands (U.S.) Wallis and Futuna Islands Western Sahara Yemen Zambia Zimbabwe  Please select a country!    Region / State   --- Please Select --- AberdeenAberdeenshireAngleseyAngusArgyll and ButeBedfordshireBerkshireBlaenau GwentBridgendBristolBuckinghamshireCaerphillyCambridgeshireCardiffCarmarthenshireCeredigionCheshireClackmannanshireConwyCornwallCounty AntrimCounty ArmaghCounty DownCounty FermanaghCounty LondonderryCounty TyroneCumbriaDenbighshireDerbyshireDevonDorsetDumfries and GallowayDundeeDurhamEast AyrshireEast DunbartonshireEast LothianEast RenfrewshireEast Riding of YorkshireEast SussexEdinburghEssexFalkirkFifeFlintshireGlasgowGloucestershireGreater LondonGreater ManchesterGwyneddHampshireHerefordshireHertfordshireHighlandsInverclydeIsle of WightKentLancashireLeicestershireLincolnshireMerseysideMerthyr TydfilMidlothianMonmouthshireMorayNeath Port TalbotNewportNorfolkNorth AyrshireNorth LanarkshireNorth YorkshireNorthamptonshireNorthumberlandNottinghamshireOrkney IslandsOxfordshirePembrokeshirePerth and KinrossPowysRenfrewshireRhondda Cynon TaffRutlandScottish BordersShetland IslandsShropshireSomersetSouth AyrshireSouth LanarkshireSouth YorkshireStaffordshireStirlingSuffolkSurreySwanseaTorfaenTyne and WearVale of GlamorganWarwickshireWest DunbartonshireWest LothianWest MidlandsWest SussexWest YorkshireWestern IslesWiltshireWorcestershireWrexham Please select a region / state!    Post Code   Postcode must be between 2 and 10 characters!     Get Quotes     &lt;!--
$(&quot; , &quot;'&quot; , &quot;#button-quote&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/shipping/quote&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;country_id=&quot; , &quot;'&quot; , &quot; + $(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val() + &quot; , &quot;'&quot; , &quot;&amp;zone_id=&quot; , &quot;'&quot; , &quot; + $(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val() + &quot; , &quot;'&quot; , &quot;&amp;postcode=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-quote&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-quote&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Get Quotes&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible, .text-danger&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#collapse-shipping .card-body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;country&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;postcode&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
}
}
if (json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#modal-shipping&quot; , &quot;'&quot; , &quot;).remove();
html = &quot; , &quot;'&quot; , &quot;&lt;div id=&quot;modal-shipping&quot; class=&quot;modal&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-dialog&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-content&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-header&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;h5 class=&quot;modal-title&quot;>Please select the preferred shipping method to use on this order.&lt;/h5>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-body&quot;>&quot; , &quot;'&quot; , &quot;;
for (i in json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;]) {
html += &quot; , &quot;'&quot; , &quot;&lt;p>&lt;strong>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/strong>&lt;/p>&quot; , &quot;'&quot; , &quot;;
if (!json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
for (j in json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;]) {
html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-check&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;label>&quot; , &quot;'&quot; , &quot;;
if (json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;] == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
html += &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;radio&quot; name=&quot;shipping_method&quot; value=&quot;&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&quot; checked=&quot;checked&quot; /> &quot; , &quot;'&quot; , &quot;;
} else {
html += &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;radio&quot; name=&quot;shipping_method&quot; value=&quot;&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&quot; /> &quot; , &quot;'&quot; , &quot;;
}
html += json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/label>&lt;/div>&quot; , &quot;'&quot; , &quot;;
}
} else {
html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
}
}
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-footer&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;btn btn-light&quot; data-dismiss=&quot;modal&quot;>Cancel&lt;/button>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; id=&quot;button-shipping&quot; data-loading-text=&quot;Loading...&quot; disabled=&quot;disabled&quot; class=&quot;btn btn-primary&quot;>Apply Shipping&lt;/button>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot;&lt;/div> &quot; , &quot;'&quot; , &quot;;
$(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).append(html);
$(&quot; , &quot;'&quot; , &quot;#modal-shipping&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;shipping_method\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
$(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
$(document).delegate(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/shipping/shipping&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;shipping_method=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;shipping_method\&quot; , &quot;'&quot; , &quot;]:checked&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Apply Shipping&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;.breadcrumb&quot; , &quot;'&quot; , &quot;).after(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop: 0}, &quot; , &quot;'&quot; , &quot;slow&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;]) {
location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//--> &lt;!--
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/shipping/country&amp;country_id=&quot; , &quot;'&quot; , &quot; + this.value,
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
},
success: function(json) {
if (json[&quot; , &quot;'&quot; , &quot;postcode_required&quot; , &quot;'&quot; , &quot;] == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).parent().parent().addClass(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).parent().parent().removeClass(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
}
html = &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot;> --- Please Select --- &lt;/option>&quot; , &quot;'&quot; , &quot;;
if (json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;] &amp;&amp; json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;] != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
for (i = 0; i &lt; json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;].length; i++) {
html += &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;zone_id&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot;;
if (json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;zone_id&quot; , &quot;'&quot; , &quot;] == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
html += &quot; , &quot;'&quot; , &quot; selected=&quot;selected&quot;&quot; , &quot;'&quot; , &quot;;
}
html += &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;;
}
} else {
html += &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;0&quot; selected=&quot;selected&quot;> --- None --- &lt;/option>&quot; , &quot;'&quot; , &quot;;
}
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).html(html);
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
//-->  Use Gift Certificate       &lt;!--
$(&quot; , &quot;'&quot; , &quot;#button-voucher&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/voucher/voucher&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;voucher=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;voucher\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-voucher&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-voucher&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;Apply Gift Certificate&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#collapse-voucher .card-body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;] &amp;&amp; !$(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length) {
location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
}
if($(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length){
$.getJSON(&quot;index.php?route=extension/maza/checkout/total/update&quot;, function(json) {
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;]) location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
$(&quot;#checkout-total&quot;).replaceWith(json.total);
$(&quot;#payment-method&quot;).replaceWith(json.payment_method);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//-->      Sub-Total: $879.99   Eco Tax (-2.00): $14.00   VAT (20%): $176.00   Total: $1,069.99      Continue Shopping Checkout   &quot;) or . = concat(&quot;    Shopping Cart     Shopping Cart
 (0.57kg)
      Image Product Name Model Quantity Unit Price Total        HTC Touch HD  Reward Points: 1200  Product 1          $146.00 $438.00      Palm Treo Pro  Product 2          $337.99 $337.99      Nikon D300  Product 4          $98.00 $294.00         What would you like to do next? Choose if you have a discount code or reward points you want to use or would like to estimate your delivery cost.   Use Coupon Code    Apply Coupon   &lt;!--
$(&quot; , &quot;'&quot; , &quot;#button-coupon&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/coupon/coupon&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;coupon=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;coupon\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-coupon&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-coupon&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Apply Coupon&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#collapse-coupon .card-body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;] &amp;&amp; !$(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length) {
location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
}
if($(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length){
$.getJSON(&quot;index.php?route=extension/maza/checkout/total/update&quot;, function(json) {
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;]) location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
$(&quot;#checkout-total&quot;).replaceWith(json.total);
$(&quot;#payment-method&quot;).replaceWith(json.payment_method);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//-->   Estimate Shipping &amp; Taxes     Country    --- Please Select ---  Aaland Islands Afghanistan Albania Algeria American Samoa Andorra Angola Anguilla Antarctica Antigua and Barbuda Argentina Armenia Aruba Ascension Island (British) Australia Austria Azerbaijan Bahamas Bahrain Bangladesh Barbados Belarus Belgium Belize Benin Bermuda Bhutan Bolivia Bonaire, Sint Eustatius and Saba Bosnia and Herzegovina Botswana Bouvet Island Brazil British Indian Ocean Territory Brunei Darussalam Bulgaria Burkina Faso Burundi Cambodia Cameroon Canada Canary Islands Cape Verde Cayman Islands Central African Republic Chad Chile China Christmas Island Cocos (Keeling) Islands Colombia Comoros Congo Cook Islands Costa Rica Cote D&quot; , &quot;'&quot; , &quot;Ivoire Croatia Cuba Curacao Cyprus Czech Republic Democratic Republic of Congo Denmark Djibouti Dominica Dominican Republic East Timor Ecuador Egypt El Salvador Equatorial Guinea Eritrea Estonia Ethiopia Falkland Islands (Malvinas) Faroe Islands Fiji Finland France, Metropolitan French Guiana French Polynesia French Southern Territories FYROM Gabon Gambia Georgia Germany Ghana Gibraltar Greece Greenland Grenada Guadeloupe Guam Guatemala Guernsey Guinea Guinea-Bissau Guyana Haiti Heard and Mc Donald Islands Honduras Hong Kong Hungary Iceland India Indonesia Iran (Islamic Republic of) Iraq Ireland Isle of Man Israel Italy Jamaica Japan Jersey Jordan Kazakhstan Kenya Kiribati Kosovo, Republic of Kuwait Kyrgyzstan Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic Latvia Lebanon Lesotho Liberia Libyan Arab Jamahiriya Liechtenstein Lithuania Luxembourg Macau Madagascar Malawi Malaysia Maldives Mali Malta Marshall Islands Martinique Mauritania Mauritius Mayotte Mexico Micronesia, Federated States of Moldova, Republic of Monaco Mongolia Montenegro Montserrat Morocco Mozambique Myanmar Namibia Nauru Nepal Netherlands Netherlands Antilles New Caledonia New Zealand Nicaragua Niger Nigeria Niue Norfolk Island North Korea Northern Mariana Islands Norway Oman Pakistan Palau Palestinian Territory, Occupied Panama Papua New Guinea Paraguay Peru Philippines Pitcairn Poland Portugal Puerto Rico Qatar Reunion Romania Russian Federation Rwanda Saint Kitts and Nevis Saint Lucia Saint Vincent and the Grenadines Samoa San Marino Sao Tome and Principe Saudi Arabia Senegal Serbia Seychelles Sierra Leone Singapore Slovak Republic Slovenia Solomon Islands Somalia South Africa South Georgia &amp; South Sandwich Islands South Korea South Sudan Spain Sri Lanka St. Barthelemy St. Helena St. Martin (French part) St. Pierre and Miquelon Sudan Suriname Svalbard and Jan Mayen Islands Swaziland Sweden Switzerland Syrian Arab Republic Taiwan Tajikistan Tanzania, United Republic of Thailand Togo Tokelau Tonga Trinidad and Tobago Tristan da Cunha Tunisia Turkey Turkmenistan Turks and Caicos Islands Tuvalu Uganda Ukraine United Arab Emirates United Kingdom United States United States Minor Outlying Islands Uruguay Uzbekistan Vanuatu Vatican City State (Holy See) Venezuela Viet Nam Virgin Islands (British) Virgin Islands (U.S.) Wallis and Futuna Islands Western Sahara Yemen Zambia Zimbabwe  Please select a country!    Region / State   --- Please Select --- AberdeenAberdeenshireAngleseyAngusArgyll and ButeBedfordshireBerkshireBlaenau GwentBridgendBristolBuckinghamshireCaerphillyCambridgeshireCardiffCarmarthenshireCeredigionCheshireClackmannanshireConwyCornwallCounty AntrimCounty ArmaghCounty DownCounty FermanaghCounty LondonderryCounty TyroneCumbriaDenbighshireDerbyshireDevonDorsetDumfries and GallowayDundeeDurhamEast AyrshireEast DunbartonshireEast LothianEast RenfrewshireEast Riding of YorkshireEast SussexEdinburghEssexFalkirkFifeFlintshireGlasgowGloucestershireGreater LondonGreater ManchesterGwyneddHampshireHerefordshireHertfordshireHighlandsInverclydeIsle of WightKentLancashireLeicestershireLincolnshireMerseysideMerthyr TydfilMidlothianMonmouthshireMorayNeath Port TalbotNewportNorfolkNorth AyrshireNorth LanarkshireNorth YorkshireNorthamptonshireNorthumberlandNottinghamshireOrkney IslandsOxfordshirePembrokeshirePerth and KinrossPowysRenfrewshireRhondda Cynon TaffRutlandScottish BordersShetland IslandsShropshireSomersetSouth AyrshireSouth LanarkshireSouth YorkshireStaffordshireStirlingSuffolkSurreySwanseaTorfaenTyne and WearVale of GlamorganWarwickshireWest DunbartonshireWest LothianWest MidlandsWest SussexWest YorkshireWestern IslesWiltshireWorcestershireWrexham Please select a region / state!    Post Code   Postcode must be between 2 and 10 characters!     Get Quotes     &lt;!--
$(&quot; , &quot;'&quot; , &quot;#button-quote&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/shipping/quote&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;country_id=&quot; , &quot;'&quot; , &quot; + $(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val() + &quot; , &quot;'&quot; , &quot;&amp;zone_id=&quot; , &quot;'&quot; , &quot; + $(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val() + &quot; , &quot;'&quot; , &quot;&amp;postcode=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-quote&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-quote&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Get Quotes&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible, .text-danger&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#collapse-shipping .card-body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;country&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;][&quot; , &quot;'&quot; , &quot;postcode&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
}
}
if (json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#modal-shipping&quot; , &quot;'&quot; , &quot;).remove();
html = &quot; , &quot;'&quot; , &quot;&lt;div id=&quot;modal-shipping&quot; class=&quot;modal&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-dialog&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-content&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-header&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;h5 class=&quot;modal-title&quot;>Please select the preferred shipping method to use on this order.&lt;/h5>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-body&quot;>&quot; , &quot;'&quot; , &quot;;
for (i in json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;]) {
html += &quot; , &quot;'&quot; , &quot;&lt;p>&lt;strong>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/strong>&lt;/p>&quot; , &quot;'&quot; , &quot;;
if (!json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
for (j in json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;]) {
html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;form-check&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;label>&quot; , &quot;'&quot; , &quot;;
if (json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;] == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
html += &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;radio&quot; name=&quot;shipping_method&quot; value=&quot;&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&quot; checked=&quot;checked&quot; /> &quot; , &quot;'&quot; , &quot;;
} else {
html += &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;radio&quot; name=&quot;shipping_method&quot; value=&quot;&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;code&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&quot; /> &quot; , &quot;'&quot; , &quot;;
}
html += json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot;][j][&quot; , &quot;'&quot; , &quot;text&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/label>&lt;/div>&quot; , &quot;'&quot; , &quot;;
}
} else {
html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;shipping_method&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;;
}
}
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;div class=&quot;modal-footer&quot;>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;btn btn-light&quot; data-dismiss=&quot;modal&quot;>Cancel&lt;/button>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; id=&quot;button-shipping&quot; data-loading-text=&quot;Loading...&quot; disabled=&quot;disabled&quot; class=&quot;btn btn-primary&quot;>Apply Shipping&lt;/button>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot; &lt;/div>&quot; , &quot;'&quot; , &quot;;
html += &quot; , &quot;'&quot; , &quot;&lt;/div> &quot; , &quot;'&quot; , &quot;;
$(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).append(html);
$(&quot; , &quot;'&quot; , &quot;#modal-shipping&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;shipping_method\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
$(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
$(document).delegate(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/shipping/shipping&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;shipping_method=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;shipping_method\&quot; , &quot;'&quot; , &quot;]:checked&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-shipping&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Apply Shipping&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;.breadcrumb&quot; , &quot;'&quot; , &quot;).after(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop: 0}, &quot; , &quot;'&quot; , &quot;slow&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;]) {
location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//--> &lt;!--
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/shipping/country&amp;country_id=&quot; , &quot;'&quot; , &quot; + this.value,
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
},
success: function(json) {
if (json[&quot; , &quot;'&quot; , &quot;postcode_required&quot; , &quot;'&quot; , &quot;] == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).parent().parent().addClass(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
} else {
$(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;postcode\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).parent().parent().removeClass(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
}
html = &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot;> --- Please Select --- &lt;/option>&quot; , &quot;'&quot; , &quot;;
if (json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;] &amp;&amp; json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;] != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
for (i = 0; i &lt; json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;].length; i++) {
html += &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;zone_id&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&quot;&quot; , &quot;'&quot; , &quot;;
if (json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;zone_id&quot; , &quot;'&quot; , &quot;] == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
html += &quot; , &quot;'&quot; , &quot; selected=&quot;selected&quot;&quot; , &quot;'&quot; , &quot;;
}
html += &quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;zone&quot; , &quot;'&quot; , &quot;][i][&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;;
}
} else {
html += &quot; , &quot;'&quot; , &quot;&lt;option value=&quot;0&quot; selected=&quot;selected&quot;> --- None --- &lt;/option>&quot; , &quot;'&quot; , &quot;;
}
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;zone_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).html(html);
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
$(&quot; , &quot;'&quot; , &quot;select[name=\&quot; , &quot;'&quot; , &quot;country_id\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
//-->  Use Gift Certificate       &lt;!--
$(&quot; , &quot;'&quot; , &quot;#button-voucher&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
$.ajax({
url: &quot; , &quot;'&quot; , &quot;index.php?route=extension/total/voucher/voucher&quot; , &quot;'&quot; , &quot;,
type: &quot; , &quot;'&quot; , &quot;post&quot; , &quot;'&quot; , &quot;,
data: &quot; , &quot;'&quot; , &quot;voucher=&quot; , &quot;'&quot; , &quot; + encodeURIComponent($(&quot; , &quot;'&quot; , &quot;input[name=\&quot; , &quot;'&quot; , &quot;voucher\&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;).val()),
dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
beforeSend: function() {
$(&quot; , &quot;'&quot; , &quot;#button-voucher&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;Loading...&quot; , &quot;'&quot; , &quot;);
},
complete: function() {
$(&quot; , &quot;'&quot; , &quot;#button-voucher&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;Apply Gift Certificate&quot; , &quot;'&quot; , &quot;);
},
success: function(json) {
$(&quot; , &quot;'&quot; , &quot;.alert-dismissible&quot; , &quot;'&quot; , &quot;).remove();
if (json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;]) {
$(&quot; , &quot;'&quot; , &quot;#collapse-voucher .card-body&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;alert alert-danger alert-dismissible&quot;>&lt;i class=&quot;fa fa-exclamation-circle&quot;>&lt;/i> &quot; , &quot;'&quot; , &quot; + json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] + &quot; , &quot;'&quot; , &quot; &lt;button type=&quot;button&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/button>&lt;/div>&quot; , &quot;'&quot; , &quot;);
}
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;] &amp;&amp; !$(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length) {
location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
}
if($(&quot; , &quot;'&quot; , &quot;#maza-checkout&quot; , &quot;'&quot; , &quot;).length){
$.getJSON(&quot;index.php?route=extension/maza/checkout/total/update&quot;, function(json) {
if (json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;]) location = json[&quot; , &quot;'&quot; , &quot;redirect&quot; , &quot;'&quot; , &quot;];
$(&quot;#checkout-total&quot;).replaceWith(json.total);
$(&quot;#payment-method&quot;).replaceWith(json.payment_method);
});
}
},
error: function(xhr, ajaxOptions, thrownError) {
alert(thrownError + &quot;\r\n&quot; + xhr.statusText + &quot;\r\n&quot; + xhr.responseText);
}
});
});
//-->      Sub-Total: $879.99   Eco Tax (-2.00): $14.00   VAT (20%): $176.00   Total: $1,069.99      Continue Shopping Checkout   &quot;))]</value>
      <webElementGuid>731e038b-b3dd-4b6e-b27a-d041b370af47</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
