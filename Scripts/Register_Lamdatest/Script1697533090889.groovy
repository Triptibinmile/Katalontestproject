import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import java.util.Random as Random

String mail = CustomKeywords.'mubasher.help_keyword_RandomEmail.getEmail'(opps('sdsfsd'), mailinator.com('dfdsfadf'))

WebUI.openBrowser('')

WebUI.navigateToUrl('https://ecommerce-playground.lambdatest.io/index.php?route=common/home')

WebUI.click(findTestObject('Object Repository/Page_Your Store/span_My account'))

WebUI.click(findTestObject('Object Repository/Page_Your Store/p_This is a dummy website for Web Automatio_ab7216'))

WebUI.click(findTestObject('Object Repository/Page_Account Login/input_E-Mail Address_email'))

WebUI.click(findTestObject('Object Repository/Page_Account Login/i_Register_fas fa-user-plus fa-fw mr-1'))

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_First Name_firstname'), 'Tripti ')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_Last Name_lastname'), 'Pandey')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_E-Mail_email'), 'tripti234@gmail.com')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_Telephone_telephone'), '9540580302')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Register Account/input_Password_password'), 'RigbBhfdqOBGNlJIWM1ClA==')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Register Account/input_Password Confirm_confirm'), 'RigbBhfdqOBGNlJIWM1ClA==')

WebUI.click(findTestObject('Object Repository/Page_Register Account/input_Privacy Policy_btn btn-primary'))

WebUI.click(findTestObject('Object Repository/Page_Register Account/label_Yes'))

WebUI.click(findTestObject('Object Repository/Page_Register Account/div_I have read and agree to the Privacy Policy'))

WebUI.click(findTestObject('Object Repository/Page_Register Account/input_Privacy Policy_btn btn-primary'))

WebUI.click(findTestObject('Object Repository/Page_Your Account Has Been Created/a_Continue'))

