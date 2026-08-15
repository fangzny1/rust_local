<script  setup>
import { onMounted, ref } from 'vue';
const entries=ref([])
const errorMsg =ref("")
const currentPath=ref('')
async function loadlist() {
try {
 const res=await fetch(`http://127.0.0.1:8099/file/${currentPath.value}`)
    
  entries.value =await res.json()
} catch (error) {
 
  errorMsg.value=String(error)
}
}
 function openEntry(entry){
 try {
   if(entry.is_dir){
    currentPath.value =currentPath.value+entry.name+`/`
  
    loadlist()
  }
 } catch (error) {
  console.log(error);
  
 }
}
function goUP(){
  const parts =currentPath.value.split('/').filter(p=>p !=='')
  parts.pop()
  currentPath.value =parts.length? parts.join('/')+'/':''
  loadlist()
}

onMounted(loadlist)

</script>
<template>
  <p v-if="errorMsg" style="color: red;">{{ errorMsg }}</p>
  <ul>
    <li v-for="entry in entries" :key="entry.name" @click="openEntry(entry)">
    <span v-if="entry.is_dir">📁</span>
    <span v-else>📄</span>
        {{ entry.name }}
    <span v-if="!entry.is_dir">{{ entry.size }}</span>
    </li>
  </ul>
  <button v-if="currentPath" @click="goUP()">返回上级</button>
</template>