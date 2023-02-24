<template>
  <!-- <div class="box">
    <div ref="preview" v-on:paste="handlePaste" style="height: 300px;background-color: darkgray;text-align: center;">
      <span>将图片按Ctrl+V 粘贴至此处</span>
    </div>
    <button class="btn" @click="uploadPlans">上传并解析</button>
  </div> -->

  <div class="card card-compact w-96 bg-base-100 shadow-xl">
  <figure ref="preview" v-on:paste="handlePaste"><div class="grid w-96 h-96 bg-base-300 place-items-center">将图片按Ctrl+V 粘贴至此处</div></figure>
  <div class="card-body">
    <h2 class="card-title">解析二维码</h2>
    <p>如果二维码中是一个URL那么将会返回请求URL后的结果</p>
    <div class="card-actions justify-end">
      <button class="btn btn-primary"  @click="uploadPlans">上传并解析</button>
    </div>
  </div>
</div>
</template>
<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
//@ts-ignore
import '//cdn.staticfile.org/sweetalert2/11.7.2/sweetalert2.all.min.js';
let file:File|null;
const preview = ref()

onMounted(async () => {
  preview.value.focus()
  document.body.addEventListener('paste',handlePaste)
})
onUnmounted(async () => {
  document.body.removeEventListener('paste',handlePaste)
})

function handlePaste(event:ClipboardEvent) {
      const items = event.clipboardData?.items;
      file = null;

      if (!items || items.length === 0) {
        console.error("当前浏览器不支持本地");
        //@ts-ignore
        Sweetalert2.fire({
          title: 'Error!',
          text: '当前浏览器不支持本地',
          icon: 'error',
          confirmButtonText: '好'
        })
        return;
      }
      // 搜索剪切板items
      for (let i = 0; i < items.length; i++) {
        if (items[i].type.indexOf("image") !== -1) {
          file = items[i].getAsFile();
          break;
        }
      }
      if (!file) {
        //@ts-ignore
        Sweetalert2.fire({
          title: 'Error!',
          text: '粘贴内容非图片',
          icon: 'error',
          confirmButtonText: '好'
        })
        console.error("粘贴内容非图片");
        return;
      }
      // 此时file就是我们的剪切板中的图片对象
      // 如果需要预览，可以执行下面代码
      const reader = new FileReader();
      reader.onload = (event:ProgressEvent<FileReader>) => {
        console.log('event',event)
        // console.log('target',event.currentTarget?.result)
        preview.value.innerHTML = `<img style="height: 300px;" src="${(event.currentTarget as FileReader).result}">`;
      };
      reader.readAsDataURL(file);
      console.log(file)
    }
function uploadPlans() {
      if (!file) {
        console.error("请粘贴图片后上传");
        //@ts-ignore
        Sweetalert2.fire({
          title: 'Error!',
          text: '请粘贴图片后上传',
          icon: 'error',
          confirmButtonText: '好'
        })
        return;
      }
      // loading = true;
      const form = new FormData();
      form.append("file", file);
      // form.append("type", this.type);
      fetch('/upload', {
        method: 'POST',
        body: form,
        // headers: {
        //   'Content-Type': 'multipart/form-data'
        // }
      }).then(res => {
        if(res.ok) {
          console.log('success')
          return res.json();
        } else {
          //@ts-ignore
          Sweetalert2.fire({
          title: 'Error!',
          text: '服务端异常,请稍后重试',
          icon: 'error',
          confirmButtonText: '好'
        })
          console.log('error')
        }
      }).then(res => {
        console.log('res is',res);
        const r = document.createElement('div')
        if(res.result_origin){
          r.innerHTML = `${res.result_origin}<br/>${res.result}`
        }else{
          r.innerHTML = `${res.result}`
        }
        document.body.appendChild(r)
      })
    }
</script>
<style>
@import url('https://lf9-cdn-tos.bytecdntp.com/cdn/expire-1-M/tailwindcss/2.2.19/tailwind.min.css');
@import "https://lf9-cdn-tos.bytecdntp.com/cdn/expire-1-M/daisyui/2.2.2/full.min.css";
</style>