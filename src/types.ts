export const WeightUnit = {
	Kg: "Kg",
	Lb: "Lb"
};

export const displayWeightUnit = (unit: string, value: number = 2) => {
	switch (unit) {
		case WeightUnit.Kg:
			return value >= 2 ? "kgs" : "kg";
		case WeightUnit.Lb:
			return value >= 2 ? "lbs" : "lb";
	}
}

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