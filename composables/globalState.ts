const CELL_HEIGHT = 60;
const HOUR_OFFSET = 6;

interface Event {
	// hhmm (start) -> hh,hh (start) -> (hh (start) - HOUR_OFFSET) * CELL_HEIGHT
	renderTop: number;
	// hhmm (end) -> hh,hh (end) -> (hh,hh (end) - hh,hh (start) - HOUR_OFFSET) * CELL_HEIGHT
	renderHeight: number;

	displayDate: string;
	subjectName: string;
	subjectId: string;
	room: string;
}

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
const dailyEvents: Ref<Record<Weekday, Event[]>> = ref({
	Mon: [
		{
			renderTop: (8.5 - HOUR_OFFSET) * CELL_HEIGHT,
			renderHeight: 1 * CELL_HEIGHT,
			displayDate: "8:30AM - 10:30AM",
			subjectName: "Toán rời rạc",
			subjectId: "INT1001",
			room: "A1.201",
		},
		{
			renderTop: (9.25 - HOUR_OFFSET) * CELL_HEIGHT,
			renderHeight: 1 * CELL_HEIGHT,
			displayDate: "9:00AM - 12:30PM",
			subjectName: "Toán rời rạc",
			subjectId: "INT1001",
			room: "A1.202",
		},
	],
	Tue: [],
	Wed: [],
	Thu: [],
	Fri: [],
	Sat: [],
	Sun: [],
});

export { dailyEvents, CELL_HEIGHT, HOUR_OFFSET, Weekday };
export type { Event };
