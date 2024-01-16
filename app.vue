<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

interface Classes {
	subject_id: string;
	class_id: string;
	included_id: string;
	class_title: string;
	credit: string;
	note: string;
	data: string;
	location: string;
	validity: string;
}

const classesList = ref<Classes[]>([]);

const selected = ref("");
const message = ref("");
const textboxValue = ref("");
const result = ref<string | null>("");
async function select(): Promise<void> {
	result.value = (await open({
		multiple: false,
		filters: [
			{
				name: "Spreadsheet",
				extensions: ["xlsx", "xls", "csv"],
			},
		],
	})) as string | null;
	void parse();
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
</script>

<template>
	<div>
		<div class="ml-4 mt-4 flex items-center">
			<button class="btn-s btn btn-primary" @click="select">
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
