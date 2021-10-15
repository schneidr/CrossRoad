# CrossRoad

CrossRoad is a browser selector for Windows. It is useful if you use multiple browsers and/or browser profiles on a regular basis. Set it as the default browser and every time an URL is opened outside of a browser window which is handled by Windows CrossRoad pops up and allows you to select the browser profile you want to use for this URL.

At a later point it will be possible to define URLs that should always be opened with the same browser profile.

## Usage

- Edit the .reg files for the correct path to the .exe file.
- import the .reg files.  
`register_browser.reg` must be imported with admin permissions, since it registers the Browser in the `HKEY_CLASSES_ROOT` branch in the registry.  
`register_application.reg` must be imported as the regular user.
- Open the Default Application settings and select CrossRoad as your Browser.

## Used libraries

- https://github.com/gabdube/native-windows-gui
- Icon made by [Freepik](https://www.freepik.com) from [www.flaticon.com](https://www.flaticon.com/)

## Coding documentation

- https://stackoverflow.com/a/32355457/212107
- https://docs.rs/native-windows-gui/1.0.12/native_windows_gui/index.html
