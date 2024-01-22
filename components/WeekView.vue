<script setup lang="ts">
import { Weekday } from "#imports";
import { invoke } from "@tauri-apps/api";

const WEEKDAYS = Object.values(Weekday);

// 6AM -> 9PM
const HOURS = Array.from({ length: 16 }, (_, i) => i + 6).map((hour) => {
	switch (true) {
		case hour < 12:
			return `${hour}AM`;
		case hour === 12:
			return `${hour}PM`;
		default:
			return `${hour - 12}PM`;
	}
});

function enumKeys<O extends object, K extends keyof O = keyof O>(obj: O): K[] {
	return Object.keys(obj).filter((k) => !Number.isNaN(k)) as K[];
}

async function deleteClass(box: CurEvent): Promise<void> {
	const response = await invoke("remove_chosen_class", {
		classId: box.class_id,
	});
	message.value = response as string;
	for (const weekdays in Weekday) {
		const weekday = weekdays as Weekday;
		for (let i = 0; i < dailyEvents.value[weekday].length; i++) {
			if (dailyEvents.value[weekday][i].class_id == box.class_id) {
				dailyEvents.value[weekday].splice(i, 1);
				console.log(dailyEvents.value[weekday]);
			}
		}
	}
}
</script>

<template>
	<div class="size-full">
		<!-- Weekdays in text -->
		<div
			class="sticky top-0 grid w-full grid-cols-7 pl-20 backdrop-blur-sm"
		>
			<div
				v-for="(item, index) in WEEKDAYS"
				:key="index"
				class="flex justify-center py-4 font-bold"
			>
				<div class="w-20 text-center">{{ item }}</div>
			</div>
		</div>

		<!-- Wrapper of the rest of the calendar -->
		<div class="flex flex-row">
			<!-- Hours col, flex w/ fixed width -->
			<div class="flex w-20 flex-col">
				<div
					v-for="(item, index) in HOURS"
					:key="index"
					class="flex flex-col items-center justify-center"
					:style="`height: ${CELL_HEIGHT}px`"
				>
					<div class="w-20 -translate-y-8 text-center">
						{{ item }}
					</div>
				</div>
			</div>

			<!-- Cols container -->
			<div class="grid w-full grid-cols-7">
				<!-- Each col is a day -->
				<div
					v-for="day in WEEKDAYS"
					class="relative flex flex-col"
					:ref="`dayCol${day}`"
				>
					<!-- Each row is an hour -->
					<div
						v-for="_ in HOURS"
						class="border-l border-t border-gray-500"
						:style="{
							borderRightWidth: day === Weekday.Sun ? '1px' : '',
							height: `${CELL_HEIGHT}px`,
						}"
					></div>

					<!-- Each element is an event -->
					<button
						v-for="event in dailyEvents[day]"
						class="event-element absolute w-[calc(100%-0.5rem)] overflow-hidden rounded-md bg-primary px-3 py-2 text-start text-black shadow-[0_-3px_20px_-5px_rgba(0,0,0,0.5)] hover:z-[100] hover:scale-[1.03]"
						:style="{
							top: `${event.renderTop}px`,
							transitionProperty: 'max-height, transform',
							transitionDuration: '0.3s',
							transitionTimingFunction:
								'cubic-bezier(0.4, 0, 0.2, 1)',
						}"
						:class="`event-${day}`"
					>
						<span class="font-bold"
							>{{ event.subjectId }} {{ event.subjectName }}</span
						><br />
						<span>{{ event.displayDate }}</span
						><br />
						<span>{{ event.room }}</span
						><br />
						<span>{{ event.class_type }}</span>
						<br />
						<button
							class=" ml-4 btn-sm btn btn-active btn-error text-white "
							@click="deleteClass(event)"
						>
							Xoá môn học
						</button>
					</button>
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped>
.event-element {
	min-height: 60px;
	max-height: 60px;
}
.event-element:hover {
	max-height: 200px;
}
</style>
