# Shuttle Demo
后端: Shuttle + poem   
前端: Vue 3 + TypeScript + Vite + pnpm

demo 地址:   
<a href="https://myqr-80wg.shuttle.app" target="_blank">https://myqr-80wg.shuttle.app</a>  
<a href="https://myqr-80wg.shuttle.app/hello" target="_blank">https://myqr-80wg.shuttle.app/hello</a>

api demo https://myqr-80wg.shuttle.app/api/hello2?name=jack  
swagger doc example https://myqr-80wg.shuttle.app/docs  

custom domain: [https://myqr.lazy.icu](https://myqr.lazy.icu)  

run local :  
 
cargo shuttle run --external --port 8123  

本地自动重新加载
shuttle run --bacon

shuttle不再维护cargo watch的方式也能够使用
# This will execute `shuttle run` when you save a file.
cargo watch -s 'shuttle run'
# This will also (q)uietly (c)lear the console between runs.
cargo watch -qcs 'shuttle run'

# 部署(但还是推荐github actions自动部署)
cargo shuttle deploy --working-directory ./API --allow-dirty --no-test  

 --allow-dirty  



需要配置: 
XF_QWEN_API_KEY
XF_QWEN_TRANSLATE_PROMPT (可省,用默认)