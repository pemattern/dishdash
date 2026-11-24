import { m } from '#lib/paraglide/messages.js';
import type { MealTypeCode } from '$bindings';

export const mealTypeLabelMap = new Map<MealTypeCode, string>([
	['breakfast', m.breakfast()],
	['brunch', m.brunch()],
	['lunch', m.lunch()],
	['dinner', m.dinner()],
	['snack', m.snack()],
	['dessert', m.dessert()],
	['appetizer', m.appetizer()],
	['side-dish', m.sideDish()],
	['drink', m.drink()]
]);
