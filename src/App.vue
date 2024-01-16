<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

type Classes = {
  subject_id : string,
  class_id : string,
  included_id: string,
  class_title: string,
  credit: string,
  note: string,
  data: string,
  location: string,
  validity: string
}

let classesList = ref<Classes[]>([]);

let selected = ref("");
let message = ref("");
let textboxValue = ref("");
const result = ref<string | null >("");
const select = async () => {
  result.value = (await open({
    multiple: false,
    filters: [{
      name: "Spreadsheet",
      extensions: ["xlsx", "xls", "csv"]
    }],
  })) as string | null;
  parse();
};


async function parse() {
  if (result.value != null){
    selected.value = result.value;
    message.value = "You selected: " + result;
    try{
    const response = await invoke('parse_file', { path: selected.value });
    message.value = response as string;
    } catch (error) {
      console.log(error);
    }
  } else {
    message.value = "You didn't select anything";
  }
}

async function submit(){
  try {
    const response = await invoke('sort_by_class_id', { subjectId: textboxValue.value });
    classesList.value = response as Classes[];
    console.log(response)
  } catch (error) {
    console.log(error);
  }
}
</script>

<template>
  <div class="flex items-center mt-4 ml-4">
    <button class="btn btn-primary btn-s" @click="select">Chọn file Excel</button>
    <div v-if="message" class="ml-2">{{ message }}</div> 
  </div>
  <div class="flex items-center mt-4 ml-4">
    <input type="text" v-model="textboxValue" class="form-input" placeholder="Mã học phần">
    <button class="btn btn-primary ml-2" @click="submit">Submit</button>
  </div>
  <div v-for="(item, index) in classesList" :key="index">
      <p>{{ item.subject_id }}</p>
      <p>{{ item.class_id }}</p>
      <p>{{ item.included_id }}</p>
      <p>{{ item.class_title }}</p>
      <p>{{ item.credit }}</p>
      <p>{{ item.note }}</p>
      <p>{{ item.data }}</p>
      <p>{{ item.location }}</p>
      <p>{{ item.validity }}</p>
    </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
