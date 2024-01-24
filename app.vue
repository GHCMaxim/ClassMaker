<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import type { CurEvent, Weekday } from "./composables/globalState";

interface Classes {
	subject_id: string;
	class_id: string;
	included_id: string;
	class_title: string;
	credit: string;
	note: string;
	data: Time[];
	location: string;
	lab: string;
	class_type: string;
	validity: string;
}

interface Time {
	day: string;
	start: string;
	end: string;
}

interface TableTime {
	day: string;
	render_top: number;
	render_height: number;
	subject_name: string;
	subject_id: string;
	class_id: string;
	class_type: string;
	display_date: string;
	room: string;
}

const chosenClasses = ref<Classes[]>([]);
const classesList = ref<Classes[]>([]);
const labs_req = ref([""]);
const isDialogOpen = ref(false);
const isFileParsed = ref(false);
const toAppend = ref<TableTime[]>([]);
const confirmClass = ref(false);

let selectedData: Classes | null = null;
const selected = ref("");
const ltClass = ref("");
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

async function selectClass(data: Classes): Promise<void> {
	if (
		data.included_id !== `String("NULL")` &&
		data.included_id !== data.class_id
	) {
		confirmClass.value = true;
		ltClass.value = data.included_id;
		selectedData = data;
		console.log(confirmClass.value);
	} else {
		confirmClass.value = false;
		ltClass.value = "";
		selectedData = null;
		void confirm(data);
	}
	if (data.lab == "TN") {
		try {
			const response = await invoke("get_chosen_classes");
			chosenClasses.value = response as Classes[];
			for (let i = 0; i < chosenClasses.value.length; i++) {
				if (
					chosenClasses.value[i].subject_id == data.subject_id &&
					chosenClasses.value[i].class_type == "TN"
				) {
					break;
				} else {
					labs_req.value.push(chosenClasses.value[i].subject_id);
				}
			}
		} catch (error) {
			console.log(error);
		}
	}
	if (data.class_type == "TN") {
		labs_req.value = labs_req.value.filter(
			(item) => item !== data.subject_id,
		);
	}
}

async function confirm(data: Classes): Promise<void> {
	try {
		const response = await invoke("add_chosen_class", { data });
		toAppend.value = response as TableTime[];
		for (let i = 0; i < toAppend.value.length; i++) {
			const day_of_week = toAppend.value[i].day as Weekday;
			const event_data: CurEvent = {
				day: day_of_week,
				renderTop: toAppend.value[i].render_top,
				renderHeight: toAppend.value[i].render_height,
				displayDate: toAppend.value[i].display_date.toString(),
				subjectName: toAppend.value[i].subject_name.toString(),
				subjectId: toAppend.value[i].subject_id.toString(),
				class_id: toAppend.value[i].class_id.toString(),
				class_type: toAppend.value[i].class_type.toString(),
				room: toAppend.value[i].room.toString(),
			};
			dailyEvents.value[day_of_week].push(event_data);
		}
		classesList.value = [];
	} catch (error) {
		console.log(error);
	}
}

async function confirmSelection(): Promise<void> {
	confirmClass.value = false;
	if (selectedData !== null) {
		await confirm(selectedData);
	}
}

function cancelSelection(): void {
	confirmClass.value = false;
	ltClass.value = "";
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

function parseTime(time: Time[]): string {
	let result = "";
	for (let i = 0; i < time.length; i++) {
		const element = time[i];
		result += ` Thứ ${element.day}: ${parseHour(element.start)} - ${parseHour(element.end)}\n`;
	}
	return result;
}

function parseHour(hhmm: any): string {
	if (typeof hhmm !== "string") {
		hhmm = hhmm.toString();
	}
	if (hhmm.length < 4) {
		hhmm = "0" + hhmm;
	}
	return `${hhmm.substring(0, 2)}:${hhmm.substring(2, 4)}`;
}

function requireLab(input: string): string {
	if (input === "TN") {
		return "Có";
	} else {
		return "Không";
	}
}

function parseIncludedId(includedId: string): string {
	if (includedId === `String("NULL")`) return "Không có";
	else return includedId;
}
</script>

<template>
	<div>
		<div class="ml-4 mt-4 flex items-center">
			<button
				class="btn-s btn btn-primary"
				@click="select"
				:disabled="isDialogOpen"
			>
				Chọn file Excel
			</button>
			<div v-if="message" class="ml-2">{{ message }}</div>
		</div>
		<div class="flex justify-between">
			<div class="ml-4 mt-4 flex items-center">
				<input
					type="text"
					v-model="textboxValue"
					class="input input-primary w-full max-w-xs"
					placeholder="Mã học phần"
				/>
				<button
					class="btn btn-primary mb-3 ml-2"
					@click="submit"
					:disabled="!isFileParsed"
				>
					Submit
				</button>
			</div>
			<div class="ml-4 mt-4 flex items-center">
				Các học phần cần lớp TN: {{ labs_req }}
			</div>
		</div>
		<div class="section">
			<div
				v-for="(item, index) in classesList"
				:key="index"
				class="card border"
			>
				<div class="card-body">
					<h5 class="card-title">{{ item.class_id }}</h5>
					<p class="card-text">{{ item.class_title }}</p>
					<p class="card-text">Mã học phần: {{ item.subject_id }}</p>
					<p class="card-text">
						Mã lớp đi kèm: {{ parseIncludedId(item.included_id) }}
					</p>
					<p class="card-text">Tín chỉ: {{ item.credit }}</p>
					<p class="card-text">Ghi chú: {{ item.note }}</p>
					<p class="card-text">
						Thời gian:{{ parseTime(item.data) }}
					</p>
					<p class="card-text">Loại lớp: {{ item.class_type }}</p>
					<p class="card-text">Phòng học: {{ item.location }}</p>
					<p class="card-text">
						Yêu cầu thí nghiệm: {{ requireLab(item.lab) }}
					</p>
					<p class="card-text">
						Đăng ký dành cho: {{ item.validity }}
					</p>
					<button class="btn btn-primary" @click="selectClass(item)">
						Choose
					</button>
				</div>
			</div>
		</div>
		<WeekView />
	</div>
	<dialog v-if="confirmClass" class="modal" open>
		<div class="modal-box">
			<h3 class="text-lg font-bold">Xác nhận lớp</h3>
			<p>
				Môn này bắt buộc phải có học phần {{ ltClass }} đi cùng. Hãy ấn
				OK nếu bạn chắc chắn muốn học cả 2 học phần này.
			</p>
			<div class="modal-action">
				<form method="dialog">
					<button
						class="btn btn-primary mr-4"
						@click="confirmSelection"
					>
						OK
					</button>
					<button class="btn btn-primary" @click="cancelSelection">
						Huỷ bỏ
					</button>
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
	box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
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
