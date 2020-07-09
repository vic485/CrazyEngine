@echo off
echo [35mBuilding [REDACTED]engine!
echo [36mBuild script by Luuk

pushd EngineRenderer
echo Building renderer...[0m

cargo build --release

echo [36mRenderer built!

popd

echo Copying renderer...
xcopy /y EngineRenderer\target\release\engine_renderer.dll EngineCore\EngineRenderer.dll
echo Renderer copied!

pushd EngineCore

echo Building and running the engine core!
echo Please wait for the application to start.[0m
dotnet run

popd
