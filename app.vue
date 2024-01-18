<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import WeekView from "./components/WeekView.vue";



// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup


interface Classes {
	subject_id: string;
	class_id: string;
	included_id: string;
	class_title: string;
	credit: string;
	note: string;
	data: Time[];
	location: string;
	class_type: string;
	validity: string;
}

interface Time{
	day: string;
	start: string;
	end: string;
}

const classesList = ref<Classes[]>([]);

const isDialogOpen = ref(false);
const isFileParsed = ref(false);


const confirmClass = ref(false);

let selectedData: Classes | null = null;
const selected = ref("");
const message = ref("");
const LT_class = ref("");
const textboxValue = ref("");
const result = ref<string | null>("");

async function select(): Promise<void> {
	isDialogOpen.value = true;
	result.value = (await open({
		multiple: false,
		filters: [
			{
				name: "Spreadsheet",
				extensions: ["xlsx", "xls", "csv"],
			},
		],
	})) as string | null;
	isDialogOpen.value = false;
	void parse();
	isFileParsed.value = true;
}

function selectClass(data: Classes){
	if (data.included_id != `String("NULL")` && data.included_id != data.class_id){
		confirmClass.value = true;
		LT_class.value = data.included_id;
		selectedData = data;
		console.log(confirmClass.value);
	}
	else{
		confirmClass.value = false;
		LT_class.value = "";
		selectedData = null;
		confirm(data); 
	}
}

async function confirm(data: Classes): Promise<void>{
	try {
		const response = await invoke("add_chosen_class", {data: data})
		message.value = response as string;
		classesList.value = [];
	} catch (error) {
		console.log(error);
	}
}

async function confirmSelection(): Promise<void>{
	confirmClass.value = false;
	if (selectedData){
		await confirm(selectedData);
	}
}

function cancelSelection(): void{
	confirmClass.value = false;
	LT_class.value = "";
	selectedData = null;
}

async function parse(): Promise<void> {
	if (result.value != null) {
		selected.value = result.value;
		message.value = `You selected: ${selected.value}`;
		try {
			const response = await invoke("parse_file", {
				path: selected.value,
			});
			message.value = response as string;
		} catch (error) {
			console.log(error);
		}
	} else {
		message.value = "You didn't select anything";
	}
}

async function submit(): Promise<void> {
	try {
		const response = await invoke("sort_by_class_id", {
			subjectId: textboxValue.value,
		});
		classesList.value = response as Classes[];
		console.log(response);
	} catch (error) {
		console.log(error);
	}
}

function parse_time(time: Time[]){
	let result = "";
	for (let i = 0; i < time.length; i++) {
		const element = time[i];
		result+=(` Thứ ${element.day}: ${parse_hour(element.start)} - ${parse_hour(element.end)}\n`)
	}	
	return result;
}

function parse_hour(hhmm: any){
	if (typeof hhmm !== 'string'){
		hhmm = hhmm.toString();
	}
	return `${hhmm.substring(0,2)}:${hhmm.substring(2,4)}`
}

function require_lab(input: String){
	if (input == "TN"){
		return "Có";
	}
	else{
		return "Không";
	}
}

function parse_included_id(included_id: String){
	if (included_id == `String("NULL")`)
		return "Không có";
	else
		return included_id;
}

// Watch for changes in confirmClass

</script>


<template>
	<div>
		<div class="ml-4 mt-4 flex items-center">
			<button class="btn-s btn btn-primary" @click="select" :disabled="isDialogOpen">
				Chọn file Excel
			</button>
			<div v-if="message" class="ml-2">{{ message }}</div>
		</div>
		<div class="ml-4 mt-4 flex items-center">
			<input
				type="text"
				v-model="textboxValue"
				class="form-input"
				placeholder="Mã học phần"
			/>
			<button class="btn btn-primary ml-2" @click="submit" :disabled="!isFileParsed">Submit</button>
		</div>
		<div class="section">
    		<div v-for="(item, index) in classesList" :key="index" class="card border">
     			 <div class="card-body">
					<h5 class="card-title">{{ item.class_id }}</h5>
					<p class="card-text">{{ item.class_title }}</p>
					<p class="card-text">Mã học phần: {{ item.subject_id }}</p>
					<p class="card-text">Mã lớp đi kèm: {{ parse_included_id(item.included_id) }}</p>
					<p class="card-text">Tín chỉ: {{ item.credit }}</p>
					<p class="card-text">Ghi chú: {{ item.note }}</p>
					<p class="card-text">Thời gian:{{ parse_time(item.data) }}</p>
					<p class="card-text">Loại lớp: {{ item.class_type }}</p>
					<p class="card-text">Phòng học: {{ item.location }}</p>
					<p class="card-text">Yêu cầu thí nghiệm: {{ require_lab(item.class_type) }}</p>
					<p class="card-text">Đăng ký dành cho: {{ item.validity }}</p>
					<button class="btn btn-primary" @click="selectClass(item)">Choose</button>
      			</div>
    		</div>
  		</div>
	<WeekView/>
	</div>
	<dialog v-if="confirmClass" class="modal" open>
		<div class="modal-box">
			<h3 class="text-lg font-bold">Xác nhận lớp</h3>
			<p>Môn này bắt buộc phải có học phần {{ LT_class }} đi cùng. Hãy ấn OK nếu bạn chắc chắn muốn học cả 2 học phần này.</p>
			<div class = "modal-action">
				<form method = "dialog">
					<button class="btn btn-primary mr-4" @click="confirmSelection">OK</button>
					<button class="btn btn-primary" @click="cancelSelection">Huỷ bỏ</button>
				</form>
			</div>
		</div>
	</dialog>
</template>

<style scoped>
.logo.vite:hover {
	filter: drop-shadow(0 0 2em #747bff);
}

.section {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
}

.card {
  flex: 0 0 calc(33.3333% - 1em);
  margin: 0.5em;
  box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
}

.card-body {
  padding: 1em;
}

.btn {
  margin-top: 1em;
}
.logo.vue:hover {
	filter: drop-shadow(0 0 2em #249b73);
}
</style>
