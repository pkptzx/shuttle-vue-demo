# shuttle 使用说明

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
 
## QA
1. 不想git提交前端编译后的文件,导致部署失败  
    这是由于如果在.gitignore忽略了前端编译的文件那么shuttle也会忽略.  
    解决办法: https://discord.com/channels/803236282088161321/1072652298678960269/1075338035601879130  
    使用.ignore再设置为不忽略,这样git会忽略,shuttle不会忽略
2. 遇到部署时卡住很久很久或爆500错误后再也无法部署  
    重启这个项目:    
    ```shell
    cargo shuttle project rm
    cargo shuttle project new --idle-minutes 0
    ```
3. shuttle升级后默认30分钟后进入休眠
   cargo shuttle project new --idle-minutes 0 禁用休眠

cargo shuttle project restart --idle-minutes 0
cargo shuttle deploy --allow-dirty --no-test --working-directory ./API