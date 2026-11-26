const loginPath = '/login';
const homePath = '/(protected)';
const newPath = '/new';
const idPath = '/[id]';
const editPath = '/edit';
const recipesPath = `${homePath}/recipes`;
const ingredientsPath = `${homePath}/ingredients`;
const shoppingListsPath = `${homePath}/shopping-lists`;

export const routes = {
	login: loginPath,
	home: homePath,
	recipes: recipesPath,
	recipe: `${recipesPath}${idPath}`,
	recipeNew: `${recipesPath}${newPath}`,
	recipeEdit: `${recipesPath}${idPath}${editPath}`,
	ingredients: ingredientsPath,
	ingredient: `${ingredientsPath}${idPath}`,
	shoppingLists: shoppingListsPath,
	shoppingList: `${shoppingListsPath}${idPath}`
} as const;
