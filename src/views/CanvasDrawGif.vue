<template>
    <div>
    <canvas ref="canvas" class="example"  width="670" height="100"></canvas>
</div>
</template>
<script setup lang="ts">

import 'gifler';
import { onMounted, ref } from 'vue';
var frames = 0;

const canvas = ref()

function onDrawFrame(ctx:CanvasRenderingContext2D , frame:any) {
  // Match width/height to remove distortion
  ctx.canvas.width  = ctx.canvas.offsetWidth;
  ctx.canvas.height = ctx.canvas.offsetHeight;
  // 装作背景...
  ctx.fillText('半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖半糖',0,50)
  // Determine how many pikachus will fit on screen
  var n = Math.floor((ctx.canvas.width)/150)

  for(var x = 0; x < n; x++) {
    // Draw a pikachu
    var left = x * 150;
    // ctx.globalCompositeOperation = 'source-over';
    ctx.drawImage(frame.buffer, frame.x + left, frame.y, 150, 100);

    // Composite a color
    // var hue = (frames * 10 + x * 50) % 360;
    // ctx.globalCompositeOperation = 'source-atop';
    // ctx.fillStyle = 'hsla(' + hue + ', 100%, 50%, 0.5)';
    //@ts-ignore
    // ctx.fillRect(left, 0, 150, this.height);
  }
  frames++;
}

onMounted(async()=>{
// Load the GIF, set custom frame render function
//@ts-ignore
gifler('/run.gif')
  .frames(canvas.value, onDrawFrame);
})  
</script>