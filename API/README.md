# shuttle v0.10使用说明

https://www.shuttle.rs/

## 安装或更新 shuttle
```shell
cargo install cargo-shuttle
# 如果系统磁盘剩余空间比较少,可以通过--target-dir来指定编译时产生的临时文件目录
cargo install cargo-shuttle --target-dir F:\temp
```

## 登录shuttle
```shell
cargo shuttle login abc123abc123abc123
```

## 初始化
```shell
cargo shuttle init
```

## 部署
```shell
cargo shuttle deploy
# 如果未提交git,提示异常,根据提示提交或增加参数即可
cargo shuttle deploy --allow-dirty
```

## 本地运行
```shell
cargo shuttle run
```

## 报错解决(貌似只有windows才会必须使用1.65?)
error:
>error: couldn't read \\?\x:\...\target\debug\build\rustversion-...\out/version.expr: 文件名、目录名或卷标语法不正确。 (os error 123)

answer in discord:
>You may need to do a cargo +1.65 build first, and then cargo +1.65 shuttle run for it to work on windows.

>1.66 may work too. The +<version> syntax is just to run a cargo command with a specific toolchain, if your default toolchain is that version you shouldn't need it.

1. 对项目使用指定rust版本,方式一(实际修改了.rustup/settings.toml)
    - 在工程中执行: rustup override set 1.65
    - 移除指定的版本: rustup override unset
2. 方式二:
   - 工程中创建: rust-toolchain.toml
   - 添加内容:
   
   ```toml
    [toolchain]
    channel = "1.65"
    # channel = "nightly"
    ```
3. 方式三: 每次执行时添加+1.65
    ```shell
    # 安装rust 1.65
    rustup toolchain install 1.65
    # 使用1.65重新安装cargo-shuttle (或者直接binstall[未测])
    cargo +1.65 install cargo-shuttle --force
    cargo +1.65 build
    cargo +1.65 shuttle run
    ```

## QA
1. 不想git提交前端编译后的文件,导致部署失败  
    这是由于如果在.gitignore忽略了前端编译的文件那么shuttle也会忽略.  
    解决办法: https://discord.com/channels/803236282088161321/1072652298678960269/1075338035601879130  
    使用.ignore再设置为不忽略,这样git会忽略,shuttle不会忽略
2. 遇到部署时卡住很久很久或爆500错误后再也无法部署  
    重启这个项目:    
    ```shell
    cargo shuttle project rm
    cargo shuttle project new
    ```
