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

cargo shuttle deploy --working-directory ./API --allow-dirty --no-test  

 --allow-dirty  