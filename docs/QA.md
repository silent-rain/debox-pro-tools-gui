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
