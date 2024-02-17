<script setup>
import { marked } from 'marked'
import { debounce } from 'lodash-es'
import { ref, computed } from 'vue'

const input = ref('# Welcome to MDEditor')

const output = computed(() => marked(input.value))

const update = debounce((e) => {
  input.value = e.target.value
}, 100)
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
