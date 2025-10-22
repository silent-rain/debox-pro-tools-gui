# QA

## 解决Android 9.0以上HTTP网络请求被限制问题

参考文档: <https://www.jianshu.com/p/d7776201c483>

```text
vim gen/android/app/build.gradle.kts


将值修改为 "true"
manifestPlaceholders["usesCleartextTraffic"] = "true"
```

or

```text
vim gen/android/app/src/main/AndroidManifest.xml

android:usesCleartextTraffic="${usesCleartextTraffic}">
调整为:

android:usesCleartextTraffic="true">
```

## WSL2 打包 AppImage 失败

- 错误信息

```text
failed to bundle project `failed to run linuxdeploy`
```

- 解决

```sh
# DeskTop 打包
# cargo tauri build
# AppImage 打包
NO_STRIP=true cargo tauri build
```
