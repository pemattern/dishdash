import { m } from '#lib/paraglide/messages.js';
import type { MealTypeCode } from '$bindings';

const loginPath = '/login';
const homePath = '/(protected)';
const idPath = '/[id]';
const recipesPath = `${homePath}/recipes`;
const ingredientsPath = `${homePath}/ingredients`;
const shoppingListsPath = `${homePath}/shopping-lists`;

export const routes = {
	login: loginPath,
	home: homePath,
	recipes: recipesPath,
	recipe: `${recipesPath}${idPath}`,
	ingredients: ingredientsPath,
	ingredient: `${ingredientsPath}${idPath}`,
	shoppingLists: shoppingListsPath,
	shoppingList: `${shoppingListsPath}${idPath}`
} as const;

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
