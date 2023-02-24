<template>
  <div class="subtitles-renderer" style="color:white;">
    {{ subtitles }}
  </div>
</template>
<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, reactive, watch } from "vue";
import { useSpeechRecognition } from '@vueuse/core'
const subtitles = ref('请允许浏览器弹出的使用麦克风,请使用Edge浏览器')
const speech = useSpeechRecognition({
  lang: 'zh-CN',
  interimResults: true,
  continuous: true,
})
const zimu_max_length = 50
let pos = zimu_max_length / 2
let keyword: string
async function startSpeechRecognition() {
  // speech.result.value = ''
  speech.start()
  watch(speech.error, () => {
    console.log('异常',speech.error)
  })
  watch(speech.result, () => {
    console.log('结果',speech.result.value)
    if (speech.result.value.length >= zimu_max_length) {
      // pos 截取的问题是 返回的内容不是一个字一个字的增加
      //改为用固定结尾的字取。
      if (!keyword) {
        keyword = speech.result.value.slice(-pos - 5, -pos)
      }
      pos = speech.result.value.lastIndexOf(keyword)
      if (pos == -1) {
        console.log(speech.result.value)
        console.log('keyword', keyword)
        console.log('%c见鬼,识别完整句后keyword中间被插入标点了,重新算keyword...', 'color: red')
        pos = zimu_max_length / 2
        keyword = speech.result.value.slice(-pos - 5, -pos)
        pos = speech.result.value.lastIndexOf(keyword)
        console.log('修正新keyword', keyword)
      }
      subtitles.value = speech.result.value.slice(pos)
      if (subtitles.value.length >= zimu_max_length + 5) {
        console.log('%c字幕满了,开始清一行', 'color: blue')
        pos = zimu_max_length / 2
        keyword = speech.result.value.slice(-pos - 5, -pos)
        pos = speech.result.value.lastIndexOf(keyword)
        console.log('新keyword', keyword)
      }
    } else {
      subtitles.value = speech.result.value
    }
  })
  watch(speech.isListening, () => {
    console.log('isListening', speech.isListening.value);
    if (!speech.isListening.value) {
      console.log('isListening', speech.isListening.value);
      speech.start()
    }
  })
}
startSpeechRecognition()
</script>
<style scoped>
.subtitles-renderer {
  text-shadow: -2px -2px #000000, -2px -1px #000000, -2px 0px #000000, -2px 1px #000000, -2px 2px #000000, -1px -2px #000000, -1px -1px #000000, -1px 0px #000000, -1px 1px #000000, -1px 2px #000000, 0px -2px #000000, 0px -1px #000000, 0px 0px #000000, 0px 1px #000000, 0px 2px #000000, 1px -2px #000000, 1px -1px #000000, 1px 0px #000000, 1px 1px #000000, 1px 2px #000000, 2px -2px #000000, 2px -1px #000000, 2px 0px #000000, 2px 1px #000000, 2px 2px #000000;
  font-family: "Imprima", "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "\5FAE \8F6F \96C5 \9ED1 ", SimHei, Arial, sans-serif;
  font-size: 36px !important;
  line-height: 36px !important;
}
</style>