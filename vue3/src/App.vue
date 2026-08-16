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
async function download_Link(entry)
{
  const res=`http://127.0.0.1:8099/file/${currentPath.value}`
  const name =entry.name
  const url = String(res+name)
  window.open(url,'_blank')
}



onMounted(loadlist)

</script>
<template>
  <div class="back_Card">
  <p v-if="errorMsg" style="color: red;">{{ errorMsg }}</p>
  <ul class="entry">
    <li class="lists" v-for="entry in entries" :key="entry.name" @click="entry.is_dir? openEntry(entry):download_Link(entry)">
    <span v-if="entry.is_dir">📁</span>
    <span v-else>📄</span>
        {{ entry.name }}
    <span v-if="!entry.is_dir" >{{ entry.size }} 字节</span>
    </li>
  </ul>
   <button class="button" v-if="currentPath" @click="goUP()">←</button>
  </div>
 
</template>

<style scoped>
.lists span + span{
  margin-left: auto;
}
.entry{
  display: flex;
  flex-direction: column;
  cursor: pointer;
  margin: 20px 10px;
  flex:1;
  list-style: none;
  padding: 0px 10px;
}
.lists{
  transition: background-color  100ms ease;
  display: flex;
  align-items: center;
  gap:8px;
  padding: 8px 12px;
  border-radius: 12px;
}

.lists:hover{
  
  background-color: rgba(255, 255, 255, 0.1);
}


.back_Card{
    background-color: rgba(255, 255, 255, 0.5);
    backdrop-filter: blur(12px);
    min-width:1200px;
    min-height: 75vh;
   display: flex;
   flex-direction: column;
    border-radius: 30px;
    margin: 0px auto;
    
}
.button{
  margin: 10px 10px;
  width: 50px;
  height: 30px;
  border-radius: 30px;
    background-color: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(12px);
}



</style>