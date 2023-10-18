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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://ecommerce-playground.lambdatest.io/index.php?route=account/login')

WebUI.setText(findTestObject('Object Repository/Page_Account Login/input_E-Mail Address_email'), 'tripti@binmile.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Account Login/input_Password_password'), 'RigbBhfdqOBGNlJIWM1ClA==')

WebUI.click(findTestObject('Object Repository/Page_Account Login/input_Forgotten Password_btn btn-primary'))

WebUI.click(findTestObject('Object Repository/Page_My Account/svg'))

WebUI.click(findTestObject('Object Repository/Page_My Wish List/i_146.00_fas fa-cart-plus'))

WebUI.click(findTestObject('Object Repository/Page_My Wish List/a_Checkout'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/label_I have read and agree to the Terms  C_2d56d6'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/button_Continue'))

WebUI.click(findTestObject('Object Repository/Page_Confirm Order/button_Confirm Order'))

WebUI.click(findTestObject('Object Repository/Page_Your order has been placed/h1_Your order has been placed'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Page_Your order has been placed/h1_Your order has been placed'))

