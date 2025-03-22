# Shuttle Demo
后端: Shuttle + poem   
前端: Vue 3 + TypeScript + Vite + pnpm

demo 地址:   
<a href="https://myqr.shuttleapp.rs" target="_blank">https://myqr.shuttleapp.rs</a>  
<a href="https://myqr.shuttleapp.rs/hello" target="_blank">https://myqr.shuttleapp.rs/hello</a>

api demo https://myqr.shuttleapp.rs/api/hello2?name=jack  
swagger doc example https://myqr.shuttleapp.rs/docs  

run local :
D:\rustproject\qr\API\target\release\qr.exe --port 80 --storage-manager-type working-dir  --storage-manager-path d:\rustprojet\qr\API

cargo shuttle run --external --port 8123


cargo shuttle project restart --idle-minutes 0 --working-directory ./API

cargo shuttle deploy --working-directory ./API --allow-dirty --no-test


 --allow-dirty