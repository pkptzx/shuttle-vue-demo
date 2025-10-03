<template>
  <main class="min-h-screen bg-background">
    <div class="mx-auto max-w-screen-2xl">
      <header class="border-b p-4 flex items-center justify-between">
        <h1 class="text-2xl font-bold">My Navigation</h1>
        <div class="flex items-center gap-2">
          <Clock class="h-5 w-5 text-gray-500" />
          <span class="text-gray-500 hidden sm:inline">
            {{ currentDate }} {{ currentTime }}
          </span>
          <button 
            @click="toggleTheme" 
            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-3 border border-input bg-background hover:bg-accent hover:text-accent-foreground"
          >
            <Sun v-if="theme === 'dark'" class="h-[1.2rem] w-[1.2rem]" />
            <Moon v-else class="h-[1.2rem] w-[1.2rem]" />
            <span class="sr-only">Toggle theme</span>
          </button>
        </div>
      </header>
      
      <div class="p-4 border-b">
        <div class="w-full max-w-3xl mx-auto">
          <div class="relative">
            <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-500" />
            <input 
              v-model="searchQuery"
              placeholder="Search bookmarks..." 
              class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 pl-9"
            />
            <button 
              v-if="searchQuery"
              @click="clearSearch"
              class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-700"
            >
              <X class="h-4 w-4" />
            </button>
          </div>
        </div>
      </div>
      
      <div v-if="searchQuery" class="px-4 py-2 bg-muted border-b">
        <div class="max-w-3xl mx-auto">
          <p v-if="totalResults > 0" class="text-sm">
            Found {{ totalResults }} bookmark{{ totalResults !== 1 ? 's' : '' }} matching "{{ searchQuery }}"
          </p>
          <p v-else class="text-sm">
            No bookmarks found matching "{{ searchQuery }}"
          </p>
        </div>
      </div>
      
      <div class="flex h-[calc(100vh-132px)]" :class="{ 'h-[calc(100vh-164px)]': searchQuery }">
        <div class="md:hidden p-4">
          <button 
            @click="isMobileMenuOpen = true"
            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 w-9 border border-input bg-background hover:bg-accent hover:text-accent-foreground"
          >
            <Menu class="h-5 w-5" />
            <span class="sr-only">Toggle category menu</span>
          </button>
          
          <Teleport to="body">
            <div v-if="isMobileMenuOpen" class="fixed inset-0 z-50 bg-black/80" @click="isMobileMenuOpen = false"></div>
            <div 
              v-if="isMobileMenuOpen" 
              class="fixed inset-y-0 left-0 z-50 sidebar bg-background p-0 shadow-lg"
            >
              <div class="py-4">
                <h2 class="px-4 text-lg font-semibold mb-2">Categories</h2>
                <div class="h-[calc(100vh-100px)] overflow-auto">
                  <div class="space-y-1 p-2">
                    <button
                      v-for="category in filteredCategories"
                      :key="category.id"
                      :class="[
                        'w-full text-left px-3 py-2 rounded-md text-sm',
                        activeCategory === category.id 
                          ? 'bg-primary text-primary-foreground' 
                          : 'hover:bg-accent hover:text-accent-foreground'
                      ]"
                      @click="scrollToCategory(category.id); isMobileMenuOpen = false"
                    >
                      {{ category.name }}
                      <span v-if="searchQuery" class="ml-1 text-xs opacity-70">
                        ({{ category.filteredBookmarks.length }})
                      </span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </Teleport>
        </div>
        
        <div class="hidden md:block sidebar border-r">
          <div class="py-4">
            <h2 class="px-4 text-lg font-semibold mb-2">Categories</h2>
            <div class="h-[calc(100vh-132px)] overflow-auto" :class="{ 'h-[calc(100vh-164px)]': searchQuery }">
              <div class="space-y-1 p-2">
                <button
                  v-for="category in filteredCategories"
                  :key="category.id"
                  :class="[
                    'w-full text-left px-3 py-2 rounded-md text-sm',
                    activeCategory === category.id 
                      ? 'bg-primary text-primary-foreground' 
                      : 'hover:bg-accent hover:text-accent-foreground'
                  ]"
                  @click="scrollToCategory(category.id)"
                >
                  {{ category.name }}
                  <span v-if="searchQuery" class="ml-1 text-xs opacity-70">
                    ({{ category.filteredBookmarks.length }})
                  </span>
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <div 
          ref="contentRef"
          class="flex-1 p-4 overflow-auto"
          @scroll="handleScroll"
        >
          <div v-if="totalResults === 0 && searchQuery" class="flex flex-col items-center justify-center h-full">
            <FileSearch class="h-16 w-16 text-muted-foreground mb-4" />
            <h3 class="text-xl font-medium mb-2">No bookmarks found</h3>
            <p class="text-muted-foreground text-center max-w-md">
              No bookmarks match your search for "{{ searchQuery }}". 
              Try a different search term or clear the search.
            </p>
            <button 
              @click="clearSearch"
              class="mt-4 inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 bg-primary text-primary-foreground shadow hover:bg-primary/90"
            >
              Clear Search
            </button>
          </div>
          
          <div v-else class="space-y-8">
            <div 
              v-for="category in filteredCategories"
              :key="category.id"
              :data-category="category.id"
              v-show="category.filteredBookmarks.length > 0"
            >
              <div class="space-y-3">
                <h2 class="text-2xl font-bold tracking-tight">{{ category.name }}</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                  <a 
                    v-for="bookmark in category.filteredBookmarks"
                    :key="bookmark.name"
                    :href="bookmark.url"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="no-underline"
                  >
                    <div
                      class="bookmark-card h-auto w-full flex items-start gap-3 p-4 hover:bg-accent rounded-md border border-input bg-background text-sm"
                    >
                      <span class="text-2xl mt-1">{{ bookmark.icon }}</span>
                      <div class="flex-1 min-w-0">
                        <div class="font-medium" v-html="highlightMatch(bookmark.name)"></div>
                        <div 
                          v-if="bookmark.description" 
                          class="description-text text-muted-foreground text-xs mt-1" 
                          v-html="highlightMatch(bookmark.description)"
                          :title="bookmark.description"
                        ></div>
                      </div>
                    </div>
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <div 
      v-if="showTooltip" 
      class="tooltip fixed z-50 p-2 bg-popover text-popover-foreground text-xs rounded shadow-md max-w-xs"
      :style="tooltipStyle"
    >
      {{ tooltipContent }}
    </div>
  </main>
</template>

<script setup lang="ts">
// @ts-nocheck
import { ref, onMounted, computed, nextTick, watch, onUnmounted } from 'vue'
import { Clock, FileSearch, Menu, Moon, Search, Sun, X } from 'lucide-vue-next'

// Theme handling
const theme = ref(localStorage.getItem('theme') || 'light')
const toggleTheme = () => {
  theme.value = theme.value === 'light' ? 'dark' : 'light'
  localStorage.setItem('theme', theme.value)
  if (theme.value === 'dark') {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}

// Initialize theme
onMounted(() => {
  if (theme.value === 'dark' || 
      (theme.value === 'system' && 
       window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark')
  }
  
  // Set up event listeners for custom tooltip
  document.addEventListener('mouseover', handleMouseOver)
  document.addEventListener('mouseout', handleMouseOut)
  document.addEventListener('mousemove', handleMouseMove)
})

onUnmounted(() => {
  // Clean up event listeners
  document.removeEventListener('mouseover', handleMouseOver)
  document.removeEventListener('mouseout', handleMouseOut)
  document.removeEventListener('mousemove', handleMouseMove)
})

// Date and time
const currentDate = computed(() => {
  return new Date().toLocaleDateString()
})
const currentTime = computed(() => {
  return new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
})

// Mobile menu state
const isMobileMenuOpen = ref(false)

// Search functionality
const searchQuery = ref('')
const clearSearch = () => {
  searchQuery.value = ''
}

// Custom tooltip functionality
const showTooltip = ref(false)
const tooltipContent = ref('')
const tooltipStyle = ref({
  top: '0px',
  left: '0px'
})

const handleMouseOver = (event) => {
  // Check if the target is a description element
  if (event.target.classList.contains('description-text') || 
      event.target.parentElement?.classList.contains('description-text')) {
    const descriptionEl = event.target.classList.contains('description-text') ? 
      event.target : event.target.parentElement
    
    // Only show tooltip if the text is truncated (overflowing)
    if (descriptionEl.scrollHeight > descriptionEl.clientHeight) {
      tooltipContent.value = descriptionEl.getAttribute('title')
      showTooltip.value = true
      
      // Position the tooltip
      updateTooltipPosition(event)
    }
  }
}

const handleMouseOut = (event) => {
  if (event.target.classList.contains('description-text') || 
      event.target.parentElement?.classList.contains('description-text')) {
    showTooltip.value = false
  }
}

const handleMouseMove = (event) => {
  if (showTooltip.value) {
    updateTooltipPosition(event)
  }
}

const updateTooltipPosition = (event) => {
  // Position the tooltip near the cursor
  const offset = 15 // Distance from cursor
  tooltipStyle.value = {
    top: `${event.clientY + offset}px`,
    left: `${event.clientX + offset}px`
  }
}

// 书签数据（保持原内容不变）
const bookmarkData = {
  search: [
    { name: "百度一下，你就知道", url: "https://www.baidu.com/", icon: "🔍", description: "中国最大的搜索引擎，提供网页、图片、视频等多种搜索服务" },
    { name: "开发者搜索-beta", url: "https://kaifa.baidu.com", icon: "👨‍💻", description: "百度专为开发者提供的搜索服务，帮助开发者快速找到相关技术资源" },
    { name: "微软 Bing 搜索 - 国内版", url: "http://www.bing.com/?mkt=zh-CN", icon: "🔎", description: "微软推出的搜索引擎，提供网页、图片、视频等搜索服务的国内版" },
    { name: "Google", url: "https://www.google.com", icon: "🌐", description: "全球最大的搜索引擎，提供多语言搜索服务和各种网络应用" },
    { name: "秘塔AI搜索", url: "https://metaso.cn/", icon: "🤖", description: "基于AI技术的智能搜索引擎，提供更精准的搜索结果" },
  ],
  development: [
    { name: "GitHub", url: "https://github.com/", icon: "🐙", description: "全球最大的代码托管平台，为开发者提供Git仓库服务" },
    { name: "码云 Gitee", url: "https://gitee.com/", icon: "🌲", description: "基于Git的代码托管和研发协作平台，国内版GitHub替代品" },
    { name: "Coding.net", url: "https://coding.net/", icon: "💻", description: "一站式软件研发管理协作平台，提供代码托管、项目管理等服务" },
    { name: "开源中国", url: "https://www.oschina.net/", icon: "🏗️", description: "找到您想要的开源项目，分享和交流的开发者社区" },
    { name: "博客园", url: "https://www.cnblogs.com/", icon: "📝", description: "开发者的网上家园，提供博客写作和技术交流平台" },
    { name: "CSDN论坛", url: "http://bbs.csdn.net/home", icon: "🔧", description: "IT技术交流平台，提供编程技术问答和学习资源" },
    { name: "V2EX", url: "https://www.v2ex.com/", icon: "💬", description: "创意工作者的社区，关注编程、设计和创业等话题" },
    { name: "掘金", url: "https://juejin.im/", icon: "⛏️", description: "一个帮助开发者成长的社区，提供优质技术文章和讨论" },
    { name: "黑客派", url: "https://hacpai.com/", icon: "🔐", description: "为未来而构建的开发者社区，找到技术解决方案" },
    { name: "发现优质编程学习资源", url: "https://www.code-nav.cn/", icon: "🧭", description: "编程导航，帮助开发者发现高质量的编程学习资源" },
  ],
  tools: [
    { name: "Shell NGN", url: "https://app.shellngn.com/", icon: "🐚", description: "Web Based SSH Client，在浏览器中使用SSH连接服务器" },
    { name: "Play with Docker", url: "https://labs.play-with-docker.com/", icon: "🐳", description: "在线Docker实验环境，无需安装即可体验Docker" },
    { name: "小书匠", url: "http://soft.xiaoshujiang.com/", icon: "📔", description: "Markdown编辑器，支持多种格式导出的写作工具" },
    { name: "Hurricane Electric Hosted DNS", url: "https://dns.he.net/", icon: "🌀", description: "免费的DNS托管服务，提供域名解析管理" },
    { name: "Postimages", url: "https://postimages.org/", icon: "🖼️", description: "免费图片托管/图片上传服务，无需注册" },
    { name: "在线二维码解码器", url: "https://jiema.wwei.cn/", icon: "📱", description: "二维码安全检测工具，解析二维码内容" },
    { name: "在线二维码解码器", url: "https://jie.2weima.com/", icon: "🔍", description: "二维码批量解码工具，支持多个二维码同时解析" },
    { name: "Containers - goormIDE", url: "https://ide.goorm.io/", icon: "📦", description: "云端集成开发环境，支持多种编程语言" },
    { name: "ZeroTier", url: "https://www.zerotier.com/", icon: "🔗", description: "Global Area Networking，构建虚拟网络的工具" },
    { name: "HexEd.it", url: "https://hexed.it/", icon: "🔢", description: "基于浏览器的在线十六进制编辑器，支持离线使用" },
  ],
  ai: [
    { name: "ChatGPT", url: "https://chat.openai.com/", icon: "🤖", description: "OpenAI开发的大型语言模型，可进行自然语言对话" },
    { name: "ChatGPT Next Web", url: "https://chatgpt.nextweb.fun", icon: "🌐", description: "ChatGPT的网页客户端，提供更好的用户体验" },
    { name: "chat-shared.zhile.io", url: "https://chat-shared.zhile.io/shared.html", icon: "💬", description: "ChatGPT共享服务，无需账号即可使用" },
    { name: "文心一言", url: "https://yiyan.baidu.com/", icon: "🧠", description: "百度推出的人工智能语言模型，支持中文对话和创作" },
    { name: "讯飞星火认知大模型", url: "https://xinghuo.xfyun.cn/desk", icon: "✨", description: "科大讯飞推出的AI大语言模型，专注中文理解" },
    { name: "通义千问", url: "https://tongyi.aliyun.com/qianwen/", icon: "❓", description: "阿里云推出的大语言模型，擅长知识问答和对话" },
    { name: "来自 Microsoft 必应的图像创建者", url: "https://www.bing.com/images/create", icon: "🎨", description: "微软必应提供的AI图像生成服务" },
    { name: "Chat Blackbox", url: "https://www.blackbox.ai", icon: "⬛", description: "AI代码生成、代码聊天、代码搜索工具" },
    { name: "DeepSeek - 探索未至之境", url: "https://chat.deepseek.com/", icon: "🔭", description: "专注于深度学习和AI研究的对话模型" },
    { name: "Kimi.ai - 帮你看更大的世界", url: "https://kimi.moonshot.cn/", icon: "🌙", description: "月之暗面推出的AI助手，拥有强大的知识库" },
    { name: "豆包MarsCode - 工作台", url: "https://www.marscode.cn", icon: "🚀", description: "字节跳动推出的AI编程助手，提供代码生成和优化" },
    { name: "smithery.ai", url: "https://smithery.ai/", icon: "⚒️", description: "AI内容创作和编辑工具，提升写作效率" },
  ],
  media: [
    { name: "哔哩哔哩", url: "https://www.bilibili.com/", icon: "📺", description: "中国年轻人喜爱的视频弹幕网站，提供动画、游戏、科技等内容" },
    { name: "优酷首页", url: "http://www.youku.com/", icon: "▶️", description: "中国领先的视频网站，提供电影、电视剧、综艺等内容" },
    { name: "YouTube", url: "https://www.youtube.com/", icon: "📹", description: "全球最大的视频分享平台，提供各类视频内容" },
    { name: "Rabbit", url: "https://www.rabb.it/", icon: "🐰", description: "Watch Anything. With Anyone. Anytime. 在线共享观影平台" },
    { name: "茶杯狐官方", url: "https://www.cbportal.org/", icon: "🦊", description: "努力让找电影变得简单，电影搜索引擎" },
    { name: "放屁网", url: "https://www.fangpi.net/", icon: "🎵", description: "全网音乐MP3高品质在线免费下载、在线免费播放" },
    { name: "musicForProgramming", url: "https://musicforprogramming.net", icon: "🎧", description: "为编程提供的专注音乐，提高工作效率" },
    { name: "slider.kz", url: "https://hayqbhgr.slider.kz/", icon: "🔊", description: "Just another music searcher，音乐搜索和下载工具" },
  ],
  social: [
    { name: "微博", url: "http://www.weibo.com/", icon: "📱", description: "中国领先的社交媒体平台，分享生活动态和热点话题" },
    { name: "Teamind", url: "https://www.teamind.co/", icon: "👥", description: "看得见协作的远程互动会议，团队协作工具" },
    { name: "NOISE | 知识效率集", url: "https://www.noisesite.cn/", icon: "📚", description: "知识管理和效率提升平台，帮助用户整理思路" },
    { name: "flomo", url: "https://flomoapp.com/", icon: "💭", description: "卡片笔记工具，帮助收集和整理碎片化思考" },
    { name: "Teamind", url: "https://www.teamind.co/", icon: "🤝", description: "看得见协作的远程互动会议平台" },
  ],
  shopping: [
    { name: "京东", url: "https://www.jd.com/", icon: "🛒", description: "中国领先的自营式电商企业，提供丰富的商品和快速配送" },
    { name: "淘宝网", url: "https://www.taobao.com/", icon: "🛍️", description: "淘！我喜欢，中国最大的网上购物平台" },
  ],
  code: [
    { name: "Dashboard – Vercel", url: "https://vercel.com", icon: "▲", description: "前端开发平台，提供静态网站和Serverless函数部署" },
    { name: "Glitch", url: "https://glitch.com/", icon: "🎏", description: "友好的社区，每个人都在这里构建网络应用" },
    { name: "CodeSandbox", url: "https://codesandbox.io/", icon: "📦", description: "在线代码编辑器和原型工具，快速创建Web应用" },
    { name: "CodePen", url: "https://codepen.io/", icon: "🖊️", description: "前端设计师和开发者的社交开发环境" },
    { name: "CSS-Tricks", url: "https://css-tricks.com/", icon: "🎨", description: "关于制作网站的网站，提供CSS技巧和教程" },
    { name: "All | NavNav+", url: "https://navnav.co/", icon: "🧭", description: "网页导航组件集合，提供各种导航设计灵感" },
    { name: "UI Store", url: "http://uistore.org/", icon: "🎯", description: "免费UI资源下载，包括UI套件、模板和图标" },
    { name: "LogicFlow", url: "http://logic-flow.org/", icon: "📊", description: "流程图编辑框架，提供可视化的流程设计工具" },
    { name: "Search Icons & Find the Perfect Design", url: "https://fontawesome.com/v6/search", icon: "🔍", description: "Font Awesome图标搜索，找到完美的图标设计" },
    { name: "Icônes", url: "https://icones.js.org/", icon: "🖼️", description: "图标搜索和管理工具，支持多个图标库" },
    { name: "Open-Source UI elements", url: "https://uiverse.io/", icon: "🌌", description: "使用CSS和HTML制作的开源UI元素库" },
  ],
  learning: [
    { name: "Design Pattern Catalog", url: "https://java-design-patterns.com/patterns/", icon: "📐", description: "Java设计模式目录，提供各种设计模式的实现和解释" },
    { name: "Lab - Work with Microsoft Office integration", url: "https://learn.microsoft.com/en-us/training/modules", icon: "📊", description: "Microsoft Learn培训模块，学习Office集成功能" },
    { name: "Sysinternals 实用工具", url: "https://learn.microsoft.com/zh-cn/sysinternals/downloads/", icon: "🔧", description: "微软提供的系统内部工具，用于Windows系统诊断和管理" },
    { name: "MSDN, 我告诉你", url: "https://msdn.itellyou.cn/", icon: "💿", description: "提供微软MSDN资源下载的网站" },
    { name: "PyCharm激活码最新2022永久免费提供", url: "https://vrg123.com/", icon: "🔑", description: "提供JetBrains系列产品激活码的网站" },
    { name: "pandas.PeriodIndex.week", url: "https://avxdhg-data.oss.lafyun.com/pandas/reference/api/pandas.PeriodIndex.week.html", icon: "🐼", description: "pandas库文档，关于PeriodIndex.week的API参考" },
  ],
  utility: [
    { name: "佐糖-免费在线抠图神器", url: "https://picwish.cn/", icon: "✂️", description: "证件照换底色、照片修复、无损压缩创意图像平台" },
    { name: "免费在线图片/视频压缩工具", url: "http://www.yalijuda.com/", icon: "📉", description: "免费JPG、PNG、GIF图像压缩服务" },
    { name: "Google Translate APIs", url: "https://get.hexingxing.cn/gtapis/", icon: "🌐", description: "谷歌翻译API服务，提供多语言翻译功能" },
    { name: "Welcome to sslip.io", url: "https://sslip.io/", icon: "🔒", description: "DNS服务，将IP地址映射为域名，便于开发测试" },
    { name: "文本转语音", url: "https://azure.microsoft.com/zh-cn/products/cognitive-services/text-to-speech/", icon: "🔊", description: "微软Azure提供的真实AI语音生成器" },
    { name: "Weblate", url: "https://weblate.org/zh-hans/", icon: "🌍", description: "基于Web的在线本地化平台，用于翻译项目" },
    { name: "简单云链 EasyLink", url: "https://easylink.cc/", icon: "🔗", description: "免费文件转链接工具，二维码生成器" },
    { name: "文叔叔", url: "https://www.wenshushu.cn/", icon: "📁", description: "传文件，找文叔叔（大文件、永不限速）" },
    { name: "PP直连", url: "https://www.ppzhilian.com/", icon: "📡", description: "点对点文件传输服务，无需上传到服务器" },
    { name: "钛盘", url: "https://app.tmp.link/", icon: "💾", description: "超好用的文件中转站，临时文件存储服务" },
    { name: "Send", url: "https://send.vis.ee/", icon: "📤", description: "安全、私密的文件分享服务" },
    { name: "世界著名的免费摄影图库", url: "https://pxhere.com", icon: "📷", description: "素材中国，高清壁纸 - PxHere摄影图库" },
    { name: "Picshack.net", url: "https://picshack.net/", icon: "🖼️", description: "Upload and share your images，图片上传和分享服务" },
    { name: "Free Cloud Hosting by OnWorks", url: "https://www.onworks.net/", icon: "☁️", description: "免费的云主机服务，支持多种操作系统" },
    { name: "Cloud Application Hosting for Developers", url: "https://render.com/", icon: "🚀", description: "为开发者提供的云应用托管服务" },
    { name: "shuttle", url: "https://www.shuttle.rs/", icon: "🚀", description: "Rust语言开发的应用部署工具" },
  ],
  other: [
    { name: "攻防世界", url: "https://adworld.xctf.org.cn", icon: "🛡️", description: "网络安全学习平台，提供CTF训练" },
    { name: "后台管理系统", url: "http://vmms.ourvend.com:83/YSTemplet/index#", icon: "🖥️", description: "企业后台管理系统模板" },
    { name: "Free shadowsocks", url: "https://free.gyteng.com/", icon: "🔑", description: "提供免费shadowsocks服务的网站" },
    { name: "串口液晶论坛", url: "http://bbs.feelelec.cn", icon: "💻", description: "Powered by FeelElec，专注串口液晶技术讨论" },
    { name: "网站状态监控", url: "https://stats.uptimerobot.com/qHojtzLD1g", icon: "👨‍💻", description: "网站状态监控" },
    { name: "代码统计 – Code::Stats", url: "https://codestats.net", icon: "📊", description: "代码统计服务，记录编程活动" },
    { name: "ithome.com/rss/", url: "https://www.ithome.com/rss/", icon: "📰", description: "IT之家RSS订阅源，获取最新IT资讯" },
  ]
}

// Convert the data object to an array of categories
const categories = Object.entries(bookmarkData).map(([id, bookmarks]) => ({
  id,
  name: id.charAt(0).toUpperCase() + id.slice(1), // Capitalize first letter
  bookmarks
}))

// Fuzzy search functions
const normalizeText = (text) => {
  if (!text) return ''
  return text.toLowerCase().replace(/\s+/g, ' ').trim()
}

const calculateSimilarity = (str1, str2) => {
  // Simple implementation of Levenshtein distance (edit distance)
  const a = normalizeText(str1)
  const b = normalizeText(str2)
  
  // If either string is empty, the distance is the length of the other string
  if (a.length === 0) return b.length
  if (b.length === 0) return a.length
  
  // For small strings, we can use a full matrix approach
  const matrix = []
  
  // Initialize the matrix
  for (let i = 0; i <= a.length; i++) {
    matrix[i] = [i]
  }
  
  for (let j = 0; j <= b.length; j++) {
    matrix[0][j] = j
  }
  
  // Fill the matrix
  for (let i = 1; i <= a.length; i++) {
    for (let j = 1; j <= b.length; j++) {
      const cost = a[i - 1] === b[j - 1] ? 0 : 1
      matrix[i][j] = Math.min(
        matrix[i - 1][j] + 1,      // deletion
        matrix[i][j - 1] + 1,      // insertion
        matrix[i - 1][j - 1] + cost // substitution
      )
    }
  }
  
  // Calculate similarity score (0-1 where 1 is perfect match)
  const maxLength = Math.max(a.length, b.length)
  if (maxLength === 0) return 1 // Both strings are empty
  
  const distance = matrix[a.length][b.length]
  return 1 - (distance / maxLength)
}

// Check if a bookmark matches the search query with fuzzy matching
const fuzzyMatch = (bookmark, query) => {
  if (!query.trim()) return true
  
  const normalizedQuery = normalizeText(query)
  const normalizedName = normalizeText(bookmark.name)
  const normalizedUrl = normalizeText(bookmark.url)
  const normalizedDescription = normalizeText(bookmark.description || '')
  
  // Check for exact substring match first (faster)
  if (normalizedName.includes(normalizedQuery) || 
      normalizedUrl.includes(normalizedQuery) || 
      normalizedDescription.includes(normalizedQuery)) {
    return true
  }
  
  // If no exact match, check for fuzzy match
  const nameSimilarity = calculateSimilarity(normalizedName, normalizedQuery)
  const urlSimilarity = calculateSimilarity(normalizedUrl, normalizedQuery)
  const descriptionSimilarity = bookmark.description ? 
    calculateSimilarity(normalizedDescription, normalizedQuery) : 0
  
  // Threshold for considering it a match (can be adjusted)
  const SIMILARITY_THRESHOLD = 0.6
  
  return nameSimilarity >= SIMILARITY_THRESHOLD || 
         urlSimilarity >= SIMILARITY_THRESHOLD || 
         descriptionSimilarity >= SIMILARITY_THRESHOLD
}

// Filtered categories based on fuzzy search
const filteredCategories = computed(() => {
  if (!searchQuery.value.trim()) {
    return categories.map(category => ({
      ...category,
      filteredBookmarks: category.bookmarks
    }))
  }
  
  return categories.map(category => {
    const filteredBookmarks = category.bookmarks.filter(bookmark => 
      fuzzyMatch(bookmark, searchQuery.value)
    )
    
    return {
      ...category,
      filteredBookmarks
    }
  })
})

// Total results count
const totalResults = computed(() => {
  return filteredCategories.value.reduce((total, category) => {
    return total + category.filteredBookmarks.length
  }, 0)
})

// Highlight matching text in search results
const highlightMatch = (text) => {
  if (!text || !searchQuery.value.trim()) return text
  
  const normalizedQuery = normalizeText(searchQuery.value)
  const normalizedText = normalizeText(text)
  
  // If there's a direct substring match, highlight it
  if (normalizedText.includes(normalizedQuery)) {
    const regex = new RegExp(`(${escapeRegExp(normalizedQuery)})`, 'gi')
    return text.replace(regex, '<span class="bg-yellow-200 dark:bg-yellow-800">$1</span>')
  }
  
  // For fuzzy matches, we'll just return the text without highlighting
  // as it's harder to determine exactly which parts should be highlighted
  return text
}

// Helper to escape special characters in regex
const escapeRegExp = (string) => {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

// Navigation state
const activeCategory = ref(categories[0].id)
const contentRef = ref(null)

// Scroll to category function
const scrollToCategory = (categoryId) => {
  nextTick(() => {
    if (contentRef.value) {
      // Find the category element by data-category attribute
      const categoryElement = contentRef.value.querySelector(`[data-category="${categoryId}"]`)
      
      if (categoryElement) {
        // Calculate position to scroll to (accounting for any padding/offset)
        const topPosition = categoryElement.offsetTop - 20
        
        // Smooth scroll to the element
        contentRef.value.scrollTo({
          top: topPosition,
          behavior: 'smooth'
        })
        
        activeCategory.value = categoryId
      }
    }
  })
}

// Handle scroll to update active category
const handleScroll = () => {
  if (!contentRef.value) return
  
  const scrollPosition = contentRef.value.scrollTop + 100 // Add offset for better detection
  
  // Find all category elements
  const categoryElements = contentRef.value.querySelectorAll('[data-category]')
  
  // Find the category that is currently most visible
  let currentCategory = categories[0].id
  
  categoryElements.forEach((element) => {
    const rect = element.getBoundingClientRect()
    const containerRect = contentRef.value.getBoundingClientRect()
    
    // Check if element is in viewport
    if (rect.top <= containerRect.top + 100) {
      currentCategory = element.getAttribute('data-category')
    }
  })
  
  if (currentCategory !== activeCategory.value) {
    activeCategory.value = currentCategory
  }
}

// Reset active category when search changes
watch(searchQuery, () => {
  // Find the first category with results
  const firstCategoryWithResults = filteredCategories.value.find(
    category => category.filteredBookmarks.length > 0
  )
  
  if (firstCategoryWithResults) {
    activeCategory.value = firstCategoryWithResults.id
    nextTick(() => {
      scrollToCategory(firstCategoryWithResults.id)
    })
  }
})
</script>

<style>
/* Base styles */
:root {
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  --card: 0 0% 100%;
  --card-foreground: 222.2 84% 4.9%;
  --popover: 0 0% 100%;
  --popover-foreground: 222.2 84% 4.9%;
  --primary: 221.2 83.2% 53.3%;
  --primary-foreground: 210 40% 98%;
  --secondary: 210 40% 96.1%;
  --secondary-foreground: 222.2 47.4% 11.2%;
  --muted: 210 40% 96.1%;
  --muted-foreground: 215.4 16.3% 46.9%;
  --accent: 210 40% 96.1%;
  --accent-foreground: 222.2 47.4% 11.2%;
  --destructive: 0 84.2% 60.2%;
  --destructive-foreground: 210 40% 98%;
  --border: 214.3 31.8% 91.4%;
  --input: 214.3 31.8% 91.4%;
  --ring: 221.2 83.2% 53.3%;
  --radius: 0.5rem;
}

.dark {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;
  --card: 222.2 84% 4.9%;
  --card-foreground: 210 40% 98%;
  --popover: 222.2 84% 4.9%;
  --popover-foreground: 210 40% 98%;
  --primary: 217.2 91.2% 59.8%;
  --primary-foreground: 222.2 47.4% 11.2%;
  --secondary: 217.2 32.6% 17.5%;
  --secondary-foreground: 210 40% 98%;
  --muted: 217.2 32.6% 17.5%;
  --muted-foreground: 215 20.2% 65.1%;
  --accent: 217.2 32.6% 17.5%;
  --accent-foreground: 210 40% 98%;
  --destructive: 0 62.8% 30.6%;
  --destructive-foreground: 210 40% 98%;
  --border: 217.2 32.6% 17.5%;
  --input: 217.2 32.6% 17.5%;
  --ring: 224.3 76.3% 48%;
}

* {
  border-color: hsl(var(--border));
}

body {
  background-color: hsl(var(--background));
  color: hsl(var(--foreground));
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  line-height: 1.5;
  margin: 0;
}

.bg-background {
  background-color: hsl(var(--background));
}

.bg-primary {
  background-color: hsl(var(--primary));
}

.text-primary-foreground {
  color: hsl(var(--primary-foreground));
}

.bg-accent {
  background-color: hsl(var(--accent));
}

.text-accent-foreground {
  color: hsl(var(--accent-foreground));
}

.border-input {
  border-color: hsl(var(--input));
}

.text-gray-500 {
  color: hsl(215.4 16.3% 46.9%);
}

.bg-muted {
  background-color: hsl(var(--muted));
}

.text-muted-foreground {
  color: hsl(var(--muted-foreground));
}

/* Utility classes */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

/* Sidebar fixed width */
.sidebar {
  width: 240px;
  flex-shrink: 0;
}

/* Fixed height for bookmark cards */
.bookmark-card {
  height: 90px;
}

/* Description text with ellipsis */
.description-text {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  max-height: 2.5rem;
}

/* Tooltip styling */
.tooltip {
  max-width: 300px;
  word-wrap: break-word;
  border: 1px solid hsl(var(--border));
}
</style>