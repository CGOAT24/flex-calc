export const WeightUnit = {
	Kg: "Kg",
	Lb: "Lb"
};

export const displayWeightUnit = (unit: string, value: number = 2) => {
	switch (unit.toLowerCase()) {
		case "kg":
			return value >= 2 ? "kgs" : "kg";
		case "lb":
			return value >= 2 ? "lbs" : "lb";
	}
}

export const weightUnitToEnum = (unit: string) => `${unit[0].toUpperCase()}${unit.slice(1)}`;

export interface PlateCount {
		weight: number;
		count: number;
}

export interface CalcRequest {
	total_weight: number;
	bar_weight: number;
	unit: string;
}

export interface CalcResponse {
	unit: string;
	plates: PlateCount[];
}

export interface PlateSetting {
	weight_unit: string;
	weight: number,
	enabled: boolean;
}

export interface Setting {
	weight_unit: string;
	plates: PlateSetting[]

}