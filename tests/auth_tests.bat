@echo off
setlocal EnableDelayedExpansion

echo Authentication API Test Script
echo =============================
echo.

:: Configuration
set HOST=localhost
set PORT=8002
set BASE_URL=http://%HOST%:%PORT%/api/v1
set TEST_EMAIL=test-%RANDOM%@example.com
set TEST_PASSWORD=TestPassword123!
set TEST_NAME=Test User

:: Parse command line arguments
:parse_args
if "%~1"=="" goto :endparse
if /i "%~1"=="--host" set HOST=%~2& shift & shift & goto :parse_args
if /i "%~1"=="--port" set PORT=%~2& shift & shift & goto :parse_args
goto :parse_args
:endparse

:: Initialize variables
set TOKEN=
set SESSION_COOKIE=
set USER_ID=
set TESTS_PASSED=true

echo Using API at %BASE_URL%
echo Test user email: %TEST_EMAIL%
echo.

:: Test registration
call :test_register
if "!TESTS_PASSED!"=="false" goto :end

:: Test login
call :test_login
if "!TESTS_PASSED!"=="false" goto :end

:: Test get current user
call :test_get_current_user
if "!TESTS_PASSED!"=="false" goto :end

:: Test update profile
call :test_update_profile
if "!TESTS_PASSED!"=="false" goto :end

:: Test change password
call :test_change_password
if "!TESTS_PASSED!"=="false" goto :end

:: Test logout
call :test_logout
if "!TESTS_PASSED!"=="false" goto :end

echo.
echo All tests completed successfully!
goto :end

:: ==================
:: Test Functions
:: ==================

:test_register
echo Testing user registration...
curl -s -X POST %BASE_URL%/auth/register ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"%TEST_EMAIL%\",\"password\":\"%TEST_PASSWORD%\",\"name\":\"%TEST_NAME%\"}" > register_response.json

findstr /C:"token" register_response.json > nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Registration failed
    type register_response.json
    set TESTS_PASSED=false
    exit /b 1
)

echo ✅ Registration successful
for /f "tokens=2 delims=:," %%a in ('findstr /C:"\"id\"" register_response.json') do (
    set USER_ID=%%a
    set USER_ID=!USER_ID:"=!
    set USER_ID=!USER_ID: =!
)

for /f "tokens=2 delims=:," %%a in ('findstr /C:"\"token\"" register_response.json') do (
    set TOKEN=%%a
    set TOKEN=!TOKEN:"=!
    set TOKEN=!TOKEN: =!
)
echo   User ID: !USER_ID!
exit /b 0

:test_login
echo Testing user login...
curl -s -X POST %BASE_URL%/auth/login ^
  -H "Content-Type: application/json" ^
  -d "{\"email\":\"%TEST_EMAIL%\",\"password\":\"%TEST_PASSWORD%\"}" > login_response.json

findstr /C:"token" login_response.json > nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Login failed
    type login_response.json
    set TESTS_PASSED=false
    exit /b 1
)

echo ✅ Login successful
for /f "tokens=2 delims=:," %%a in ('findstr /C:"\"token\"" login_response.json') do (
    set TOKEN=%%a
    set TOKEN=!TOKEN:"=!
    set TOKEN=!TOKEN: =!
)
echo   Token: !TOKEN:~0,20!...
exit /b 0

:test_get_current_user
echo Testing get current user...
curl -s -X GET %BASE_URL%/auth/me ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer !TOKEN!" > me_response.json

findstr /C:"email" me_response.json > nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Get current user failed
    type me_response.json
    set TESTS_PASSED=false
    exit /b 1
)

echo ✅ Get current user successful
exit /b 0

:test_update_profile
echo Testing update profile...
curl -s -X PUT %BASE_URL%/users/profile ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer !TOKEN!" ^
  -d "{\"first_name\":\"Test\",\"last_name\":\"User\",\"timezone\":\"UTC\",\"language\":\"en\"}" > profile_response.json

findstr /C:"message" profile_response.json > nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Update profile failed
    type profile_response.json
    set TESTS_PASSED=false
    exit /b 1
)

echo ✅ Profile update successful
exit /b 0

:test_change_password
echo Testing password change...
:: Ensure new password meets complexity requirements
set NEW_PASSWORD=NewPassword456!@Strong
curl -s -X PUT %BASE_URL%/auth/change-password ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer !TOKEN!" ^
  -d "{\"current_password\":\"%TEST_PASSWORD%\",\"new_password\":\"%NEW_PASSWORD%\"}" > pwd_change_response.json

findstr /C:"message" pwd_change_response.json > nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Password change failed
    echo Raw response:
    type pwd_change_response.json
    set TESTS_PASSED=false
    exit /b 1
)

echo ✅ Password change successful
set TEST_PASSWORD=%NEW_PASSWORD%
exit /b 0

echo ✅ Logout successful

:: Try to access protected endpoint after logout (should fail)
echo Testing access after logout...
curl -s -X GET %BASE_URL%/auth/me ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer !TOKEN!" > post_logout_response.json

findstr /C:"id" post_logout_response.json > nul
if %ERRORLEVEL% EQU 0 (
    echo ❌ Still able to access protected resource after logout
    type post_logout_response.json
    set TESTS_PASSED=false
    exit /b 1
)

echo ✅ Protected resource properly secured after logout
exit /b 0

:end
if "!TESTS_PASSED!"=="true" (
    echo.
    echo ✅ All tests passed!
) else (
    echo.
    echo ❌ Some tests failed!
)

:: Clean up temp files
del register_response.json login_response.json me_response.json profile_response.json pwd_change_response.json logout_response.json post_logout_response.json 2>nul

endlocal