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

## git 私库拉取失败

### 配置 Git 凭据

运行以下命令配置 Git 凭据：

```sh
git config --global credential.helper store
```

### 使用 `net.git-fetch-with-cli`

- 在 Cargo 配置中启用 `net.git-fetch-with-cli` ：
- 编辑 `~/.cargo/config.toml` 文件，添加以下内容

```toml
[net]
git-fetch-with-cli = true
```
