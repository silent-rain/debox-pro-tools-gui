# Android-adb使用文档

## adb 路径

```sh
# Linux
~/.Android/Sdk/platform-tools/adb

# windows
C:\Users\Administrator\AppData\Local\Android\Sdk\platform-tools\adb.exe

# WSL
/mnt/c/Users/v_yufeicao/AppData/Local/Android/Sdk/platform-tools/adb.exe
```

## emulator 路径

```sh
# Linux
~/.Android/Sdk/emulator/emulator

# windows
C:\Users\Administrator\AppData\Local\Android\Sdk\emulator\emulator.exe

# WSL
/mnt/c/Users/v_yufeicao/AppData/Local/Android/Sdk/emulator/emulator.exe
```

## 查看版本

```sh
➜ adb --version
```

## 重启 adb 服务

```sh
➜ adb kill-server
➜ adb start-server
```

## 检查设备列表

```sh
➜ adb devices

List of devices attached
emulator-5554   device
```

## 本地连接

默认端口是 5555

```shell
➜ adb connect localhost:5555
```

## 手动连接模拟器

```shell
➜ adb connect emulator-5554
```

## 获取正在运行的模拟器列表

```sh
➜ emulator -list-avds
```

## 重新启动模拟器

```sh
➜ adb emu kill
➜ emulator -avd <你的AVD名称>
```

## 检查模拟器日志

```sh
➜ adb logcat
```

## 断开现有的 adb 连接

```sh
➜ adb disconnect
```
