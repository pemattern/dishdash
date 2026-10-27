const loginPath = '/login';
const homePath = '/(protected)';
const recipesPath = `${homePath}/recipes`;
const ingredientsPath = `${homePath}/ingredients`;
const shoppingListsPath = `${homePath}/shopping-lists`;

export const routes = {
	login: loginPath,
	home: homePath,
	recipes: recipesPath,
	recipe: (id: string) => `${recipesPath}/${id}`,
	ingredients: ingredientsPath,
	ingredient: (id: string) => `${ingredientsPath}/${id}`,
	shoppingLists: shoppingListsPath,
	shoppingList: (id: string) => `${shoppingListsPath}/${id}`
} as const;
