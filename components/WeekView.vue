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

function handleMouseEnterEvent(target: EventTarget | null): void {
	if (target === null) return;
	const element = target as HTMLElement;

	element.style.maxHeight = "100vh";
	element.style.transform = "scale(1.01)";
	element.style.zIndex = "100";
}

function handleMouseLeaveEvent(
	target: EventTarget | null,
	renderHeight: number,
): void {
	if (target === null) return;
	const element = target as HTMLElement;
	element.style.maxHeight = `${renderHeight}px`;
	element.style.transform = "scale(1)";
}

function handleTransitionEnd(target: EventTarget | null) {
	if (target === null) return;
	const element = target as HTMLElement;

	if (element.style.transform !== "scale(1)") return;

	element.style.zIndex = "0";
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
						v-for="anEvent in dailyEvents[day]"
						class="event-element absolute w-[calc(100%-0.5rem)] overflow-hidden rounded-md bg-primary px-3 py-2 text-start text-black shadow-[0_-3px_20px_-5px_rgba(0,0,0,0.5)] hover:z-[100] hover:scale-[1.03]"
						:style="{
							top: `${anEvent.renderTop}px`,
							transitionProperty: 'max-height, transform',
							transitionDuration: '0.3s',
							transitionTimingFunction:
								'cubic-bezier(0.4, 0, 0.2, 1)',
							maxHeight: `${anEvent.renderHeight}px`,
						}"
						@mouseenter="handleMouseEnterEvent($event.target)"
						@mouseleave="
							handleMouseLeaveEvent(
								$event.target,
								anEvent.renderHeight,
							)
						"
						@transitionend="handleTransitionEnd($event.target)"
					>
						>
						<span class="font-bold"
							>{{ anEvent.subjectId }}
							{{ anEvent.subjectName }}</span
						><br />
						<span>{{ anEvent.displayDate }}</span
						><br />
						<span>{{ anEvent.room }}</span
						><br />
						<span>{{ anEvent.class_type }}</span>
						<br />
						<button
							class="btn btn-error btn-active btn-sm ml-4 text-white"
							@click="deleteClass(anEvent)"
						>
							Xoá môn học
						</button>
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
