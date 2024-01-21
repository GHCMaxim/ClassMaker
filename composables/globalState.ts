import { ref } from "vue";

const CELL_HEIGHT = 60;
const HOUR_OFFSET = 6;

interface CurEvent {
	day: Weekday;
	// hhmm (start) -> hh,hh (start) -> (hh (start) - HOUR_OFFSET) * CELL_HEIGHT
	renderTop: number;
	// hhmm (end) -> hh,hh (end) -> (hh,hh (end) - hh,hh (start) - HOUR_OFFSET) * CELL_HEIGHT
	renderHeight: number;

	displayDate: string;
	subjectName: string;
	subjectId: string;
	class_type: string;
	class_id: string;
	room: string;
}

const message = ref("");
enum Weekday {
	Mon = "Mon",
	Tue = "Tue",
	Wed = "Wed",
	Thu = "Thu",
	Fri = "Fri",
	Sat = "Sat",
	Sun = "Sun",
}

// Do the calculation in the backend
const dailyEvents: Ref<Record<Weekday, CurEvent[]>> = ref({
	Mon: [],
	Tue: [],
	Wed: [],
	Thu: [],
	Fri: [],
	Sat: [],
	Sun: [],
});

export { dailyEvents, CELL_HEIGHT, HOUR_OFFSET, Weekday };
export type { CurEvent };
export { message };
