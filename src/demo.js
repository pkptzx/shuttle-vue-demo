import { getTranslationText,getAudio } from "lingva-scraper";
 
// import * as fs from 'fs';

getTranslationText("auto", "en", "冬灰条吃了吗").then(res=>{
    console.log(res)
    getAudio("en", res).then(resa=>{
        // return fs.createWriteStream("apple.mp3")
        console.log( resa )
        let src= 'data:audio/mp3;base64,' + Buffer.from(resa).toString("base64");
         
        console.log(src)
        // let audio = new Audio();
        // audio.src = src;
        // audio.load();
        // audio.play();
    })
})