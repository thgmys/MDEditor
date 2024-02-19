<script setup>
import { marked } from "marked";
import { debounce } from "lodash-es";
import { ref, computed } from "vue";

const input = ref("# Welcome to MDEditor");

const output = computed(() => marked(input.value));

const update = debounce((e) => {
  input.value = e.target.value;
}, 100);
</script>

<template>
  <div class="topnav">
    <a class="active" href="#home">name of file</a>
    <a href="#about">New File</a>
    <a href="#news">Save File</a>
    <a href="#contact">Open File</a>
  </div>
  <div class="editor">
    <textarea class="input" :value="input" @input="update"></textarea>
    <div class="output" v-html="output"></div>
  </div>
</template>

<!--

<script setup>
import { ref } from 'vue'

const message = ref('Hello World!')

function reverseMessage() {
  // Access/mutate the value of a ref via
  // its .value property.
  message.value = message.value.split('').reverse().join('')
}

function notify() {
  alert('navigation was prevented.')
}
</script>

<template>

  <h1>{{ message }}</h1>

  
  <button @click="reverseMessage">Reverse Message</button>

  <button @click="message += '!'">Append "!"</button>


  <a href="https://vuejs.org" @click.prevent="notify">
    A link with e.preventDefault()
  </a>
</template>

<style>
button, a {
  display: block;
  margin-bottom: 1em;
}
</style> 



<script setup>
import { ref, watchEffect } from 'vue'

const API_URL = `https://api.github.com/repos/vuejs/core/commits?per_page=3&sha=`
const branches = ['main', 'v2-compat']

const currentBranch = ref(branches[0])
const commits = ref(null)

watchEffect(async () => {
  // this effect will run immediately and then
  // re-run whenever currentBranch.value changes
  const url = `${API_URL}${currentBranch.value}`
  commits.value = await (await fetch(url)).json()
})

function truncate(v) {
  const newline = v.indexOf('\n')
  return newline > 0 ? v.slice(0, newline) : v
}

function formatDate(v) {
  return v.replace(/T|Z/g, ' ')
}
</script>

<template>
  <h1>Latest Vue Core Commits</h1>
  <template v-for="branch in branches">
    <input type="radio"
      :id="branch"
      :value="branch"
      name="branch"
      v-model="currentBranch">
    <label :for="branch">{{ branch }}</label>
  </template>
  <p>vuejs/vue@{{ currentBranch }}</p>
  <ul>
    <li v-for="{ html_url, sha, author, commit } in commits">
      <a :href="html_url" target="_blank" class="commit">{{ sha.slice(0, 7) }}</a>
      - <span class="message">{{ truncate(commit.message) }}</span><br>
      by <span class="author">
        <a :href="author.html_url" target="_blank">{{ commit.author.name }}</a>
      </span>
      at <span class="date">{{ formatDate(commit.author.date) }}</span>
    </li>
  </ul>
</template>

<style>
a {
  text-decoration: none;
  color: #42b883;
}
li {
  line-height: 1.5em;
  margin-bottom: 20px;
}
.author,
.date {
  font-weight: bold;
}
</style>



<script setup>
import { ref } from 'vue'
import TreeItem from './TreeItem.vue'

const treeData = ref({
  name: 'My Tree',
  children: [
    { name: 'hello' },
    { name: 'world' },
    {
      name: 'child folder',
      children: [
        {
          name: 'child folder',
          children: [{ name: 'hello' }, { name: 'world' }]
        },
        { name: 'hello' },
        { name: 'world' },
        {
          name: 'child folder',
          children: [{ name: 'hello' }, { name: 'world' }]
        }
      ]
    }
  ]
})
</script>

<template>
  <ul>
    <TreeItem class="item" :model="treeData"></TreeItem>
  </ul>
</template>

<style>
.item {
  cursor: pointer;
  line-height: 1.5;
}
.bold {
  font-weight: bold;
}
</style>








<script setup>
import { ref, shallowReactive, toRaw } from 'vue'

const history = shallowReactive([[]])
const index = ref(0)
const circles = ref([])
const selected = ref()
const adjusting = ref(false)

function onClick({ clientX: x, clientY: y }) {
  if (adjusting.value) {
    adjusting.value = false
    selected.value = null
    push()
    return
  }

  selected.value = [...circles.value].reverse().find(({ cx, cy, r }) => {
    const dx = cx - x
    const dy = cy - y
    return Math.sqrt(dx * dx + dy * dy) <= r
  })

  if (!selected.value) {
    circles.value.push({
      cx: x,
      cy: y,
      r: 50
    })
    push()
  }
}

function adjust(circle) {
  selected.value = circle
  adjusting.value = true
}

function push() {
  history.length = ++index.value
  history.push(clone(circles.value))
  console.log(toRaw(history))
}

function undo() {
  circles.value = clone(history[--index.value])
}

function redo() {
  circles.value = clone(history[++index.value])
}

function clone(circles) {
  return circles.map((c) => ({ ...c }))
}
</script>

<template>
  <svg @click="onClick">
    <foreignObject x="0" y="40%" width="100%" height="200">
      <p class="tip">
        Click on the canvas to draw a circle. Click on a circle to select it.
        Right-click on the canvas to adjust the radius of the selected circle.
      </p>
    </foreignObject>
    <circle
      v-for="circle in circles"
      :cx="circle.cx"
      :cy="circle.cy"
      :r="circle.r"
      :fill="circle === selected ? '#ccc' : '#fff'"
      @click="selected = circle"
      @contextmenu.prevent="adjust(circle)"
    ></circle>
  </svg>

  <div class="controls">
    <button @click="undo" :disabled="index <= 0">Undo</button>
    <button @click="redo" :disabled="index >= history.length - 1">Redo</button>
  </div>

  <div class="dialog" v-if="adjusting" @click.stop>
    <p>Adjust radius of circle at ({{ selected.cx }}, {{ selected.cy }})</p>
    <input type="range" v-model="selected.r" min="1" max="300">
  </div>
</template>

<style>
body {
  margin: 0;
  overflow: hidden;
}

svg {
  width: 100vw;
  height: 100vh;
  background-color: #eee;
}

circle {
  stroke: #000;
}

.controls {
  position: fixed;
  top: 10px;
  left: 0;
  right: 0;
  text-align: center;
}

.controls button + button {
  margin-left: 6px;
}

.dialog {
  position: fixed;
  top: calc(50% - 50px);
  left: calc(50% - 175px);
  background: #fff;
  width: 350px;
  height: 100px;
  padding: 5px 20px;
  box-sizing: border-box;
  border-radius: 4px;
  text-align: center;
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.25);
}

.dialog input {
  display: block;
  width: 200px;
  margin: 0px auto;
}

.tip {
  text-align: center;
  padding: 0 50px;
  color: #bbb;
}
</style>


















-->
