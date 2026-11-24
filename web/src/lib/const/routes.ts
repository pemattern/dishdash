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
